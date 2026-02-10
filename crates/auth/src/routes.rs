use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use crate::AuthService;
use crate::handlers::{login_handler, register_handler};

pub fn create_auth_routes(auth_service: Arc<AuthService>) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(auth_service)
}