use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub mysql: Option<MysqlConfig>,
    pub server: Option<ServerConfig>,
    pub storage: Option<StorageConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysqlConfig {
    pub url: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub local_storage_path: String,
    pub enable_chain_storage: bool,
    pub enable_ipfs_storage: bool,
    pub ipfs_gateway_url: Option<String>,
}

impl AppConfig {
    pub fn load(config_path: &str) -> Result<Self> {
        let config_content = fs::read_to_string(config_path)?;
        let config: AppConfig = serde_json::from_str(&config_content)?;
        Ok(config)
    }
}
