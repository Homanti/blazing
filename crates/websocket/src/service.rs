use crate::{ClientId, Result, Broadcaster};
use axum::extract::ws::{Message, WebSocket};
use serde::{Serialize, de::DeserializeOwned};
use async_trait::async_trait;
use futures::{StreamExt, SinkExt, stream::{SelectAll, select_all}};
use std::sync::Arc;
use tokio_stream::wrappers::BroadcastStream;

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

        if let Err(e) = broadcasts.broadcast(&broadcast_key, message_to_send) {
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

        let mut broadcast_receivers = Vec::new();
        for key in user_channels {
            broadcast_receivers.push(broadcasts.subscribe(&key));
        }

        let mut combined_stream: SelectAll<BroadcastStream<H::Message>> =
            select_all(broadcast_receivers.into_iter().map(BroadcastStream::new));

        let mut send_task = tokio::spawn(async move {
            while let Some(result) = combined_stream.next().await {
                match result {
                    Ok(msg) => {
                        if let Ok(json) = serde_json::to_string(&msg) {
                            if sender.send(Message::Text(json.into())).await.is_err() {
                                tracing::debug!("Client {} WebSocket closed", client_id);
                                return;
                            }
                        }
                    }
                    Err(tokio_stream::wrappers::errors::BroadcastStreamRecvError::Lagged(n)) => {
                        tracing::warn!(
                        "Client {} lagged by {} messages, some messages were dropped",
                        client_id, n
                    );
                    }
                }
            }
        });

        let broadcasts_clone = broadcasts.clone();
        let handler_clone = handler.clone();
        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                match msg {
                    Message::Text(text) => {
                        Self::handle_incoming_message::<H>(
                            client_id,
                            user_id,
                            &text,
                            &broadcasts_clone,
                            &handler_clone,
                        ).await;
                    }
                    Message::Close(_) => {
                        tracing::debug!("Client {} sent close frame", client_id);
                        break;
                    }
                    Message::Ping(_) => {
                        tracing::trace!("Received ping from client {}", client_id);
                    }
                    Message::Pong(_) => {
                        tracing::trace!("Received pong from client {}", client_id);
                    }
                    Message::Binary(_) => {
                        tracing::warn!("Client {} sent unexpected binary message", client_id);
                    }
                }
            }
        });

        tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
            tracing::debug!("Send task completed for client {}", client_id);
        },
        _ = &mut recv_task => {
            send_task.abort();
            tracing::debug!("Receive task completed for client {}", client_id);
        },
    }

        tracing::info!("Client disconnected: {}", client_id);
        let _ = handler.on_disconnect(client_id).await;
    }
}