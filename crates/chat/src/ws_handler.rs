use async_trait::async_trait;
use blazing_models::{Message, SendMessageRequest};
use blazing_ws::{MessageHandler, ClientId, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use blazing_auth::validate_token;
use crate::MessagesService;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "message")]
    NewMessage(SendMessageRequest),

    #[serde(rename = "message_created")]
    MessageCreated { message: Message },

    #[serde(rename = "typing_start")]
    TypingStart {
        channel_id: Uuid,
        user_id: Uuid
    },

    #[serde(rename = "typing_stop")]
    TypingStop {
        channel_id: Uuid,
        user_id: Uuid
    },
}

#[derive(Clone)]
pub struct ChatMessageHandler {
    messages_service: Arc<MessagesService>,
    jwt_secret: String,
}

impl ChatMessageHandler {
    pub fn new(messages_service: Arc<MessagesService>, jwt_secret: String) -> Self {
        Self {
            messages_service,
            jwt_secret,
        }
    }
}

#[async_trait]
impl MessageHandler for ChatMessageHandler {
    type Message = WsMessage;
    type BroadcastKey = Uuid;

    async fn authenticate(&self, token: &str) -> Result<Uuid> {
        validate_token(token, &self.jwt_secret)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    async fn on_connect(&self, client_id: ClientId, user_id: Uuid) -> Result<()> {
        tracing::info!("User {} connected with client_id: {}", user_id, client_id);
        Ok(())
    }

    async fn on_disconnect(&self, client_id: ClientId) -> Result<()> {
        tracing::info!("Client disconnected: {}", client_id);
        Ok(())
    }

    async fn on_message(
        &self,
        _client_id: ClientId,
        user_id: Uuid,
        message: Self::Message,
    ) -> Result<Option<(Self::BroadcastKey, Self::Message)>> {
        match message {
            WsMessage::NewMessage(request) => {
                let channel_id = request.channel_id;

                let created_message = self.messages_service
                    .create_message(request, user_id)
                    .await
                    .map_err(|e| format!("Failed to create message: {}", e))?;

                Ok(Some((
                    channel_id,
                    WsMessage::MessageCreated { message: created_message }
                )))
            }

            WsMessage::TypingStart { channel_id, user_id } => {
                Ok(Some((channel_id, WsMessage::TypingStart { channel_id, user_id })))
            }

            WsMessage::TypingStop { channel_id, user_id } => {
                Ok(Some((channel_id, WsMessage::TypingStop { channel_id, user_id })))
            }
            _ => Ok(None),
        }
    }

    async fn get_user_broadcast_keys(&self, user_id: Uuid) -> Result<Vec<Self::BroadcastKey>> {
        let channels = sqlx::query!(
            r#"
                SELECT DISTINCT c.id
                FROM channels c
                INNER JOIN guild_members gm ON c.guild_id = gm.guild_id
                WHERE gm.user_id = $1
            "#,
            user_id
        )
            .fetch_all(self.messages_service.get_pool())
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let channel_ids: Vec<Uuid> = channels.into_iter().map(|row| row.id).collect();
        tracing::info!("User {} subscribed to channels: {:?}", user_id, channel_ids);  // Добавь лог
        Ok(channel_ids)
    }

    async fn validate_message(&self, message: &Self::Message) -> Result<()> {
        match message {
            WsMessage::NewMessage(request) => {
                if request.content.trim().is_empty() {
                    return Err("Message content cannot be empty".into());
                }
                if request.content.len() > 2000 {
                    return Err("Message too long".into());
                }
                Ok(())
            }
            _ => Ok(())
        }
    }
}