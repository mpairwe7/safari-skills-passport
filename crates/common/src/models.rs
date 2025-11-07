use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// User types in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Professional,
    Institution,
    Employer,
}

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub wallet_address: String,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Institution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub id: Uuid,
    pub user_id: Uuid,
    pub institution_name: String,
    pub institution_type: String,
    pub country: String,
    pub accreditation_number: Option<String>,
    pub is_accredited: bool,
    pub created_at: DateTime<Utc>,
}

/// Credential types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CredentialType {
    Certificate,
    License,
    Degree,
    WorkExperience,
    Skill,
}

/// Status of a credential
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CredentialStatus {
    Pending,
    Issued,
    Revoked,
    Expired,
}

/// Credential information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: Uuid,
    pub credential_id: String, // Unique on-chain identifier
    pub holder_id: Uuid,
    pub issuer_id: Uuid,
    pub credential_type: CredentialType,
    pub title: String,
    pub description: String,
    pub ipfs_hash: String, // Hash of the credential document stored on IPFS
    pub chain_hash: String, // Hash recorded on blockchain
    pub issue_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub status: CredentialStatus,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Verification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationRequest {
    pub credential_id: String,
    pub verifier_id: Option<Uuid>,
}

/// Verification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResponse {
    pub valid: bool,
    pub credential: Option<Credential>,
    pub issuer: Option<Institution>,
    pub holder: Option<User>,
    pub message: String,
}

// DTO Models for API requests/responses

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize)]
pub struct RegisterInstitutionRequest {
    pub institution_name: String,
    pub institution_type: String,
    pub country: String,
    pub accreditation_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct IssueCredentialRequest {
    pub holder_email: String,
    pub credential_type: CredentialType,
    pub title: String,
    pub description: String,
    pub issue_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub document_data: String, // Base64 encoded document
}

#[derive(Debug, Serialize)]
pub struct IssueCredentialResponse {
    pub credential_id: String,
    pub ipfs_hash: String,
    pub chain_hash: String,
    pub qr_code: String, // Base64 encoded QR code image
}

#[derive(Debug, Serialize)]
pub struct CredentialListResponse {
    pub credentials: Vec<Credential>,
    pub total: usize,
}
