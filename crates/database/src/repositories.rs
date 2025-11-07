use sqlx::PgPool;
use uuid::Uuid;
use async_trait::async_trait;
use common::{AppError, AppResult};
use crate::models::*;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &UserDb) -> AppResult<UserDb>;
    async fn get_user_by_id(&self, id: Uuid) -> AppResult<Option<UserDb>>;
    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<UserDb>>;
    async fn get_user_by_wallet(&self, wallet_address: &str) -> AppResult<Option<UserDb>>;
    async fn update_user_verification(&self, id: Uuid, verified: bool) -> AppResult<()>;
}

#[async_trait]
pub trait InstitutionRepository: Send + Sync {
    async fn create_institution(&self, institution: &InstitutionDb) -> AppResult<InstitutionDb>;
    async fn get_institution_by_user_id(&self, user_id: Uuid) -> AppResult<Option<InstitutionDb>>;
    async fn get_institution_by_id(&self, id: Uuid) -> AppResult<Option<InstitutionDb>>;
    async fn update_accreditation(&self, id: Uuid, accredited: bool) -> AppResult<()>;
}

#[async_trait]
pub trait CredentialRepository: Send + Sync {
    async fn create_credential(&self, credential: &CredentialDb) -> AppResult<CredentialDb>;
    async fn get_credential_by_id(&self, id: Uuid) -> AppResult<Option<CredentialDb>>;
    async fn get_credential_by_credential_id(&self, credential_id: &str) -> AppResult<Option<CredentialDb>>;
    async fn get_credentials_by_holder(&self, holder_id: Uuid) -> AppResult<Vec<CredentialDb>>;
    async fn get_credentials_by_issuer(&self, issuer_id: Uuid) -> AppResult<Vec<CredentialDb>>;
    async fn update_credential_status(&self, id: Uuid, status: &str) -> AppResult<()>;
}

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, user: &UserDb) -> AppResult<UserDb> {
        let result = sqlx::query_as::<_, UserDb>(
            r#"
            INSERT INTO users (id, wallet_address, email, password_hash, name, role, is_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(user.id)
        .bind(&user.wallet_address)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.name)
        .bind(&user.role)
        .bind(user.is_verified)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn get_user_by_id(&self, id: Uuid) -> AppResult<Option<UserDb>> {
        let result = sqlx::query_as::<_, UserDb>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<UserDb>> {
        let result = sqlx::query_as::<_, UserDb>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn get_user_by_wallet(&self, wallet_address: &str) -> AppResult<Option<UserDb>> {
        let result = sqlx::query_as::<_, UserDb>(
            "SELECT * FROM users WHERE wallet_address = $1"
        )
        .bind(wallet_address)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn update_user_verification(&self, id: Uuid, verified: bool) -> AppResult<()> {
        sqlx::query("UPDATE users SET is_verified = $1, updated_at = $2 WHERE id = $3")
            .bind(verified)
            .bind(chrono::Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}

pub struct InstitutionRepositoryImpl {
    pool: PgPool,
}

impl InstitutionRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InstitutionRepository for InstitutionRepositoryImpl {
    async fn create_institution(&self, institution: &InstitutionDb) -> AppResult<InstitutionDb> {
        let result = sqlx::query_as::<_, InstitutionDb>(
            r#"
            INSERT INTO institutions (id, user_id, institution_name, institution_type, country, accreditation_number, is_accredited, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(institution.id)
        .bind(institution.user_id)
        .bind(&institution.institution_name)
        .bind(&institution.institution_type)
        .bind(&institution.country)
        .bind(&institution.accreditation_number)
        .bind(institution.is_accredited)
        .bind(institution.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn get_institution_by_user_id(&self, user_id: Uuid) -> AppResult<Option<InstitutionDb>> {
        let result = sqlx::query_as::<_, InstitutionDb>(
            "SELECT * FROM institutions WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn get_institution_by_id(&self, id: Uuid) -> AppResult<Option<InstitutionDb>> {
        let result = sqlx::query_as::<_, InstitutionDb>(
            "SELECT * FROM institutions WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn update_accreditation(&self, id: Uuid, accredited: bool) -> AppResult<()> {
        sqlx::query("UPDATE institutions SET is_accredited = $1 WHERE id = $2")
            .bind(accredited)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}

pub struct CredentialRepositoryImpl {
    pool: PgPool,
}

impl CredentialRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CredentialRepository for CredentialRepositoryImpl {
    async fn create_credential(&self, credential: &CredentialDb) -> AppResult<CredentialDb> {
        let result = sqlx::query_as::<_, CredentialDb>(
            r#"
            INSERT INTO credentials (
                id, credential_id, holder_id, issuer_id, credential_type, 
                title, description, ipfs_hash, chain_hash, qr_code, issue_date, 
                expiry_date, status, metadata, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(credential.id)
        .bind(&credential.credential_id)
        .bind(credential.holder_id)
        .bind(credential.issuer_id)
        .bind(&credential.credential_type)
        .bind(&credential.title)
        .bind(&credential.description)
        .bind(&credential.ipfs_hash)
        .bind(&credential.chain_hash)
        .bind(&credential.qr_code)
        .bind(credential.issue_date)
        .bind(credential.expiry_date)
        .bind(&credential.status)
        .bind(&credential.metadata)
        .bind(credential.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn get_credential_by_id(&self, id: Uuid) -> AppResult<Option<CredentialDb>> {
        let result = sqlx::query_as::<_, CredentialDb>(
            "SELECT * FROM credentials WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn get_credential_by_credential_id(&self, credential_id: &str) -> AppResult<Option<CredentialDb>> {
        let result = sqlx::query_as::<_, CredentialDb>(
            "SELECT * FROM credentials WHERE credential_id = $1"
        )
        .bind(credential_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn get_credentials_by_holder(&self, holder_id: Uuid) -> AppResult<Vec<CredentialDb>> {
        let result = sqlx::query_as::<_, CredentialDb>(
            "SELECT * FROM credentials WHERE holder_id = $1 ORDER BY created_at DESC"
        )
        .bind(holder_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn get_credentials_by_issuer(&self, issuer_id: Uuid) -> AppResult<Vec<CredentialDb>> {
        let result = sqlx::query_as::<_, CredentialDb>(
            "SELECT * FROM credentials WHERE issuer_id = $1 ORDER BY created_at DESC"
        )
        .bind(issuer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn update_credential_status(&self, id: Uuid, status: &str) -> AppResult<()> {
        sqlx::query("UPDATE credentials SET status = $1 WHERE id = $2")
            .bind(status)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
