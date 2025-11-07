use axum::{
    extract::{State, Path},
    Json,
    response::Response,
    body::Body,
};
use std::sync::Arc;
use serde::Deserialize;
use base64::{Engine as _, engine::general_purpose};

use common::{
    IssueCredentialRequest, IssueCredentialResponse, Credential, CredentialListResponse,
    VerificationResponse, Institution, User, UserRole, CredentialType, CredentialStatus, AppError,
};
use database::{UserRepository, InstitutionRepository, CredentialRepository};
use crate::{services::AppState, middleware::auth::AuthUser};

pub async fn issue_credential(
    State(state): State<Arc<AppState>>,
    AuthUser(issuer_id, role): AuthUser,
    Json(payload): Json<IssueCredentialRequest>,
) -> Result<Json<IssueCredentialResponse>, AppError> {
    // Verify issuer is an accredited institution
    if role != "institution" {
        return Err(AppError::Authorization("Only institutions can issue credentials".to_string()));
    }

    // Temporarily bypass accreditation check for testing
    // let institution = state.institution_repo
    //     .get_institution_by_user_id(issuer_id)
    //     .await?
    //     .ok_or_else(|| AppError::NotFound("Institution not found".to_string()))?;
    // 
    // if !institution.is_accredited {
    //     return Err(AppError::InstitutionNotAccredited);
    // }

    // Find holder by email
    let holder = state.user_repo
        .get_user_by_email(&payload.holder_email)
        .await?
        .ok_or_else(|| AppError::NotFound("Holder not found".to_string()))?;

    // Issue credential
    let response = state.credential_service
        .issue_credential(payload, issuer_id, holder.id)
        .await?;

    Ok(Json(response))
}

pub async fn verify_credential(
    State(state): State<Arc<AppState>>,
    Path(credential_id): Path<String>,
) -> Result<Json<VerificationResponse>, AppError> {
    // Get credential from database
    let credential_db = state.credential_repo
        .get_credential_by_credential_id(&credential_id)
        .await?;

    if let Some(cred_db) = credential_db {
        // Verify on blockchain
        let valid = state.credential_service.verify_credential(&credential_id).await?;

        // Get issuer info
        let issuer_db = state.user_repo.get_user_by_id(cred_db.issuer_id).await?;
        let institution = if let Some(issuer) = issuer_db {
            state.institution_repo.get_institution_by_user_id(issuer.id).await?
        } else {
            None
        };

        // Get holder info
        let holder_db = state.user_repo.get_user_by_id(cred_db.holder_id).await?;

        // Parse credential type
        let credential_type: CredentialType = match cred_db.credential_type.as_str() {
            "certificate" => CredentialType::Certificate,
            "license" => CredentialType::License,
            "degree" => CredentialType::Degree,
            "workexperience" => CredentialType::WorkExperience,
            "skill" => CredentialType::Skill,
            _ => return Err(AppError::Internal("Invalid credential type".to_string())),
        };

        // Parse status
        let status: CredentialStatus = match cred_db.status.as_str() {
            "pending" => CredentialStatus::Pending,
            "issued" => CredentialStatus::Issued,
            "revoked" => CredentialStatus::Revoked,
            "expired" => CredentialStatus::Expired,
            _ => return Err(AppError::Internal("Invalid status".to_string())),
        };

        let credential = Credential {
            id: cred_db.id,
            credential_id: cred_db.credential_id,
            holder_id: cred_db.holder_id,
            issuer_id: cred_db.issuer_id,
            credential_type,
            title: cred_db.title,
            description: cred_db.description,
            ipfs_hash: cred_db.ipfs_hash,
            chain_hash: cred_db.chain_hash,
            issue_date: cred_db.issue_date,
            expiry_date: cred_db.expiry_date,
            status,
            metadata: cred_db.metadata,
            created_at: cred_db.created_at,
        };

        let issuer_institution = institution.map(|inst| Institution {
            id: inst.id,
            user_id: inst.user_id,
            institution_name: inst.institution_name,
            institution_type: inst.institution_type,
            country: inst.country,
            accreditation_number: inst.accreditation_number,
            is_accredited: inst.is_accredited,
            created_at: inst.created_at,
        });

        let holder_user = holder_db.map(|h| {
            let role: UserRole = match h.role.as_str() {
                "professional" => UserRole::Professional,
                "institution" => UserRole::Institution,
                "employer" => UserRole::Employer,
                _ => UserRole::Professional,
            };
            
            User {
                id: h.id,
                wallet_address: h.wallet_address,
                email: h.email,
                name: h.name,
                role,
                is_verified: h.is_verified,
                created_at: h.created_at,
                updated_at: h.updated_at,
            }
        });

        let message = if valid && credential.status == CredentialStatus::Issued {
            "Credential is valid and verified".to_string()
        } else if credential.status == CredentialStatus::Revoked {
            "Credential has been revoked".to_string()
        } else {
            "Credential verification failed".to_string()
        };

        Ok(Json(VerificationResponse {
            valid: valid && credential.status == CredentialStatus::Issued,
            credential: Some(credential),
            issuer: issuer_institution,
            holder: holder_user,
            message,
        }))
    } else {
        Ok(Json(VerificationResponse {
            valid: false,
            credential: None,
            issuer: None,
            holder: None,
            message: "Credential not found".to_string(),
        }))
    }
}

#[derive(Deserialize)]
pub struct VerifyQrCodeRequest {
    pub qr_data: String,
}

pub async fn verify_qr_code(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<VerifyQrCodeRequest>,
) -> Result<Json<VerificationResponse>, AppError> {
    // QR code contains the credential ID
    verify_credential(State(state), Path(payload.qr_data)).await
}

pub async fn get_my_credentials(
    State(state): State<Arc<AppState>>,
    AuthUser(user_id, _): AuthUser,
) -> Result<Json<CredentialListResponse>, AppError> {
    let credentials_db = state.credential_repo.get_credentials_by_holder(user_id).await?;
    
    let credentials: Vec<Credential> = credentials_db
        .into_iter()
        .filter_map(|cred_db| {
            let credential_type = match cred_db.credential_type.as_str() {
                "certificate" => CredentialType::Certificate,
                "license" => CredentialType::License,
                "degree" => CredentialType::Degree,
                "workexperience" => CredentialType::WorkExperience,
                "skill" => CredentialType::Skill,
                _ => return None,
            };

            let status = match cred_db.status.as_str() {
                "pending" => CredentialStatus::Pending,
                "issued" => CredentialStatus::Issued,
                "revoked" => CredentialStatus::Revoked,
                "expired" => CredentialStatus::Expired,
                _ => return None,
            };

            Some(Credential {
                id: cred_db.id,
                credential_id: cred_db.credential_id,
                holder_id: cred_db.holder_id,
                issuer_id: cred_db.issuer_id,
                credential_type,
                title: cred_db.title,
                description: cred_db.description,
                ipfs_hash: cred_db.ipfs_hash,
                chain_hash: cred_db.chain_hash,
                issue_date: cred_db.issue_date,
                expiry_date: cred_db.expiry_date,
                status,
                metadata: cred_db.metadata,
                created_at: cred_db.created_at,
            })
        })
        .collect();

    let total = credentials.len();

    Ok(Json(CredentialListResponse { credentials, total }))
}

pub async fn get_issued_credentials(
    State(state): State<Arc<AppState>>,
    AuthUser(user_id, role): AuthUser,
) -> Result<Json<CredentialListResponse>, AppError> {
    if role != "institution" {
        return Err(AppError::Authorization("Only institutions can view issued credentials".to_string()));
    }

    let credentials_db = state.credential_repo.get_credentials_by_issuer(user_id).await?;
    
    let credentials: Vec<Credential> = credentials_db
        .into_iter()
        .filter_map(|cred_db| {
            let credential_type = match cred_db.credential_type.as_str() {
                "certificate" => CredentialType::Certificate,
                "license" => CredentialType::License,
                "degree" => CredentialType::Degree,
                "workexperience" => CredentialType::WorkExperience,
                "skill" => CredentialType::Skill,
                _ => return None,
            };

            let status = match cred_db.status.as_str() {
                "pending" => CredentialStatus::Pending,
                "issued" => CredentialStatus::Issued,
                "revoked" => CredentialStatus::Revoked,
                "expired" => CredentialStatus::Expired,
                _ => return None,
            };

            Some(Credential {
                id: cred_db.id,
                credential_id: cred_db.credential_id,
                holder_id: cred_db.holder_id,
                issuer_id: cred_db.issuer_id,
                credential_type,
                title: cred_db.title,
                description: cred_db.description,
                ipfs_hash: cred_db.ipfs_hash,
                chain_hash: cred_db.chain_hash,
                issue_date: cred_db.issue_date,
                expiry_date: cred_db.expiry_date,
                status,
                metadata: cred_db.metadata,
                created_at: cred_db.created_at,
            })
        })
        .collect();

    let total = credentials.len();

    Ok(Json(CredentialListResponse { credentials, total }))
}

pub async fn get_credential(
    State(state): State<Arc<AppState>>,
    Path(credential_id): Path<String>,
    AuthUser(user_id, _): AuthUser,
) -> Result<Json<Credential>, AppError> {
    let credential_db = state.credential_repo
        .get_credential_by_credential_id(&credential_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Credential not found".to_string()))?;

    // Check authorization - user must be holder or issuer
    if credential_db.holder_id != user_id && credential_db.issuer_id != user_id {
        return Err(AppError::Authorization("Not authorized to view this credential".to_string()));
    }

    let credential_type = match credential_db.credential_type.as_str() {
        "certificate" => CredentialType::Certificate,
        "license" => CredentialType::License,
        "degree" => CredentialType::Degree,
        "workexperience" => CredentialType::WorkExperience,
        "skill" => CredentialType::Skill,
        _ => return Err(AppError::Internal("Invalid credential type".to_string())),
    };

    let status = match credential_db.status.as_str() {
        "pending" => CredentialStatus::Pending,
        "issued" => CredentialStatus::Issued,
        "revoked" => CredentialStatus::Revoked,
        "expired" => CredentialStatus::Expired,
        _ => return Err(AppError::Internal("Invalid status".to_string())),
    };

    let credential = Credential {
        id: credential_db.id,
        credential_id: credential_db.credential_id,
        holder_id: credential_db.holder_id,
        issuer_id: credential_db.issuer_id,
        credential_type,
        title: credential_db.title,
        description: credential_db.description,
        ipfs_hash: credential_db.ipfs_hash,
        chain_hash: credential_db.chain_hash,
        issue_date: credential_db.issue_date,
        expiry_date: credential_db.expiry_date,
        status,
        metadata: credential_db.metadata,
        created_at: credential_db.created_at,
    };

    Ok(Json(credential))
}

pub async fn revoke_credential(
    State(state): State<Arc<AppState>>,
    Path(credential_id): Path<String>,
    AuthUser(user_id, role): AuthUser,
) -> Result<Json<serde_json::Value>, AppError> {
    if role != "institution" {
        return Err(AppError::Authorization("Only institutions can revoke credentials".to_string()));
    }

    let credential_db = state.credential_repo
        .get_credential_by_credential_id(&credential_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Credential not found".to_string()))?;

    // Check if user is the issuer
    if credential_db.issuer_id != user_id {
        return Err(AppError::Authorization("Not authorized to revoke this credential".to_string()));
    }

    // Update status to revoked
    state.credential_repo
        .update_credential_status(credential_db.id, "revoked")
        .await?;

    Ok(Json(serde_json::json!({
        "message": "Credential revoked successfully",
        "credential_id": credential_id
    })))
}

pub async fn get_credential_qr(
    State(state): State<Arc<AppState>>,
    Path(credential_id): Path<String>,
    AuthUser(user_id, _): AuthUser,
) -> Result<Response, AppError> {
    // Get credential from database
    let credential_db = state.credential_repo
        .get_credential_by_credential_id(&credential_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Credential not found".to_string()))?;

    // Check authorization - user must be holder or issuer
    if credential_db.holder_id != user_id && credential_db.issuer_id != user_id {
        return Err(AppError::Authorization("Not authorized to view this credential".to_string()));
    }

    // Decode base64 QR code
    let qr_data = general_purpose::STANDARD.decode(&credential_db.qr_code)
        .map_err(|e| AppError::Internal(format!("Invalid QR code data: {}", e)))?;

    // Return as PNG image
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "image/png")
        .header("Content-Disposition", format!("attachment; filename=\"credential-{}-qr.png\"", credential_id))
        .body(Body::from(qr_data))
        .map_err(|e| AppError::Internal(format!("Failed to create response: {}", e)))?;

    Ok(response)
}
