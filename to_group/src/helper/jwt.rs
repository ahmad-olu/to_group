use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    exp: usize,
    email: String,
}

pub fn create_token(secret: &str, email: String) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let exp = (now + Duration::hours(1)).timestamp() as usize;
    let claims = Claims { exp, email };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&token_header, &claims, &key).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error please try again",
        )
    })
}

pub fn validate_token(secret: &str, token: &str) -> Result<bool, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<Claims>(&token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature
            | jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "not authenticated")
            }

            _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "error validating token"),
        })
        .map(|_claims| true)
}
