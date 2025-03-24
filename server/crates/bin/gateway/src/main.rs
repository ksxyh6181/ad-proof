use once_cell::sync::Lazy;
use rbatis::RBatis;
use salvo::conn::TcpListener;
use salvo::http::request::set_global_secure_max_size;
use salvo::{Listener, Server};

use tracing;
use tracing_subscriber::util::SubscriberInitExt;
use zkp;

mod controller;
mod router;
mod utils;
mod middleware;
mod config;

const IP: &str = "0.0.0.0";
const PORT: i32 = 8090;

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    //0.系统参数初始化
    log::warn!("Start…………………………………server ip:{:?},port:{:?}", IP,PORT);
    
    // 加载配置
    let config = config::AppConfig::load("config/app_config.json").unwrap_or_else(|e| {
        log::warn!("加载配置文件失败: {}，使用默认配置", e);
        config::AppConfig::default()
    });
    
    // 初始化Solana存储
    if let Some(ref rpc_url) = config.solana_rpc_url {
        if let Some(ref keypair) = config.solana_keypair {
            if let Some(ref registry) = config.solana_registry_pubkey {
                log::info!("正在初始化Solana存储...");
                log::info!("Solana配置: RPC URL={}, 密钥={}, 注册表={}", rpc_url, keypair, registry);
                zkp::initialize_solana_config(rpc_url, keypair, registry);
                log::info!("Solana存储初始化完成");
                
                // 检查并初始化链上注册表
                log::info!("正在检查并初始化Solana链上注册表...");
                match solana_client::initialize_registry_if_needed(
                    rpc_url.to_string(),
                    keypair.to_string(),
                    registry.to_string()
                ).await {
                    Ok(_) => log::info!("Solana链上注册表检查/初始化完成"),
                    Err(e) => log::error!("Solana链上注册表初始化失败: {}", e)
                }
            } else {
                log::warn!("缺少Solana注册表公钥配置，跳过Solana存储初始化");
            }
        } else {
            log::warn!("缺少Solana密钥对配置，跳过Solana存储初始化");
        }
    } else {
        log::warn!("缺少Solana RPC URL配置，跳过Solana存储初始化");
    }
    
    
    // 初始化金融凭证系统
    log::info!("正在初始化金融凭证系统...");
    zkp::financial_zkp::initialize_financial_credentials();
    log::info!("金融凭证系统初始化完成");
    
    // 初始化学历证书系统
    log::info!("正在初始化学历证书系统...");
    zkp::credential::initialize_credential_system();
    log::info!("学历证书系统初始化完成");
    
    let service = router::init_service();
    let address = format!("{}:{}", IP, PORT);
    Server::new(TcpListener::new(address).bind().await).serve(service).await;
}
