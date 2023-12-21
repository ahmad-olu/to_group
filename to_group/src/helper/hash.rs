use axum::http::StatusCode;
use bcrypt::{hash, verify};

use super::app_error::AppError;

const COST: u32 = 12;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, COST).map_err(|_error| {
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "error securing password")
    })
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|_error| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error verifying your password",
        )
    })
}
