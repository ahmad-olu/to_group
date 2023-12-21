use axum::{extract::State, http::StatusCode, Extension};

use crate::{
    app_state::AppState,
    helper::{
        app_error::AppError, handler::update_user_details_helper,
        response_model::UserUpdateRequest, schema::UserSchema,
    },
};

pub async fn logout(
    Extension(user): Extension<UserSchema>,
    State(app_state): State<AppState>,
) -> Result<StatusCode, AppError> {
    let update_user = UserUpdateRequest {
        full_name: None,
        email: None,
        token: Some("".to_string()),
        job: None,
        friends: None,
    };

    let id = user.id.unwrap().to_string();

    update_user_details_helper(&id.clone(), app_state.u_collection, update_user).await?;

    Ok(StatusCode::OK)
}
