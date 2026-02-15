use axum::{routing::post, Router, middleware};
use std::sync::Arc;
use blazing_ws::{ws_routes, Broadcaster};
use blazing_auth::{AuthService, auth_middleware};
use crate::{handlers, MessagesService, ChatWsState, ChatMessageHandler, WsMessage};
use uuid::Uuid;

pub fn create_chat_routes(
    messages_service: Arc<MessagesService>,
    auth_service: Arc<AuthService>,
    jwt_secret: String,
    broadcaster: Arc<Broadcaster<Uuid, WsMessage>>,
) -> Router {
    let rest_routes = Router::new()
        .route("/messages/history", post(handlers::get_messages_handler))
        .layer(middleware::from_fn_with_state(
            auth_service.clone(),
            auth_middleware,
        ))
        .with_state(messages_service.clone());

    let ws_handler = ChatMessageHandler::new(messages_service, jwt_secret);
    let ws_state = ChatWsState::new(ws_handler, (*broadcaster).clone());
    let websocket_routes = ws_routes::<ChatMessageHandler>()
        .with_state(ws_state);

    rest_routes.merge(websocket_routes)
}