use axum::Json;

use crate::helper::{app_error::AppError, response_model::UserUpdateRequest};

pub async fn update_user(
    Json(request_user): Json<UserUpdateRequest>,
) -> Result<Json<UserUpdateRequest>, AppError> {
    Ok(Json(request_user))
}
