use axum::{extract::State, http::StatusCode, Json, response::IntoResponse, Extension};
use std::sync::Arc;
use blazing_models::{AppError, LoginRequest, RegisterRequest, User};
use crate::{AuthService, CurrentUser};

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

pub async fn me_handler(
    Extension(current_user): Extension<CurrentUser>,
    State(auth_service): State<Arc<AuthService>>,
) -> Result<impl IntoResponse, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            avatar_url,
            created_at,
            updated_at
        FROM users
        WHERE id = $1
        "#,
        current_user.user_id
    )
        .fetch_optional(&auth_service.db_pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user))
}