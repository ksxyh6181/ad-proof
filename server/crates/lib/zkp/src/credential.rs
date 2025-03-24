use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::zkp::ZKProof;
use salvo::oapi::ToSchema;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use log;
use tokio;
use solana_client;

// 全局存储Solana配置
static SOLANA_CONFIG: Lazy<Mutex<Option<SolanaConfig>>> = Lazy::new(|| {
    Mutex::new(None)
});

// Solana配置结构
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub keypair_base58: String,
    pub registry_pubkey_base58: String,
}

// 初始化Solana配置
pub fn initialize_solana_config(rpc_url: &str, keypair_base58: &str, registry_pubkey_base58: &str) {
    let config = SolanaConfig {
        rpc_url: rpc_url.to_string(),
        keypair_base58: keypair_base58.to_string(),
        registry_pubkey_base58: registry_pubkey_base58.to_string(),
    };
    
    let mut solana_config = SOLANA_CONFIG.lock().unwrap();
    *solana_config = Some(config);
    log::info!("Solana配置已初始化: {}, {}, {}", rpc_url, keypair_base58, registry_pubkey_base58);
}

// 全局存储证书数据
pub static CREDENTIALS: Lazy<Mutex<HashMap<String, Credential>>> = Lazy::new(|| {
    let credentials = load_credentials().unwrap_or_else(|_| {
        log::warn!("加载学历证书失败，使用空集合");
        HashMap::new()
    });
    Mutex::new(credentials)
});

// 存储证书到文件系统
fn save_credentials(credentials: &HashMap<String, Credential>) -> io::Result<()> {
    // 确保目录存在
    let dir_path = Path::new("data");
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }

    let file_path = dir_path.join("education_credentials.json");
    log::info!("【存储】保存学历证书到文件 {}", file_path.display());
    log::info!("【存储】证书数量: {}", credentials.len());

    // 序列化证书
    let json = serde_json::to_string_pretty(credentials).map_err(|e| {
        log::error!("【存储】序列化证书失败: {}", e);
        io::Error::new(io::ErrorKind::InvalidData, e)
    })?;

    // 写入文件
    fs::write(&file_path, json).map_err(|e| {
        log::error!("【存储】写入文件失败: {}", e);
        e
    })?;

    log::info!("【存储】证书保存成功");
    Ok(())
}

// 从文件系统加载证书
fn load_credentials() -> io::Result<HashMap<String, Credential>> {
    let file_path = Path::new("data").join("education_credentials.json");
    log::info!("【加载】尝试从 {} 加载证书", file_path.display());

    // 如果文件不存在，返回空HashMap
    if !file_path.exists() {
        log::info!("【加载】证书文件不存在，返回空集合");
        return Ok(HashMap::new());
    }

    // 读取文件内容
    let contents = fs::read_to_string(&file_path).map_err(|e| {
        log::error!("【加载】读取文件失败: {}", e);
        e
    })?;

    if contents.trim().is_empty() {
        log::info!("【加载】证书文件为空，返回空集合");
        return Ok(HashMap::new());
    }

    // 解析JSON
    let credentials: HashMap<String, Credential> = serde_json::from_str(&contents).map_err(|e| {
        log::error!("【加载】解析证书JSON失败: {}", e);
        io::Error::new(io::ErrorKind::InvalidData, e)
    })?;

    log::info!("【加载】成功加载 {} 个证书", credentials.len());
    Ok(credentials)
}

#[derive(Debug, Serialize, Deserialize, Clone,ToSchema)]
pub struct Credential {
    pub student_id: String,
    pub name: String,
    pub degree: String,
    pub graduation_date: String,
    pub hash: Option<String>,
    pub proof: Option<ZKProof>,
}

impl Credential {
    pub fn new(student_id: String, name: String, degree: String, graduation_date: String) -> Self {
        Self {
            student_id,
            name,
            degree,
            graduation_date,
            hash: None,
            proof: None,
        }
    }

    pub fn store(&mut self) -> Result<String, String> {
        // 验证证明是否存在
        if self.proof.is_none() {
            log::warn!("【存储】警告：将要保存的证书没有附加零知识证明");
        }
        
        // 获取全局证书存储的锁
        let mut credentials = match CREDENTIALS.lock() {
            Ok(c) => c,
            Err(e) => return Err(format!("获取全局证书锁失败: {}", e)),
        };
        
        // 计算hash
        let hash = self.calculate_hash();
        log::info!("【存储】保存证书: hash={}", hash);
        
        // 确保 hash 字段已设置
        self.hash = Some(hash.clone());
        
        // 更新到内存中
        credentials.insert(hash.clone(), self.clone());
        
        // 持久化存储
        if let Err(e) = save_credentials(&credentials) {
            log::error!("【存储】保存证书到文件系统失败: {}", e);
            // 即使持久化失败，我们仍然继续，因为内存中的证书已经更新
        }
        
        // 存储到链上
        let result = match self.store_on_chain(&hash) {
            Ok(tx_signature) => {
                log::info!("【存储】证书已存储到区块链，交易ID: {}", tx_signature);
                log::info!("【存储】证书存储成功");
                Ok(hash)
            },
            Err(e) => {
                log::error!("【存储】链上存储失败: {}", e);
                log::warn!("【存储】证书存储部分失败，但本地存储已经完成");
                Ok(hash)
            }
        };
        
        result
    }
    
    // 将证书存储到区块链
    fn store_on_chain(&self, hash: &str) -> Result<String, String> {
        log::info!("【store_on_chain】开始将证书存储到区块链: {}", hash);
        
        // 获取Solana配置
        let config = match *SOLANA_CONFIG.lock().unwrap() {
            Some(ref config) => {
                log::info!("【store_on_chain】获取Solana配置成功: URL={}", config.rpc_url);
                config.clone()
            },
            None => {
                log::error!("【store_on_chain】Solana配置未初始化");
                return Err("Solana配置未初始化".to_string());
            }
        };
        
        // 将证书转换为JSON格式的元数据
        let metadata = match serde_json::to_string(self) {
            Ok(json) => {
                log::debug!("【store_on_chain】证书序列化成功，长度: {} 字节", json.len());
                json
            },
            Err(e) => {
                log::error!("【store_on_chain】序列化证书失败: {}", e);
                return Err(format!("序列化证书失败: {}", e));
            }
        };
        
        // 使用新的线程来执行异步操作，避免运行时嵌套问题
        log::info!("【store_on_chain】准备创建新线程执行异步存储操作");
        let hash_str = hash.to_string();
        let metadata_str = metadata.clone();
        let rpc_url = config.rpc_url.clone();
        let keypair_base58 = config.keypair_base58.clone();
        let registry_pubkey_base58 = config.registry_pubkey_base58.clone();
        
        // 在新线程中创建tokio运行时
        log::info!("【store_on_chain】创建新线程...");
        match std::thread::spawn(move || {
            log::info!("【store_on_chain_thread】开始执行...");
            // 使用多线程运行时而不是current_thread
            let rt = match tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)  // 设置工作线程数
                .enable_all()
                .build() {
                    Ok(rt) => {
                        log::info!("【store_on_chain_thread】创建多线程Tokio运行时成功");
                        rt
                    },
                    Err(e) => {
                        log::error!("【store_on_chain_thread】创建Tokio运行时失败: {}", e);
                        return Err(format!("创建Tokio运行时失败: {}", e));
                    }
                };
            
            // 在新的运行时中执行异步操作
            log::info!("【store_on_chain_thread】开始执行异步存储操作");
            let result = rt.block_on(async {
                log::info!("【store_on_chain_async】开始调用solana_client...");
                solana_client::store_credential_on_chain(hash_str, metadata_str, rpc_url, keypair_base58, registry_pubkey_base58).await
                    .map_err(|e| {
                        log::error!("【store_on_chain_async】链上存储证书失败: {}", e);
                        format!("链上存储证书失败: {}", e)
                    })
            });
            log::info!("【store_on_chain_thread】异步操作完成: {:?}", result.is_ok());
            result
        }).join() {
            Ok(result) => {
                log::info!("【store_on_chain】线程执行完成");
                result
            },
            Err(e) => {
                log::error!("【store_on_chain】线程执行失败: {:?}", e);
                Err("线程执行失败".to_string())
            }
        }
    }

    pub fn get(hash: &str) -> Option<Self> {
        log::info!("【查询】根据hash获取证书: {}", hash);
        
        let credentials = match CREDENTIALS.lock() {
            Ok(c) => c,
            Err(e) => {
                log::error!("【查询】获取证书锁失败: {}", e);
                return None;
            }
        };
        
        let credential = credentials.get(hash).cloned();
        
        if credential.is_none() {
            log::warn!("【查询】未找到hash为{}的证书", hash);
        } else {
            log::info!("【查询】成功获取证书");
        }
        
        credential
    }
    
    pub fn new_from_hash(hash: &str) -> Result<Self, String> {
        Self::get(hash).ok_or_else(|| format!("未找到hash为{}的证书", hash))
    }

    pub fn verify(&self) -> Result<bool, String> {
        let proof = self.proof.as_ref().ok_or("证明不存在")?;
        
        // 实现使用zkp模块的验证
        match crate::zkp::verify_zk_proof(proof) {
            Ok(_) => {
                log::info!("【验证】证书验证成功");
                Ok(true)
            },
            Err(e) => {
                log::error!("【验证】证书验证失败: {:?}", e);
                Err(format!("证明验证失败: {:?}", e))
            }
        }
    }
    
    fn calculate_hash(&self) -> String {
        let hash_string = format!("{}{}{}{}",
            self.student_id,
            self.name,
            self.degree,
            self.graduation_date
        );
        
        let hash_bytes = blake3::hash(hash_string.as_bytes());
        hex::encode(hash_bytes.as_bytes())
    }
}

pub fn initialize_credential_system() {
    log::info!("初始化学历证书系统");
    
    // 触发加载证书
    let credentials = CREDENTIALS.lock().unwrap();
    log::info!("当前内存中的证书数量: {}", credentials.len());
    
    // 此处可以添加其他初始化逻辑
    
    log::info!("学历证书系统初始化完成");
}
