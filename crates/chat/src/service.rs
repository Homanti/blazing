use std::sync::Arc;
use sqlx::PgPool;
use blazing_models::{AppError, Attachment, GetMessagesRequest, Message, MessageType, SendMessageRequest};
use sqlx::types::{Json, Uuid};
use blazing_auth::CurrentUser;
use blazing_ws::Broadcaster;
use crate::WsMessage;

pub struct MessagesService {
    db_pool: PgPool,
    broadcaster: Arc<Broadcaster<Uuid, WsMessage>>
}

impl MessagesService {
    pub fn new(db_pool: PgPool, broadcaster: Arc<Broadcaster<Uuid, WsMessage>>) -> Self {
        Self { db_pool, broadcaster }
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.db_pool
    }

    pub async fn create_message(&self, request: SendMessageRequest, author_id: Uuid) -> Result<Message, AppError> {
        if !self.user_has_channel_access(author_id, request.channel_id).await? {
            return Err(AppError::Forbidden("User is not a member of this guild".to_string()));
        }

        let message_type = request.message_type.unwrap_or(MessageType::Default);

        let message = sqlx::query_as!(Message,
        r#"
            INSERT INTO messages (channel_id, author_id, content, message_type, attachments)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                channel_id,
                author_id,
                content,
                message_type as "message_type: MessageType",
                attachments as "attachments: Json<Vec<Attachment>>",
                created_at,
                updated_at
        "#, request.channel_id, author_id, request.content,
            message_type as MessageType,
            request.attachments.filter(|json| !json.is_empty()) as Option<Json<Vec<Attachment>>>
    )
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| AppError::Database(format!("Database error: {}", e)))?;

        tracing::info!("Author: {}, Content: {}", message.author_id, message.content);

        if let Err(e) = self.broadcaster.broadcast(
            &request.channel_id,
            WsMessage::MessageCreated { message: message.clone() }
        ).await {
            tracing::warn!("Failed to broadcast message: {}", e);
        }

        Ok(message)
    }

    pub async fn get_messages(&self, request: GetMessagesRequest, current_user: CurrentUser) -> Result<Vec<Message>, AppError> {
        if !self.user_has_channel_access(current_user.user_id, request.channel_id).await? {
            return Err(AppError::Forbidden("User is not a member of this guild".to_string()));
        }

        let messages = sqlx::query_as!(Message,
            r#"
                SELECT
                    id,
                    channel_id,
                    author_id,
                    content,
                    message_type as "message_type: MessageType",
                    attachments as "attachments: Json<Vec<Attachment>>",
                    created_at,
                    updated_at
                FROM messages
                WHERE channel_id = $1
                ORDER BY created_at DESC
            "#, request.channel_id
        )
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| AppError::Database(format!("Database error: {}", e)))?;

        Ok(messages)
    }

    pub async fn user_has_channel_access(
        &self,
        user_id: Uuid,
        channel_id: Uuid
    ) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM channels c
                INNER JOIN guild_members gm ON c.guild_id = gm.guild_id
                WHERE c.id = $1 AND gm.user_id = $2
            ) as "exists!"
            "#,
            channel_id,
            user_id
        )
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| AppError::Database(format!("Database error: {}", e)))?;

        Ok(result.exists)
    }
}