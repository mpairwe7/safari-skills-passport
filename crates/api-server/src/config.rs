use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub ipfs_url: String,
    pub blockchain_node_url: String,
    pub environment: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()?,
            ipfs_url: env::var("IPFS_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:5001".to_string()),
            blockchain_node_url: env::var("BLOCKCHAIN_NODE_URL")
                .unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string()),
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
        })
    }
}
