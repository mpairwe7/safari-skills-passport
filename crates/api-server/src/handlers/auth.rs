use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use common::{RegisterUserRequest, LoginRequest, AuthResponse, User, UserRole, AppError};
use database::{UserDb, UserRepository};
use crate::services::AppState;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate input
    if payload.email.is_empty() || payload.password.is_empty() || payload.name.is_empty() {
        return Err(AppError::Validation("All fields are required".to_string()));
    }

    // Check if user already exists
    if let Some(_) = state.user_repo.get_user_by_email(&payload.email).await? {
        return Err(AppError::UserExists);
    }

    // Generate wallet address
    let wallet_address = crate::services::blockchain::BlockchainService::generate_wallet_address()?;

    // Hash password
    let password_hash = state.auth_service.hash_password(&payload.password)?;

    // Create user
    let user_id = Uuid::new_v4();
    let now = Utc::now();
    
    let user_db = UserDb {
        id: user_id,
        wallet_address: wallet_address.clone(),
        email: payload.email.clone(),
        password_hash,
        name: payload.name.clone(),
        role: format!("{:?}", payload.role).to_lowercase(),
        is_verified: false,
        created_at: now,
        updated_at: now,
    };

    let created_user = state.user_repo.create_user(&user_db).await?;

    // Generate JWT token
    let token = state.auth_service.generate_token(
        created_user.id,
        created_user.email.clone(),
        payload.role.clone(),
    )?;

    // Convert to User model
    let user = User {
        id: created_user.id,
        wallet_address: created_user.wallet_address,
        email: created_user.email,
        name: created_user.name,
        role: payload.role,
        is_verified: created_user.is_verified,
        created_at: created_user.created_at,
        updated_at: created_user.updated_at,
    };

    Ok(Json(AuthResponse { token, user }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Find user
    let user_db = state.user_repo
        .get_user_by_email(&payload.email)
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

    // Verify password
    let valid = state.auth_service.verify_password(&payload.password, &user_db.password_hash)?;
    if !valid {
        return Err(AppError::Authentication("Invalid credentials".to_string()));
    }

    // Parse role
    let role: UserRole = match user_db.role.as_str() {
        "professional" => UserRole::Professional,
        "institution" => UserRole::Institution,
        "employer" => UserRole::Employer,
        _ => return Err(AppError::Internal("Invalid role".to_string())),
    };

    // Generate JWT token
    let token = state.auth_service.generate_token(
        user_db.id,
        user_db.email.clone(),
        role.clone(),
    )?;

    // Convert to User model
    let user = User {
        id: user_db.id,
        wallet_address: user_db.wallet_address,
        email: user_db.email,
        name: user_db.name,
        role,
        is_verified: user_db.is_verified,
        created_at: user_db.created_at,
        updated_at: user_db.updated_at,
    };

    Ok(Json(AuthResponse { token, user }))
}
