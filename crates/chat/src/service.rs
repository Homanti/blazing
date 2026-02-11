use sqlx::PgPool;
use blazing_models::{AppError, Attachment, GetMessagesRequest, Message, MessageType, SendMessageRequest};
use sqlx::types::{Json, Uuid};

pub struct MessageService {
    db_pool: PgPool,
}

impl MessageService {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create_message(&self, request: SendMessageRequest, author_id: Uuid) -> Result<Message, AppError> {
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

        Ok(message)
    }

    pub async fn get_messages(&self, request: GetMessagesRequest) -> Result<Vec<Message>, AppError> {
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
}