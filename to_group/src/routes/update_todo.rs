use axum::{http::StatusCode, Json};

use crate::helper::{
    app_error::AppError,
    response_model::{TodoResponse, TodoUpdateRequest},
};

pub async fn update_todo(
    Json(request_todo): Json<TodoUpdateRequest>,
) -> Result<(StatusCode, Json<TodoUpdateRequest>), AppError> {
    Ok((StatusCode::CREATED, Json(request_todo)))
}
