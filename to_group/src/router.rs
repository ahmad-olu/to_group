use axum::{
    middleware,
    routing::{get, patch, post},
    Router,
};

use crate::{
    app_state::AppState,
    middleware::require_auth::require_auth,
    routes::{
        create_todo::create_todo, create_user::create_user, get_all_todo::get_all_todo,
        get_one_todo::get_one_todo, login::login, logout::logout, update_todo::update_todo,
        update_user::update_user,
    },
};

pub async fn create_route(app_state: AppState) -> Router {
    Router::new()
        .route("/logout", post(logout))
        .route("/users", patch(update_user))
        .route("/todo", get(get_all_todo).post(create_todo))
        .route("/todo/:todo_id", get(get_one_todo).patch(update_todo))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_auth,
        ))
        .route("/users", post(create_user))
        .route("/login", post(login))
        .with_state(app_state)
}

// eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MDMxMzk3NTksImVtYWlsIjoiYWNlQDEyMzQ1In0.HONQh2fClhtk9MccyesY3sTJUx6W7_GhfLEQ1RdnZVY
