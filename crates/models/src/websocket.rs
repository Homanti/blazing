use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{SendMessageRequest, SendMessageResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "new_message")]
    NewMessage(SendMessageRequest),

    #[serde(rename = "message_created")]
    MessageCreated { message: SendMessageResponse },

    #[serde(rename = "typing_start")]
    TypingStart {
        channel_id: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        user_id: Option<Uuid>,
    },

    #[serde(rename = "typing_stop")]
    TypingStop {
        channel_id: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        user_id: Option<Uuid>,
    },
}