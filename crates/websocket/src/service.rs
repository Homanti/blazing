use crate::{ClientId, Result, Broadcaster};
use axum::extract::ws::{Message, WebSocket};
use serde::{Serialize, de::DeserializeOwned};
use async_trait::async_trait;
use futures::{StreamExt, SinkExt};
use std::sync::Arc;

#[async_trait]
pub trait MessageHandler: Send + Sync + 'static {
    type Message: Serialize + DeserializeOwned + Clone + Send + Sync + 'static;
    type BroadcastKey: std::hash::Hash + Eq + Clone + Send + Sync + 'static;

    async fn authenticate(&self, token: &str) -> Result<uuid::Uuid>;
    async fn on_connect(&self, client_id: ClientId, user_id: uuid::Uuid) -> Result<()>;
    async fn on_disconnect(&self, client_id: ClientId) -> Result<()>;
    async fn on_message(
        &self,
        client_id: ClientId,
        user_id: uuid::Uuid,
        message: Self::Message,
    ) -> Result<Option<(Self::BroadcastKey, Self::Message)>>;
    async fn get_user_broadcast_keys(&self, user_id: uuid::Uuid) -> Result<Vec<Self::BroadcastKey>>;
    async fn validate_message(&self, _message: &Self::Message) -> Result<()> {
        Ok(())
    }
}

pub struct WebSocketService;

impl WebSocketService {
    async fn handle_incoming_message<H: MessageHandler>(
        client_id: ClientId,
        user_id: uuid::Uuid,
        text: &str,
        broadcasts: &Arc<Broadcaster<H::BroadcastKey, H::Message>>,
        handler: &Arc<H>,
    ) {
        let parsed_msg = match serde_json::from_str::<H::Message>(text) {
            Ok(msg) => msg,
            Err(e) => {
                tracing::warn!(
                    "Failed to parse WebSocket message from client {}: {} - raw: {}",
                    client_id, e, text
                );
                return;
            }
        };

        if handler.validate_message(&parsed_msg).await.is_err() {
            return;
        }

        let (broadcast_key, message_to_send) = match handler.on_message(client_id, user_id, parsed_msg).await {
            Ok(Some((key, msg))) => (key, msg),
            Ok(None) => return,
            Err(e) => {
                tracing::error!("Error handling message: {}", e);
                return;
            }
        };

        if let Err(e) = broadcasts.broadcast(&broadcast_key, message_to_send).await {
            tracing::error!("Broadcast failed: {}", e);
        }
    }

    pub async fn handle_socket<H: MessageHandler>(
        socket: WebSocket,
        client_id: ClientId,
        broadcasts: Arc<Broadcaster<H::BroadcastKey, H::Message>>,
        handler: Arc<H>,
        token: Option<String>,
    ) {
        let user_id = match token {
            Some(t) => match handler.authenticate(&t).await {
                Ok(uid) => uid,
                Err(e) => {
                    tracing::error!("Authentication failed for client {}: {}", client_id, e);
                    return;
                }
            },
            None => {
                tracing::error!("No token provided for client {}", client_id);
                return;
            }
        };

        let (mut sender, mut receiver) = socket.split();

        if let Err(e) = handler.on_connect(client_id, user_id).await {
            tracing::error!("Connection handler error for client {}: {}", client_id, e);
            return;
        }

        let user_channels = match handler.get_user_broadcast_keys(user_id).await {
            Ok(keys) => keys,
            Err(e) => {
                tracing::error!("Failed to get broadcast keys for user {}: {}", user_id, e);
                return;
            }
        };

        let mut receivers = Vec::new();
        for key in user_channels {
            receivers.push(broadcasts.subscribe(&key).await);
        }

        let mut send_task = tokio::spawn(async move {
            loop {
                let mut received = false;
                for rx in &mut receivers {
                    if let Ok(msg) = rx.try_recv() {
                        if let Ok(json) = serde_json::to_string(&msg) {
                            if sender.send(Message::Text(json.into())).await.is_err() {
                                return;
                            }
                            received = true;
                        }
                    }
                }
                if !received {
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
            }
        });

        let broadcasts_clone = broadcasts.clone();
        let handler_clone = handler.clone();
        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                if let Message::Text(text) = msg {
                    Self::handle_incoming_message::<H>(
                        client_id,
                        user_id,
                        &text,
                        &broadcasts_clone,
                        &handler_clone,
                    ).await;
                }
            }
        });

        tokio::select! {
            _ = &mut send_task => recv_task.abort(),
            _ = &mut recv_task => send_task.abort(),
        }

        tracing::info!("Client disconnected: {}", client_id);
        let _ = handler.on_disconnect(client_id).await;
    }
}