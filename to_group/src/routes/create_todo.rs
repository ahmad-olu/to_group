use axum::{extract::State, http::StatusCode, Extension, Json};

use crate::{
    app_state::AppState,
    helper::{
        app_error::AppError,
        handler::create_todo_helper,
        response_model::{TodoRequest, TodoResponse},
        schema::{TodoSchema, UserSchema},
    },
};

pub async fn create_todo(
    Extension(user): Extension<UserSchema>,
    State(app_state): State<AppState>,
    Json(request_todo): Json<TodoRequest>,
) -> Result<(StatusCode, Json<TodoResponse>), AppError> {
    let new_todo = TodoSchema {
        id: None,
        uid: user.id,
        title: request_todo.title,
        description: request_todo.description,
        time_from: request_todo.time_from,
        time_to: request_todo.time_to,
        date: request_todo.date,
        is_completed: request_todo.is_completed,
        sub_todo: request_todo.sub_todo,
    };
    let res = create_todo_helper(app_state.t_collection, new_todo).await?;

    Ok((StatusCode::CREATED, Json(res)))
}
