use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, Duration};
use common::{AppError, AppResult, UserRole};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User ID
    pub email: String,
    pub role: String,
    pub exp: i64,     // Expiration timestamp
    pub iat: i64,     // Issued at timestamp
}

pub struct AuthService {
    jwt_secret: String,
    jwt_expiration_hours: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, jwt_expiration_hours: i64) -> Self {
        Self {
            jwt_secret,
            jwt_expiration_hours,
        }
    }

    pub fn generate_token(&self, user_id: Uuid, email: String, role: UserRole) -> AppResult<String> {
        let now = Utc::now();
        let expiration = now + Duration::hours(self.jwt_expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            email,
            role: format!("{:?}", role).to_lowercase(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Authentication(format!("Failed to generate token: {}", e)))
    }

    pub fn verify_token(&self, token: &str) -> AppResult<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Authentication(format!("Invalid token: {}", e)))
    }

    pub fn hash_password(&self, password: &str) -> AppResult<String> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        bcrypt::verify(password, hash)
            .map_err(|e| AppError::Authentication(format!("Failed to verify password: {}", e)))
    }
}
