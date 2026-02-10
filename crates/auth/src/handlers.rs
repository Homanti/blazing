use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use std::sync::Arc;
use blazing_models::{AppError, LoginRequest, RegisterRequest};
use crate::AuthService;

pub async fn register_handler(State(auth_service): State<Arc<AuthService>>,
                              Json(request): Json<RegisterRequest>) -> Result<impl IntoResponse, AppError> {
    let response = auth_service.register(request).await?;

    Ok((StatusCode::CREATED, Json(response)))
}
pub async fn login_handler(State(auth_service): State<Arc<AuthService>>,
                           Json(request): Json<LoginRequest>) -> Result<impl IntoResponse, AppError> {
    let response = auth_service.login(request).await?;

    Ok((StatusCode::OK, Json(response)))
}