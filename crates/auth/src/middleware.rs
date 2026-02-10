use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;
use uuid::Uuid;
use blazing_models::AppError;
use crate::{service::AuthService, Claims};

#[derive(Clone)]
pub struct CurrentUser {
    pub user_id: Uuid,
}

pub async fn auth_middleware(
    State(auth_service): State<Arc<AuthService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized("Invalid authorization format".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(auth_service.jwt_secret.as_bytes()),
        &Validation::default(),
    )
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    let user_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| AppError::Internal("Invalid user ID in token".to_string()))?;

    request.extensions_mut().insert(CurrentUser { user_id });

    Ok(next.run(request).await)
}