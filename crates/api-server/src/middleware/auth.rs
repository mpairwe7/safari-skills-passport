use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use uuid::Uuid;
use std::sync::Arc;

use crate::services::AppState;

pub struct AuthUser(pub Uuid, pub String); // (user_id, role)

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing or invalid authorization header".to_string(),
                )
            })?;

        // Verify the token
        let claims = state
            .auth_service
            .verify_token(bearer.token())
            .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))?;

        // Parse user ID
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid user ID in token".to_string(),
            )
        })?;

        Ok(AuthUser(user_id, claims.role))
    }
}
