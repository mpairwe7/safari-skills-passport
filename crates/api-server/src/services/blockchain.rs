use common::AppResult;
use sp_core::crypto::Ss58Codec;
use sp_core::{sr25519, Pair as PairTrait};

pub struct BlockchainService {
    #[allow(dead_code)]
    node_url: String,
}

impl BlockchainService {
    pub async fn new(node_url: &str) -> anyhow::Result<Self> {
        // In a production environment, you would connect to the Substrate node here
        // For this prototype, we'll keep it simple
        Ok(Self {
            node_url: node_url.to_string(),
        })
    }

    /// Generate a new wallet address
    pub fn generate_wallet_address() -> AppResult<String> {
        let (pair, _seed) = sr25519::Pair::generate();
        let public = pair.public();
        Ok(public.to_ss58check())
    }

    /// Record credential hash on blockchain
    pub async fn record_credential_hash(
        &self,
        credential_id: &str,
        ipfs_hash: &str,
    ) -> AppResult<String> {
        // In production, this would submit an extrinsic to the blockchain
        // For this prototype, we'll generate a mock chain hash
        let combined = format!("{}{}", credential_id, ipfs_hash);
        let hash = sp_core::hashing::blake2_256(combined.as_bytes());
        Ok(hex::encode(hash))
    }

    /// Verify credential on blockchain
    pub async fn verify_credential_on_chain(
        &self,
        _credential_id: &str,
    ) -> AppResult<bool> {
        // In production, this would query the blockchain
        // For this prototype, we'll return true to indicate the credential exists
        Ok(true)
    }

    /// Get node URL
    #[allow(dead_code)]
    pub fn get_node_url(&self) -> &str {
        &self.node_url
    }
}

// Add hex dependency for encoding
use hex;
