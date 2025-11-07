use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserDb {
    pub id: Uuid,
    pub wallet_address: String,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub role: String,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct InstitutionDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub institution_name: String,
    pub institution_type: String,
    pub country: String,
    pub accreditation_number: Option<String>,
    pub is_accredited: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CredentialDb {
    pub id: Uuid,
    pub credential_id: String,
    pub holder_id: Uuid,
    pub issuer_id: Uuid,
    pub credential_type: String,
    pub title: String,
    pub description: String,
    pub ipfs_hash: String,
    pub chain_hash: String,
    pub qr_code: String, // Base64 encoded QR code image
    pub issue_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub status: String,
    pub metadata: sqlx::types::JsonValue,
    pub created_at: DateTime<Utc>,
}
