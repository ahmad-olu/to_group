use axum::{extract::State, Json};

use crate::{
    app_state::AppState,
    helper::{
        app_error::AppError,
        handler::create_user_helper,
        hash::hash_password,
        jwt::create_token,
        response_model::{LoginResponseModel, UserRequest},
        schema::UserSchema,
    },
};

pub async fn create_user(
    State(app_state): State<AppState>,
    Json(request_user): Json<UserRequest>,
) -> Result<Json<LoginResponseModel>, AppError> {
    let generated_token = create_token(&app_state.jwt_secret.as_str(), request_user.email.clone())?;
    let hashed_password = hash_password(request_user.password.as_str())?;

    let user = UserSchema {
        id: None,
        full_name: request_user.full_name,
        email: request_user.email,
        password: hashed_password,
        token: Some(generated_token),
        date_joined: None,
        job: None,
        friends: None,
    };

    let res = create_user_helper(app_state.u_collection, user).await?;

    Ok(Json(res))
}
