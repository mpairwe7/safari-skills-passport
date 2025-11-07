pub mod auth;
pub mod ipfs;
pub mod blockchain;
pub mod credential;

use std::sync::Arc;
use sqlx::PgPool;
use crate::config::Config;

pub use auth::AuthService;
pub use ipfs::IpfsService;
pub use blockchain::BlockchainService;
pub use credential::CredentialService;

use database::{UserRepositoryImpl, InstitutionRepositoryImpl, CredentialRepositoryImpl};

pub struct AppState {
    #[allow(dead_code)]
    pub config: Config,
    #[allow(dead_code)]
    pub db_pool: PgPool,
    pub auth_service: Arc<AuthService>,
    #[allow(dead_code)]
    pub ipfs_service: Arc<IpfsService>,
    #[allow(dead_code)]
    pub blockchain_service: Arc<BlockchainService>,
    pub credential_service: Arc<CredentialService>,
    pub user_repo: Arc<UserRepositoryImpl>,
    pub institution_repo: Arc<InstitutionRepositoryImpl>,
    pub credential_repo: Arc<CredentialRepositoryImpl>,
}

impl AppState {
    pub async fn new(config: Config, db_pool: PgPool) -> anyhow::Result<Self> {
        let user_repo = Arc::new(UserRepositoryImpl::new(db_pool.clone()));
        let institution_repo = Arc::new(InstitutionRepositoryImpl::new(db_pool.clone()));
        let credential_repo = Arc::new(CredentialRepositoryImpl::new(db_pool.clone()));

        let auth_service = Arc::new(AuthService::new(
            config.jwt_secret.clone(),
            config.jwt_expiration_hours,
        ));

        let ipfs_service = match IpfsService::new(&config.ipfs_url) {
            Ok(service) => Arc::new(service),
            Err(e) => {
                tracing::warn!("Failed to connect to IPFS: {}. Using mock IPFS service.", e);
                Arc::new(IpfsService::mock())
            }
        };
        let blockchain_service = Arc::new(BlockchainService::new(&config.blockchain_node_url).await?);
        
        let credential_service = Arc::new(CredentialService::new(
            ipfs_service.clone(),
            blockchain_service.clone(),
            credential_repo.clone(),
        ));

        Ok(Self {
            config,
            db_pool,
            auth_service,
            ipfs_service,
            blockchain_service,
            credential_service,
            user_repo,
            institution_repo,
            credential_repo,
        })
    }
}
