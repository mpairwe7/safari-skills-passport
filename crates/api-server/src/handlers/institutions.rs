use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use common::{RegisterInstitutionRequest, Institution, AppError};
use database::{InstitutionDb, InstitutionRepository};
use crate::{services::AppState, middleware::auth::AuthUser};

pub async fn register_institution(
    State(state): State<Arc<AppState>>,
    AuthUser(user_id, role): AuthUser,
    Json(payload): Json<RegisterInstitutionRequest>,
) -> Result<Json<Institution>, AppError> {
    // Verify user is an institution
    if role != "institution" {
        return Err(AppError::Authorization("Only institutions can register".to_string()));
    }

    // Check if institution already registered for this user
    if let Some(_) = state.institution_repo.get_institution_by_user_id(user_id).await? {
        return Err(AppError::Validation("Institution already registered".to_string()));
    }

    // Create institution
    let institution_db = InstitutionDb {
        id: Uuid::new_v4(),
        user_id,
        institution_name: payload.institution_name.clone(),
        institution_type: payload.institution_type.clone(),
        country: payload.country.clone(),
        accreditation_number: payload.accreditation_number.clone(),
        is_accredited: false, // Pending accreditation
        created_at: Utc::now(),
    };

    let created = state.institution_repo.create_institution(&institution_db).await?;

    let institution = Institution {
        id: created.id,
        user_id: created.user_id,
        institution_name: created.institution_name,
        institution_type: created.institution_type,
        country: created.country,
        accreditation_number: created.accreditation_number,
        is_accredited: created.is_accredited,
        created_at: created.created_at,
    };

    Ok(Json(institution))
}

pub async fn get_my_institution(
    State(state): State<Arc<AppState>>,
    AuthUser(user_id, _): AuthUser,
) -> Result<Json<Institution>, AppError> {
    let institution_db = state.institution_repo
        .get_institution_by_user_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Institution not found".to_string()))?;

    let institution = Institution {
        id: institution_db.id,
        user_id: institution_db.user_id,
        institution_name: institution_db.institution_name,
        institution_type: institution_db.institution_type,
        country: institution_db.country,
        accreditation_number: institution_db.accreditation_number,
        is_accredited: institution_db.is_accredited,
        created_at: institution_db.created_at,
    };

    Ok(Json(institution))
}
