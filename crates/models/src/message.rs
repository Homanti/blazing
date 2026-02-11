use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::Type;
use sqlx::types::Json;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Default,
    Reply,
    UserJoin,
    UserLeave
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Attachment {
    pub id: Uuid,
    pub filename: String,
    pub url: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub message_type: Option<MessageType>,
    pub attachments: Option<Json<Vec<Attachment>>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub channel_id: Uuid,
    pub content: String,
    pub message_type: Option<MessageType>,
    pub attachments: Option<Json<Vec<Attachment>>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetMessagesRequest {
    pub channel_id: Uuid,
}