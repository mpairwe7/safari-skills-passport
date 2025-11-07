use thiserror::Error;
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Blockchain error: {0}")]
    Blockchain(String),

    #[error("IPFS error: {0}")]
    Ipfs(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Credential already exists")]
    CredentialExists,

    #[error("User already exists")]
    UserExists,

    #[error("Invalid credential")]
    InvalidCredential,

    #[error("Institution not accredited")]
    InstitutionNotAccredited,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Authorization(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Blockchain(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Ipfs(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::CredentialExists => (StatusCode::CONFLICT, "Credential already exists".to_string()),
            AppError::UserExists => (StatusCode::CONFLICT, "User already exists".to_string()),
            AppError::InvalidCredential => (StatusCode::BAD_REQUEST, "Invalid credential".to_string()),
            AppError::InstitutionNotAccredited => (StatusCode::FORBIDDEN, "Institution not accredited".to_string()),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
