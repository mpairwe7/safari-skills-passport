use ipfs_api_backend_hyper::{IpfsApi, IpfsClient, TryFromUri};
use common::{AppError, AppResult};
use std::io::Cursor;

pub struct IpfsService {
    client: Option<IpfsClient>,
}

impl IpfsService {
    pub fn new(ipfs_url: &str) -> anyhow::Result<Self> {
        let client = IpfsClient::from_str(ipfs_url)?;
        Ok(Self { client: Some(client) })
    }

    /// Create a mock IPFS service for development
    pub fn mock() -> Self {
        Self { client: None }
    }

    /// Upload data to IPFS and return the hash
    pub async fn upload(&self, data: Vec<u8>) -> AppResult<String> {
        match &self.client {
            Some(client) => {
                let cursor = Cursor::new(data);
                
                let response = client
                    .add(cursor)
                    .await
                    .map_err(|e| AppError::Ipfs(format!("Failed to upload to IPFS: {}", e)))?;

                Ok(response.hash)
            }
            None => {
                // Mock implementation - generate a hash from the data
                use sha2::{Sha256, Digest};
                let mut hasher = Sha256::new();
                hasher.update(&data);
                let hash = hasher.finalize();
                Ok(format!("mock-ipfs-{}", hex::encode(hash)))
            }
        }
    }

    /// Download data from IPFS using hash
    #[allow(dead_code)]
    pub async fn download(&self, hash: &str) -> AppResult<Vec<u8>> {
        match &self.client {
            Some(client) => {
                let data = client
                    .cat(hash)
                    .map_ok(|chunk| chunk.to_vec())
                    .try_concat()
                    .await
                    .map_err(|e| AppError::Ipfs(format!("Failed to download from IPFS: {}", e)))?;

                Ok(data)
            }
            None => {
                // Mock implementation - return empty data
                Err(AppError::Ipfs("IPFS not available in mock mode".to_string()))
            }
        }
    }

    /// Check if content exists on IPFS
    #[allow(dead_code)]
    pub async fn exists(&self, hash: &str) -> bool {
        match &self.client {
            Some(client) => client.object_stat(hash).await.is_ok(),
            None => hash.starts_with("mock-ipfs-"), // Mock hashes start with this prefix
        }
    }
}

// Import for TryStreamExt
use futures::TryStreamExt;
