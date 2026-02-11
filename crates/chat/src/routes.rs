use std::sync::Arc;
use axum::{middleware, Router};
use axum::routing::post;
use blazing_auth::{auth_middleware, AuthService};
use crate::{send_message_handler, MessageService};

pub fn create_message_routes(
    message_service: Arc<MessageService>,
    auth_service: Arc<AuthService>
) -> Router {
    let auth_layer = middleware::from_fn_with_state(
        auth_service.clone(),
        auth_middleware
    );
    
    Router::new()
        .route("/send", post(send_message_handler).layer(auth_layer))
        .with_state(message_service)
}