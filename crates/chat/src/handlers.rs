use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use blazing_auth::CurrentUser;
use blazing_models::{AppError, GetMessagesRequest};
use crate::MessagesService;

pub async fn get_messages_handler(
    Extension(_current_user): Extension<CurrentUser>,
    State(messages_service): State<Arc<MessagesService>>,
    Json(request): Json<GetMessagesRequest>
) -> Result<impl IntoResponse, AppError> {
    let messages = messages_service
        .get_messages(request)
        .await
        .map_err(|e| AppError::Internal(format!("Error fetching messages: {}", e)))?;
    Ok(Json(messages))
}