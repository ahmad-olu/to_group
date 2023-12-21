use axum::{extract::Path, http::StatusCode, Json};

use crate::helper::{app_error::AppError, response_model::TodoResponse};

//-> Result<Json<TodoResponse>, AppError>
pub async fn get_one_todo(Path(task_id): Path<i32>) -> Result<String, AppError> {
    Ok(format!("Hello world,, {}", task_id))
}
