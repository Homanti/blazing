use crate::{handlers, service::MessageHandler, WsState};
use axum::{routing::get, Router};

pub fn ws_routes<H>() -> Router<WsState<H, H::BroadcastKey, H::Message>>
where
    H: MessageHandler + Clone,
    H::BroadcastKey: std::hash::Hash + Eq + Clone,
    H::Message: Clone,
{
    Router::new()
        .route("/ws", get(handlers::ws_handler::<H>))
}