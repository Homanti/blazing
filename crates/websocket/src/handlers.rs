use crate::{service::MessageHandler, WebSocketService, WsState};
use axum::{
    extract::{ws::WebSocketUpgrade, State, Query},
    response::Response,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct WsQuery {
    pub token: String,
}

pub async fn ws_handler<H>(
    ws: WebSocketUpgrade,
    State(state): State<WsState<H, H::BroadcastKey, H::Message>>,
    Query(query): Query<WsQuery>,
) -> Response
where
    H: MessageHandler,
    H::BroadcastKey: std::hash::Hash + Eq + Clone,
    H::Message: Clone,
{
    let client_id = Uuid::new_v4();

    ws.on_upgrade(move |socket| {
        WebSocketService::handle_socket::<H>(
            socket,
            client_id,
            state.broadcasts,
            state.handler,
            Some(query.token),
        )
    })
}