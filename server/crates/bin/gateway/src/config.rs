use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    // 数据库配置
    pub mysql: Option<MysqlConfig>,

    // 服务器配置
    pub server: Option<ServerConfig>,

    // Solana 配置
    pub solana_rpc_url: Option<String>,
    pub solana_keypair: Option<String>,
    pub solana_registry_pubkey: Option<String>,
    pub solana_program_id: Option<String>,

    // 存储配置
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

    pub fn default() -> Self {
        Self {
            mysql: Some(MysqlConfig {
                url: "localhost:3306".to_string(),
                username: "root".to_string(),
                password: "password".to_string(),
                database: "adproof".to_string(),
            }),
            server: Some(ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8090,
            }),
            solana_rpc_url: Some("https://api.devnet.solana.com".to_string()),
            solana_keypair: None,
            solana_registry_pubkey: None,
            solana_program_id: Some("CGZST4ic7TB5Mr71LvCBPKV92kMqrSyzzWW6Sge4FqaV".to_string()),
            storage: Some(StorageConfig {
                local_storage_path: "./data".to_string(),
                enable_chain_storage: false,
                enable_ipfs_storage: false,
                ipfs_gateway_url: Some("https://ipfs.io/ipfs/".to_string()),
            }),
        }
    }
}
