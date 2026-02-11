use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use blazing_auth::CurrentUser;
use blazing_models::{AppError, SendMessageRequest};
use crate::MessageService;

pub async fn send_message_handler(
    Extension(current_user): Extension<CurrentUser>,
    State(message_service): State<Arc<MessageService>>,
    Json(request): Json<SendMessageRequest>
) -> Result<impl IntoResponse, AppError> {
    let message = message_service
        .create_message(request, current_user.user_id)
        .await
        .map_err(|e| AppError::Internal(format!("Error sending message: {}", e)))?;

    Ok(Json(message))
}