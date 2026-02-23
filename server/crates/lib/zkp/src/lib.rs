pub mod credential;
mod zkp;
pub mod financial_zkp;
pub mod identity;

pub use credential::*;
pub use zkp::*;
pub use financial_zkp::*;
pub use identity::*;

use once_cell::sync::Lazy;
use std::sync::Mutex;

// 初始化状态
static INITIALIZED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

// 初始化Solana配置
pub fn initialize_solana_config(rpc_url: &str, keypair_base64: &str, registry_pubkey: &str) -> Result<(), String> {
    // 设置初始化标志
    let mut initialized = INITIALIZED.lock().unwrap();
    
    // 避免重复初始化
    if *initialized {
        log::info!("Solana配置已经初始化，跳过");
        return Ok(());
    }
    
    // 初始化凭证模块中的配置
    credential::initialize_solana_config(rpc_url, keypair_base64, registry_pubkey);
    
    // 标记为已初始化
    *initialized = true;
    
    log::info!("Solana配置初始化完成");
    Ok(())
}
