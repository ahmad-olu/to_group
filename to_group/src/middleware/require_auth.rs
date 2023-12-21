use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{
    app_state::AppState,
    helper::{app_error::AppError, handler::get_user_by_token_helper, jwt::validate_token},
};

pub async fn require_auth(
    headers: HeaderMap,
    State(app_state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let header_token = if let Some(token) = headers.get("x-auth-token") {
        token
            .to_str()
            .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token"))?
    } else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "not authenticated"));
    };

    validate_token(&app_state.jwt_secret, header_token)?;

    let user = get_user_by_token_helper(app_state.u_collection, &header_token.to_string()).await;

    if let Ok(user) = user {
        request.extensions_mut().insert(user);
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized",
        ));
    }

    Ok(next.run(request).await)
}
