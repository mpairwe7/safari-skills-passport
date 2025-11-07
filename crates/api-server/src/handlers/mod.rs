pub mod auth;
pub mod institutions;
pub mod credentials;

use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "status": "healthy",
        "service": "Safari Skills Passport API",
        "version": "0.1.0"
    })))
}
