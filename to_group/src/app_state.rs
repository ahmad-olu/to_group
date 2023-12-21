use axum::extract::FromRef;
use mongodb::Collection;

use crate::helper::schema::{TodoSchema, UserSchema};

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub u_collection: Collection<UserSchema>,
    pub t_collection: Collection<TodoSchema>,
}
