mod handlers;
mod routes;
mod service;
mod broadcaster;

pub use handlers::ws_handler;
pub use routes::ws_routes;
pub use service::{WebSocketService, MessageHandler};
pub use broadcaster::Broadcaster;

use std::sync::Arc;
use uuid::Uuid;

pub type ClientId = Uuid;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub struct WsState<H, K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub handler: Arc<H>,
    pub broadcasts: Arc<Broadcaster<K, V>>,
}

impl<H, K, V> WsState<H, K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(handler: H, broadcasts: Broadcaster<K, V>) -> Self {
        Self {
            handler: Arc::new(handler),
            broadcasts: Arc::new(broadcasts),
        }
    }
}