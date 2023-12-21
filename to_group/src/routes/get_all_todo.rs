use axum::{extract::State, Extension, Json};

use crate::{
    app_state::AppState,
    helper::{
        app_error::AppError, handler::get_user_todos_helper, response_model::TodoListResponse,
        schema::UserSchema,
    },
};

pub async fn get_all_todo(
    State(app_state): State<AppState>,
    Extension(user): Extension<UserSchema>,
) -> Result<Json<TodoListResponse>, AppError> {
    let todo = get_user_todos_helper(app_state.t_collection, &user.id.unwrap().to_string()).await?;

    Ok(Json(TodoListResponse {
        length: todo.length,
        result: todo.result,
    }))
}
