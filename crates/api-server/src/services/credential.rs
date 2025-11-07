use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use base64::{Engine as _, engine::general_purpose};
use common::{AppError, AppResult, IssueCredentialRequest, IssueCredentialResponse, CredentialStatus};
use database::{CredentialDb, CredentialRepositoryImpl, CredentialRepository};
use super::{IpfsService, BlockchainService};

pub struct CredentialService {
    ipfs_service: Arc<IpfsService>,
    blockchain_service: Arc<BlockchainService>,
    credential_repo: Arc<CredentialRepositoryImpl>,
}

impl CredentialService {
    pub fn new(
        ipfs_service: Arc<IpfsService>,
        blockchain_service: Arc<BlockchainService>,
        credential_repo: Arc<CredentialRepositoryImpl>,
    ) -> Self {
        Self {
            ipfs_service,
            blockchain_service,
            credential_repo,
        }
    }

    pub async fn issue_credential(
        &self,
        request: IssueCredentialRequest,
        issuer_id: Uuid,
        holder_id: Uuid,
    ) -> AppResult<IssueCredentialResponse> {
        // Decode base64 document data
        let document_bytes = general_purpose::STANDARD.decode(&request.document_data)
            .map_err(|e| AppError::Validation(format!("Invalid base64 data: {}", e)))?;

        // Upload to IPFS
        let ipfs_hash = self.ipfs_service.upload(document_bytes).await?;

        // Generate unique credential ID
        let credential_id = format!("SSP-{}", Uuid::new_v4());

        // Record on blockchain
        let chain_hash = self.blockchain_service
            .record_credential_hash(&credential_id, &ipfs_hash)
            .await?;

        // Generate QR code
        let qr_code = crate::utils::qr::generate_qr_code(&credential_id)?;

        // Save to database
        let credential = CredentialDb {
            id: Uuid::new_v4(),
            credential_id: credential_id.clone(),
            holder_id,
            issuer_id,
            credential_type: format!("{:?}", request.credential_type).to_lowercase(),
            title: request.title,
            description: request.description,
            ipfs_hash: ipfs_hash.clone(),
            chain_hash: chain_hash.clone(),
            qr_code: qr_code.clone(), // Store QR code in database
            issue_date: request.issue_date,
            expiry_date: request.expiry_date,
            status: format!("{:?}", CredentialStatus::Issued).to_lowercase(),
            metadata: request.metadata,
            created_at: Utc::now(),
        };

        self.credential_repo.create_credential(&credential).await?;

        Ok(IssueCredentialResponse {
            credential_id,
            ipfs_hash,
            chain_hash,
            qr_code,
        })
    }

    pub async fn verify_credential(&self, credential_id: &str) -> AppResult<bool> {
        // Check if credential exists in database
        let credential = self.credential_repo
            .get_credential_by_credential_id(credential_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Credential not found".to_string()))?;

        // Check status
        if credential.status != "issued" {
            return Ok(false);
        }

        // Verify on blockchain
        let on_chain_valid = self.blockchain_service
            .verify_credential_on_chain(credential_id)
            .await?;

        Ok(on_chain_valid)
    }
}
