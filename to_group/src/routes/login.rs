use axum::{extract::State, http::StatusCode, Json};
use mongodb::bson::oid::ObjectId;

use crate::{
    app_state::AppState,
    helper::{
        app_error::AppError,
        handler::{get_user_by_email_helper, update_user_details_helper},
        hash::verify_password,
        jwt::create_token,
        response_model::{LoginRequest, LoginResponseModel, UserUpdateRequest},
    },
};

pub async fn login(
    State(app_state): State<AppState>,
    Json(request_user): Json<LoginRequest>,
) -> Result<Json<LoginResponseModel>, AppError> {
    let user =
        get_user_by_email_helper(app_state.u_collection.clone(), &request_user.email).await?;

    if !verify_password(&request_user.password, &user.password)? {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "incorrect username and/or password",
        ));
    };

    let generated_token = create_token(&app_state.jwt_secret.as_str(), request_user.email.clone())?;

    let update_user = UserUpdateRequest {
        full_name: Some(user.full_name),
        email: Some(user.email),
        token: Some(generated_token),
        job: None,
        friends: None,
    };

    let id = user.id.unwrap().to_string();

    let new_user =
        update_user_details_helper(&id.clone(), app_state.u_collection, update_user).await?;

    Ok(Json(LoginResponseModel {
        full_name: new_user.result.full_name,
        id,
        email: new_user.result.email,
        token: new_user.result.token,
    }))
}
