use std::sync::Arc;
use axum::{middleware, Router};
use axum::routing::{get, post};
use crate::{auth_middleware, me_handler, AuthService};
use crate::handlers::{login_handler, register_handler};

pub fn create_auth_routes(auth_service: Arc<AuthService>) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/me", get(me_handler)
            .layer(middleware::from_fn_with_state(
                auth_service.clone(),
                auth_middleware
            )))
        .with_state(auth_service)
}