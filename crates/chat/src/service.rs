use sqlx::PgPool;
use blazing_models::{AppError, Attachment, Author, GetMessagesRequest, Message, MessageType, SendMessageRequest, MessageWithAuthor};
use sqlx::types::{Json, Uuid};
use blazing_auth::CurrentUser;

pub struct MessagesService {
    db_pool: PgPool,
}

impl MessagesService {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.db_pool
    }

    pub async fn create_message(&self, request: SendMessageRequest, author_id: Uuid) -> Result<MessageWithAuthor, AppError> {
        if !self.user_has_channel_access(author_id, request.channel_id).await? {
            return Err(AppError::Forbidden("User is not a member of this guild".to_string()));
        }

        let message_type = request.message_type.unwrap_or(MessageType::Default);

        let row = sqlx::query!(
            r#"
                WITH inserted AS (
                    INSERT INTO messages (channel_id, author_id, content, message_type, attachments)
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING
                        id,
                        channel_id,
                        author_id,
                        content,
                        message_type,
                        attachments,
                        created_at,
                        updated_at
                )
                SELECT
                    inserted.id           as "m_id!",
                    inserted.channel_id   as "m_channel_id!",
                    inserted.author_id    as "m_author_id!",
                    inserted.content      as "m_content!",
                    inserted.message_type as "m_message_type?: MessageType",
                    inserted.attachments  as "m_attachments?: Json<Vec<Attachment>>",
                    inserted.created_at   as "m_created_at!",
                    inserted.updated_at   as "m_updated_at!",

                    u.id          as "u_id!",
                    u.username    as "u_username!",
                    u.avatar_url  as "u_avatar_url?",
                    u.created_at  as "u_created_at!",
                    u.updated_at  as "u_updated_at!"
                FROM inserted
                JOIN users u ON u.id = inserted.author_id
            "#,
            request.channel_id,
            author_id,
            request.content,
            message_type as MessageType,
            request.attachments.filter(|json| !json.is_empty()) as Option<Json<Vec<Attachment>>>
        )
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| AppError::Database(format!("db error: {}", e)))?;

        let message = Message {
            id: row.m_id,
            channel_id: row.m_channel_id,
            author_id: row.m_author_id,
            content: row.m_content,
            message_type: row.m_message_type,
            attachments: row.m_attachments,
            created_at: row.m_created_at,
            updated_at: row.m_updated_at,
        };

        let author = Author {
            id: row.u_id,
            username: row.u_username,
            avatar_url: row.u_avatar_url,
            created_at: row.u_created_at,
            updated_at: row.u_updated_at,
        };

        Ok(MessageWithAuthor { message, author })
    }

    pub async fn get_messages(&self, request: GetMessagesRequest, current_user: CurrentUser) -> Result<Vec<MessageWithAuthor>, AppError> {
        if !self.user_has_channel_access(current_user.user_id, request.channel_id).await? {
            return Err(AppError::Forbidden("User is not a member of this guild".to_string()));
        }

        let rows = sqlx::query!(
            r#"
                SELECT
                    m.id           as "m_id!",
                    m.channel_id   as "m_channel_id!",
                    m.author_id    as "m_author_id!",
                    m.content      as "m_content!",
                    m.message_type as "m_message_type?: MessageType",
                    m.attachments  as "m_attachments?: Json<Vec<Attachment>>",
                    m.created_at   as "m_created_at!",
                    m.updated_at   as "m_updated_at!",

                    u.id         as "u_id!",
                    u.username   as "u_username!",
                    u.avatar_url as "u_avatar_url?",
                    u.created_at as "u_created_at!",
                    u.updated_at as "u_updated_at!"
                FROM messages m
                JOIN users u ON u.id = m.author_id
                WHERE m.channel_id = $1
                ORDER BY m.created_at DESC
            "#,
        request.channel_id
    )
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| AppError::Database(format!("Database error: {}", e)))?;

        let result = rows
            .into_iter()
            .map(|row| {
                let message = Message {
                    id: row.m_id,
                    channel_id: row.m_channel_id,
                    author_id: row.m_author_id,
                    content: row.m_content,
                    message_type: row.m_message_type,
                    attachments: row.m_attachments,
                    created_at: row.m_created_at,
                    updated_at: row.m_updated_at,
                };

                let author = Author {
                    id: row.u_id,
                    username: row.u_username,
                    avatar_url: row.u_avatar_url,
                    created_at: row.u_created_at,
                    updated_at: row.u_updated_at,
                };

                MessageWithAuthor { message, author }
            })
            .collect();

        Ok(result)
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
                WHERE c.id = $1
                AND (
                    (c.guild_id IS NOT NULL AND EXISTS (
                        SELECT 1 FROM guild_members gm
                        WHERE gm.guild_id = c.guild_id AND gm.user_id = $2
                    ))
                    OR
                    (c.guild_id IS NULL AND EXISTS (
                        SELECT 1 FROM channel_recipients cr
                        WHERE cr.channel_id = c.id AND cr.user_id = $2
                    ))
                )
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