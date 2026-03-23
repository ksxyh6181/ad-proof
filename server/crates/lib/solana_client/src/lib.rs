use anyhow::{anyhow, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use log::{error, info};
use solana_client_sdk::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;
use std::sync::Arc;
use std::fs::File;
use std::io::Read;

// 更新的程序ID - 从Anchor.toml中获取
pub const CREDENTIAL_PROGRAM_ID: &str = "CGZST4ic7TB5Mr71LvCBPKV92kMqrSyzzWW6Sge4FqaV";

/// Solana客户端，用于与凭证注册表交互
pub struct SolanaClient {
    rpc_client: RpcClient,
    program_id: Pubkey,
    payer: Keypair,
}

/// Anchor指令枚举
#[derive(BorshSerialize, BorshDeserialize)]
pub enum CredentialInstruction {
    /// 初始化注册表
    Initialize,
    /// 注册新凭证
    RegisterCredential {
        hash: String,
        credential_type: String,
        issuer: String,
        issue_date: i64,
        metadata_uri: String,
    },
    /// 验证凭证
    VerifyCredential,
    /// 撤销凭证
    RevokeCredential,
    /// 更新凭证元数据
    UpdateMetadata {
        new_metadata_uri: String,
    },
}

// 生成Anchor指令的discriminator
fn get_instruction_discriminator(instruction_name: &str) -> [u8; 8] {
    // 根据IDL中的实际discriminator值
    let discriminator = match instruction_name {
        "initialize" => [175, 175, 109, 31, 13, 152, 155, 237],
        "register_credential" => [49, 166, 103, 150, 225, 87, 131, 212],  // 需要替换为正确的值
        "verify_credential" => [140, 176, 3, 173, 23, 2, 90, 116],        // 需要替换为正确的值
        "revoke_credential" => [41, 128, 230, 44, 155, 213, 214, 143],    // 需要替换为正确的值
        "update_metadata" => [155, 175, 166, 87, 56, 250, 176, 238],      // 需要替换为正确的值
        _ => {
            // 如果没有匹配的指令名称，使用通用的生成方法
            log::warn!("未找到 '{}' 的预定义discriminator，尝试动态生成", instruction_name);
            use sha2::{Digest, Sha256};
            let preimage = format!("{}:{}", instruction_name, instruction_name);
            let mut hasher = Sha256::new();
            hasher.update(preimage.as_bytes());
            let hash = hasher.finalize();
            let mut discriminator = [0u8; 8];
            discriminator.copy_from_slice(&hash[..8]);
            discriminator
        }
    };
    
    log::info!("使用 '{}' 指令的discriminator: {:?}", instruction_name, &discriminator);
    discriminator
}

/// 用于保存从链上获取的凭证信息
#[derive(Debug, Clone)]
pub struct SolanaCredentialMetadata {
    pub hash: String,
    pub credential_type: String,
    pub issuer: String,
    pub issue_date: i64,
    pub metadata_uri: String,
    pub owner: Pubkey,
    pub revoked: bool,
    pub registry: Pubkey,
    pub seeds: [u8; 8],
}

/// 注册表账户数据结构
#[derive(BorshDeserialize)]
pub struct RegistryAccount {
    pub authority: Pubkey,
    pub credential_count: u64,
}

/// 凭证账户数据结构
#[derive(BorshDeserialize)]
pub struct CredentialAccount {
    pub hash: String,
    pub credential_type: String,
    pub issuer: String,
    pub issue_date: i64,
    pub metadata_uri: String,
    pub owner: Pubkey,
    pub revoked: bool,
    pub registry: Pubkey,
    pub seeds: [u8; 8],
}

impl SolanaClient {
    /// 创建新的Solana客户端
    pub fn new(rpc_url: &str, payer: Keypair) -> Result<Self> {
        let rpc_client = RpcClient::new(rpc_url.to_string());
        let program_id = Pubkey::from_str(CREDENTIAL_PROGRAM_ID)?;

        Ok(Self {
            rpc_client,
            program_id,
            payer,
        })
    }

    /// 初始化注册表
    pub fn initialize_registry(&self, registry_keypair: &Keypair) -> Result<String> {
        info!("初始化凭证注册表");

        // 创建初始化指令
        let data = borsh::to_vec(&CredentialInstruction::Initialize)?;
        // 使用与Anchor程序中完全相同的函数名
        let discriminator = get_instruction_discriminator("initialize");
        let mut data_with_discriminator = Vec::new();
        data_with_discriminator.extend_from_slice(&discriminator);
        data_with_discriminator.extend_from_slice(&data);
        
        let accounts = vec![
            AccountMeta::new(registry_keypair.pubkey(), true),
            AccountMeta::new(self.payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];
        
        let instruction = Instruction {
            program_id: self.program_id,
            accounts,
            data: data_with_discriminator,
        };
        
        // 创建并提交交易
        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.payer.pubkey()),
            &[&self.payer, registry_keypair],
            recent_blockhash,
        );
        
        // 发送交易
        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        info!("注册表初始化成功, 交易ID: {}", signature);
        
        Ok(signature.to_string())
    }

    /// 注册新凭证
    pub fn register_credential(
        &self, 
        registry_pubkey: &Pubkey,
        hash: &str,
        credential_type: &str,
        issuer: &str,
        issue_date: i64,
        metadata_uri: &str,
    ) -> Result<String> {
        // 首先获取当前凭证计数来确定种子
        let account = self.rpc_client.get_account(registry_pubkey)?;

        log::warn!("register_credential account: {:?}", &account.data.to_vec());
        
        // 检查账户数据是否有效
        if account.data.is_empty() {
            return Err(anyhow!("注册表账户数据为空，注册表可能尚未初始化。请先初始化注册表"));
        }
        
        if account.data.len() < 8 {
            return Err(anyhow!("注册表账户数据长度不足，期望至少8字节，实际: {}", account.data.len()));
        }
        
        // 跳过Anchor账户的8字节discriminator
        let mut account_data = &account.data[8..];
        let registry = RegistryAccount::deserialize(&mut account_data)?;
        let credential_count = registry.credential_count;
        
        info!("注册新凭证, 当前计数器: {}", credential_count);
        
        // 使用计数器作为种子
        let seed_data = credential_count.to_le_bytes();
        
        // 查找凭证PDA
        let (credential_pda, _) = Pubkey::find_program_address(
            &[
                b"credential",
                registry_pubkey.as_ref(),
                &seed_data,
            ],
            &self.program_id,
        );
        
        info!("计算的凭证PDA: {}", credential_pda);
        
        // 创建注册指令
        let data = borsh::to_vec(&CredentialInstruction::RegisterCredential {
            hash: hash.to_string(),
            credential_type: credential_type.to_string(),
            issuer: issuer.to_string(),
            issue_date,
            metadata_uri: metadata_uri.to_string(),
        })?;
        
        let discriminator = get_instruction_discriminator("register_credential");
        let mut data_with_discriminator = Vec::new();
        data_with_discriminator.extend_from_slice(&discriminator);
        data_with_discriminator.extend_from_slice(&data);
        
        let accounts = vec![
            AccountMeta::new(*registry_pubkey, false),
            AccountMeta::new(credential_pda, false),
            AccountMeta::new(self.payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];
        
        let instruction = Instruction {
            program_id: self.program_id,
            accounts,
            data: data_with_discriminator,
        };
        
        // 创建并提交交易
        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            recent_blockhash,
        );
        
        // 发送交易
        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        info!("凭证注册成功, 交易ID: {}", signature);
        
        Ok(signature.to_string())
    }
    
    /// 验证凭证
    pub fn verify_credential(
        &self,
        registry_pubkey: &Pubkey,
        hash: &str,
    ) -> Result<bool> {
        // 首先获取凭证数据
        let credential_data = self.get_credential_data(registry_pubkey, hash)?;
        if let Some(credential) = credential_data {
            info!("找到凭证, 开始验证");
            
            // 使用存储的种子找到PDA
            let (credential_pda, _) = Pubkey::find_program_address(
                &[
                    b"credential",
                    registry_pubkey.as_ref(),
                    &credential.seeds,
                ],
                &self.program_id,
            );
            
            // 创建指令
            let data = borsh::to_vec(&CredentialInstruction::VerifyCredential)?;
            
            let discriminator = get_instruction_discriminator("verify_credential");
            let mut data_with_discriminator = Vec::new();
            data_with_discriminator.extend_from_slice(&discriminator);
            data_with_discriminator.extend_from_slice(&data);
            
            let accounts = vec![
                AccountMeta::new_readonly(*registry_pubkey, false),
                AccountMeta::new_readonly(credential_pda, false),
            ];
            
            let instruction = Instruction {
                program_id: self.program_id,
                accounts,
                data: data_with_discriminator,
            };
            
            // 创建并提交交易
            let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&self.payer.pubkey()),
                &[&self.payer],
                recent_blockhash,
            );
            
            // 尝试发送交易
            match self.rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(signature) => {
                    info!("凭证验证成功, 交易ID: {}", signature);
                    Ok(true)
                },
                Err(e) => {
                    // 如果错误是由于凭证被撤销，返回false
                    if e.to_string().contains("CredentialRevoked") {
                        info!("凭证已被撤销");
                        Ok(false)
                    } else {
                        error!("凭证验证失败: {}", e);
                        Err(anyhow!("凭证验证失败: {}", e))
                    }
                }
            }
        } else {
            info!("凭证未找到: {}", hash);
            Ok(false)
        }
    }
    
    /// 撤销凭证
    pub fn revoke_credential(
        &self,
        registry_pubkey: &Pubkey,
        hash: &str,
    ) -> Result<String> {
        // 首先获取凭证数据
        let credential_data = self.get_credential_data(registry_pubkey, hash)?;
        if let Some(credential) = credential_data {
            info!("找到凭证, 开始撤销");
            
            // 使用存储的种子找到PDA
            let (credential_pda, _) = Pubkey::find_program_address(
                &[
                    b"credential",
                    registry_pubkey.as_ref(),
                    &credential.seeds,
                ],
                &self.program_id,
            );
            
            // 创建指令
            let data = borsh::to_vec(&CredentialInstruction::RevokeCredential)?;
            
            let discriminator = get_instruction_discriminator("revoke_credential");
            let mut data_with_discriminator = Vec::new();
            data_with_discriminator.extend_from_slice(&discriminator);
            data_with_discriminator.extend_from_slice(&data);
            
            let accounts = vec![
                AccountMeta::new(*registry_pubkey, false),
                AccountMeta::new(credential_pda, false),
                AccountMeta::new(self.payer.pubkey(), true),
            ];
            
            let instruction = Instruction {
                program_id: self.program_id,
                accounts,
                data: data_with_discriminator,
            };
            
            // 创建并提交交易
            let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&self.payer.pubkey()),
                &[&self.payer],
                recent_blockhash,
            );
            
            // 发送交易
            let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
            info!("凭证撤销成功, 交易ID: {}", signature);
            
            Ok(signature.to_string())
        } else {
            return Err(anyhow!("未找到凭证: {}", hash));
        }
    }
    
    /// 更新凭证元数据
    pub fn update_metadata(
        &self,
        registry_pubkey: &Pubkey,
        hash: &str,
        new_metadata_uri: &str,
    ) -> Result<String> {
        // 首先获取凭证数据
        let credential_data = self.get_credential_data(registry_pubkey, hash)?;
        if let Some(credential) = credential_data {
            info!("找到凭证, 开始更新元数据");
            
            // 使用存储的种子找到PDA
            let (credential_pda, _) = Pubkey::find_program_address(
                &[
                    b"credential",
                    registry_pubkey.as_ref(),
                    &credential.seeds,
                ],
                &self.program_id,
            );
            
            // 创建指令
            let data = borsh::to_vec(&CredentialInstruction::UpdateMetadata {
                new_metadata_uri: new_metadata_uri.to_string(),
            })?;
            
            let discriminator = get_instruction_discriminator("update_metadata");
            let mut data_with_discriminator = Vec::new();
            data_with_discriminator.extend_from_slice(&discriminator);
            data_with_discriminator.extend_from_slice(&data);
            
            let accounts = vec![
                AccountMeta::new_readonly(*registry_pubkey, false),
                AccountMeta::new(credential_pda, false),
                AccountMeta::new(self.payer.pubkey(), true),
            ];
            
            let instruction = Instruction {
                program_id: self.program_id,
                accounts,
                data: data_with_discriminator,
            };
            
            // 创建并提交交易
            let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
            let transaction = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&self.payer.pubkey()),
                &[&self.payer],
                recent_blockhash,
            );
            
            // 发送交易
            let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
            info!("凭证元数据更新成功, 交易ID: {}", signature);
            
            Ok(signature.to_string())
        } else {
            return Err(anyhow!("未找到凭证: {}", hash));
        }
    }
    
    /// 从链上获取凭证数据
    pub fn get_credential_data(
        &self,
        registry_pubkey: &Pubkey,
        hash: &str,
    ) -> Result<Option<SolanaCredentialMetadata>> {
        info!("查找凭证数据: {}", hash);
        
        // 获取所有程序账户
        let accounts = self.rpc_client.get_program_accounts(&self.program_id)?;
        
        for (pubkey, account) in accounts {
            // 账户数据至少需要8字节的discriminator
            if account.data.len() <= 8 {
                continue;
            }
            
            // 检查账户discriminator是否匹配凭证账户
            // Anchor会在账户数据的前8个字节存储discriminator
            // 这里我们简化处理，直接尝试解析所有账户
            
            // 跳过Anchor账户的8字节discriminator
            let mut account_data = &account.data[8..];
            
            // 尝试解析凭证数据
            if let Ok(credential) = CredentialAccount::deserialize(&mut account_data) {
                if credential.hash == hash && credential.registry == *registry_pubkey {
                    info!("找到匹配的凭证, 地址: {}", pubkey);
                    return Ok(Some(SolanaCredentialMetadata {
                        hash: credential.hash,
                        credential_type: credential.credential_type,
                        issuer: credential.issuer,
                        issue_date: credential.issue_date,
                        metadata_uri: credential.metadata_uri,
                        owner: credential.owner,
                        revoked: credential.revoked,
                        registry: credential.registry,
                        seeds: credential.seeds,
                    }));
                }
            }
        }
        
        info!("未找到凭证: {}", hash);
        Ok(None)
    }
    
    /// 获取注册表信息
    pub fn get_registry_info(&self, registry_pubkey: &Pubkey) -> Result<RegistryAccount> {
        let account = self.rpc_client.get_account(registry_pubkey)?;
        
        // 跳过Anchor账户的8字节discriminator
        let mut account_data = &account.data[8..];
        let registry = RegistryAccount::deserialize(&mut account_data)?;
        
        Ok(registry)
    }
}

// 实现从字节数组反序列化的辅助函数
fn try_from_slice_unchecked<T: BorshDeserialize>(data: &[u8]) -> Result<T> {
    let mut data_mut = &data[..];
    let result = T::deserialize(&mut data_mut)?;
    Ok(result)
}

// 定义一个对象安全的 Storage trait
#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    
    async fn store_credential(&self, credential_hash: &str, metadata_uri: &str) -> Result<String>;
    async fn verify_credential(&self, credential_hash: &str) -> Result<bool>;
    async fn get_credential_metadata(&self, credential_hash: &str) -> Result<Option<String>>;
    async fn revoke_credential(&self, credential_hash: &str) -> Result<()>;
    async fn update_credential_metadata(&self, credential_hash: &str, new_metadata_uri: &str) -> Result<()>;
}

// Solana 存储实现
pub struct SolanaStorage {
    client: SolanaClient,
    registry_pubkey: Pubkey,
}

impl SolanaStorage {
    pub fn new(rpc_url: &str, payer: Keypair, registry_pubkey: Pubkey) -> Result<Self> {
        let client = SolanaClient::new(rpc_url, payer)?;
        
        Ok(Self {
            client,
            registry_pubkey,
        })
    }
    
    pub fn initialize_registry(&self, registry_keypair: &Keypair) -> Result<String> {
        self.client.initialize_registry(registry_keypair)
    }
}

#[async_trait::async_trait]
impl Storage for SolanaStorage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    async fn store_credential(&self, credential_hash: &str, metadata_uri: &str) -> Result<String> {
        // 当前时间戳
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        // 使用默认凭证类型和发行方
        let credential_type = "education";
        let issuer = "AdProof";
        
        self.client.register_credential(
            &self.registry_pubkey,
            credential_hash,
            credential_type,
            issuer,
            now,
            metadata_uri,
        )
    }
    
    async fn verify_credential(&self, credential_hash: &str) -> Result<bool> {
        self.client.verify_credential(&self.registry_pubkey, credential_hash)
    }
    
    async fn get_credential_metadata(&self, credential_hash: &str) -> Result<Option<String>> {
        let result = self.client.get_credential_data(&self.registry_pubkey, credential_hash)?;
        
        Ok(result.map(|credential| credential.metadata_uri))
    }
    
    async fn revoke_credential(&self, credential_hash: &str) -> Result<()> {
        self.client.revoke_credential(&self.registry_pubkey, credential_hash)?;
        Ok(())
    }
    
    async fn update_credential_metadata(&self, credential_hash: &str, new_metadata_uri: &str) -> Result<()> {
        self.client.update_metadata(&self.registry_pubkey, credential_hash, new_metadata_uri)?;
        Ok(())
    }
}

// 存储管理器
pub struct StorageManager {
    storage: Option<Arc<dyn Storage>>,
}

impl StorageManager {
    pub fn new() -> Self {
        Self { storage: None }
    }
    
    // 获取存储实例，如果不存在则返回错误
    pub fn get_storage(&self) -> Result<Arc<dyn Storage>> {
        self.storage.clone().ok_or_else(|| anyhow!("存储未初始化"))
    }
    
    // 初始化Solana存储
    pub fn init_solana_storage(
        &mut self, 
        rpc_url: &str, 
        payer: Keypair, 
        registry_pubkey: Pubkey
    ) -> Result<()> {
        let storage = SolanaStorage::new(rpc_url, payer, registry_pubkey)?;
        self.storage = Some(Arc::new(storage));
        Ok(())
    }
    
    // 为测试设置自定义存储
    #[cfg(test)]
    pub fn set_storage(&mut self, storage: Arc<dyn Storage>) {
        self.storage = Some(storage);
    }
    
    // 清除存储实例
    pub fn clear_storage(&mut self) {
        self.storage = None;
    }
    
    // 检查存储是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.storage.is_some()
    }
}

// 提供一个默认实例以方便在应用程序中使用
thread_local! {
    pub static STORAGE_MANAGER: std::cell::RefCell<StorageManager> = std::cell::RefCell::new(StorageManager::new());
}

// 提供一个简单的访问函数
pub fn with_storage<F, R>(f: F) -> Result<R>
where
    F: FnOnce(Arc<dyn Storage>) -> Result<R>,
{
    STORAGE_MANAGER.with(|manager| {
        let manager = manager.borrow();
        let storage = manager.get_storage()?;
        f(storage)
    })
}

// 提供一个异步版本的访问函数
pub async fn with_storage_async<F, Fut, R>(f: F) -> Result<R>
where
    F: FnOnce(Arc<dyn Storage>) -> Fut + Send,
    Fut: std::future::Future<Output = Result<R>> + Send,
    R: Send,
{
    let storage = STORAGE_MANAGER.with(|manager| {
        let manager = manager.borrow();
        manager.get_storage()
    })?;
    
    f(storage).await
}

// 提供一个初始化函数
pub fn init_storage(rpc_url: &str, payer: Keypair, registry_pubkey: Pubkey) -> Result<()> {
    STORAGE_MANAGER.with(|manager| {
        let mut manager = manager.borrow_mut();
        manager.init_solana_storage(rpc_url, payer, registry_pubkey)
    })
}

// 确保存储已初始化
async fn ensure_storage_initialized(
    rpc_url: &str, 
    keypair_base58: &str, 
    registry_pubkey_base58: &str
) -> Result<()> {
    use solana_sdk::{signature::Keypair, pubkey::Pubkey};
    use std::str::FromStr;
    use bs58;
    
    // 检查存储是否已初始化
    let initialized = STORAGE_MANAGER.with(|manager| {
        manager.borrow().is_initialized()
    });
    
    if !initialized {
        log::info!("初始化存储管理器");
        
        // 为闭包创建变量的副本
        let keypair_base58 = keypair_base58.to_string();
        let registry_pubkey_base58 = registry_pubkey_base58.to_string();
        
        // 在异步块内处理可能阻塞的操作
        let base58_decoded = tokio::task::spawn_blocking(move || {
            // 直接解码Base58格式的密钥
            log::info!("使用Base58格式解析密钥");
            bs58::decode(&keypair_base58)
                .into_vec()
                .map_err(|e| anyhow::anyhow!("解析Base58编码的密钥失败: {}", e))
        }).await??;
        
        if base58_decoded.len() != 64 {
            return Err(anyhow::anyhow!("密钥字节长度错误: {}，预期64字节", base58_decoded.len()));
        }
        
        // 创建密钥对 - 这个操作也可能是计算密集型的
        let payer = tokio::task::spawn_blocking(move || {
            solana_sdk::signature::Keypair::from_bytes(&base58_decoded)
                .map_err(|e| anyhow::anyhow!("从字节创建密钥对失败: {}", e))
        }).await??;
        
        log::info!("成功创建密钥对，公钥: {}", payer.pubkey());
        
        // 注意：我们使用提供的registry_pubkey_base58作为现有注册表公钥
        // 而不是创建新的。如果需要创建一个新的注册表，应该由用户显式请求。
        
        // 解析注册表公钥
        let registry = Pubkey::from_str(&registry_pubkey_base58)
            .map_err(|e| anyhow::anyhow!("解析公钥失败: {}", e))?;
        
        // 初始化存储 - 这可能涉及网络操作，应该已经是异步的
        init_storage(rpc_url, payer, registry)?;
    }
    
    Ok(())
}

// 提供一个函数来简化凭证存储操作
pub async fn store_credential_on_chain(
    credential_hash: String, 
    metadata_uri: String,
    rpc_url: String,
    keypair_base58: String,
    registry_pubkey_base58: String
) -> Result<String> {
    // 在使用之前确保存储已初始化
    ensure_storage_initialized(&rpc_url, &keypair_base58, &registry_pubkey_base58).await?;
    
    // 使用已初始化的存储
    with_storage_async(|storage| async move {
        storage.store_credential(&credential_hash, &metadata_uri).await
    }).await
}

// 提供一个函数来简化凭证验证操作
pub async fn verify_credential_on_chain(
    credential_hash: String,
    rpc_url: String,
    keypair_base58: String,
    registry_pubkey_base58: String
) -> Result<bool> {
    // 在使用之前确保存储已初始化
    ensure_storage_initialized(&rpc_url, &keypair_base58, &registry_pubkey_base58).await?;
    
    // 使用已初始化的存储
    with_storage_async(|storage| async move {
        storage.verify_credential(&credential_hash).await
    }).await
}

// 提供一个函数来简化获取凭证元数据操作
pub async fn get_credential_metadata_from_chain(
    credential_hash: String,
    rpc_url: String,
    keypair_base58: String,
    registry_pubkey_base58: String
) -> Result<Option<String>> {
    // 在使用之前确保存储已初始化
    ensure_storage_initialized(&rpc_url, &keypair_base58, &registry_pubkey_base58).await?;
    
    // 使用已初始化的存储
    with_storage_async(|storage| async move {
        storage.get_credential_metadata(&credential_hash).await
    }).await
}

// 提供一个函数来简化撤销凭证操作
pub async fn revoke_credential_on_chain(
    credential_hash: String,
    rpc_url: String,
    keypair_base58: String,
    registry_pubkey_base58: String
) -> Result<()> {
    // 在使用之前确保存储已初始化
    ensure_storage_initialized(&rpc_url, &keypair_base58, &registry_pubkey_base58).await?;
    
    // 使用已初始化的存储
    with_storage_async(|storage| async move {
        storage.revoke_credential(&credential_hash).await
    }).await
}

// 提供一个函数来简化更新凭证元数据操作
pub async fn update_credential_metadata_on_chain(
    credential_hash: String, 
    new_metadata_uri: String,
    rpc_url: String,
    keypair_base58: String,
    registry_pubkey_base58: String
) -> Result<()> {
    // 在使用之前确保存储已初始化
    ensure_storage_initialized(&rpc_url, &keypair_base58, &registry_pubkey_base58).await?;
    
    // 使用已初始化的存储
    with_storage_async(|storage| async move {
        storage.update_credential_metadata(&credential_hash, &new_metadata_uri).await
    }).await
}

// 检查并初始化注册表（如果需要）
pub async fn initialize_registry_if_needed(
    rpc_url: String,
    keypair_base58: String,
    registry_pubkey_base58: String
) -> Result<()> {
    log::info!("检查并初始化Solana链上注册表...");
    
    // 确保存储已初始化
    ensure_storage_initialized(&rpc_url, &keypair_base58, &registry_pubkey_base58).await?;
    
    // 检查注册表是否已初始化
    let registry_pubkey = solana_sdk::pubkey::Pubkey::from_str(&registry_pubkey_base58)?;
    
    let initialized = with_storage_async(|storage| async move {
        // 尝试获取账户信息
        let storage_ref = storage.clone();
        let solana_storage = storage_ref.as_any().downcast_ref::<SolanaStorage>()
            .ok_or_else(|| anyhow!("无法获取SolanaStorage实例"))?;
            
        // 尝试获取账户信息，忽略错误（账户可能不存在）
        match solana_storage.client.rpc_client.get_account(&registry_pubkey) {
            Ok(account) => {
                // 账户存在且有数据，认为已初始化
                log::info!("注册表账户数据长度: {}", account.data.len());
                Ok(account.data.len() > 0)
            },
            Err(e) => {
                // 账户不存在，需要初始化
                log::warn!("获取注册表账户失败: {}，可能需要初始化", e);
                Ok(false)
            }
        }
    }).await?;
    
    if !initialized {
        log::info!("注册表未初始化，正在创建...");
        
        // 解析Base58密钥对
        let keypair_data = tokio::task::spawn_blocking(move || {
            bs58::decode(&keypair_base58)
                .into_vec()
                .map_err(|e| anyhow::anyhow!("解析Base58编码的密钥失败: {}", e))
        }).await??;
        
        if keypair_data.len() != 64 {
            return Err(anyhow::anyhow!("密钥字节长度错误: {}，预期64字节", keypair_data.len()));
        }
        
        // 创建密钥对
        let payer = tokio::task::spawn_blocking(move || {
            solana_sdk::signature::Keypair::from_bytes(&keypair_data)
                .map_err(|e| anyhow::anyhow!("从字节创建密钥对失败: {}", e))
        }).await??;
        
        log::info!("成功创建密钥对，公钥: {}", payer.pubkey());
        
        // 注意：我们使用提供的registry_pubkey_base58作为现有注册表公钥
        // 而不是创建新的。如果需要创建一个新的注册表，应该由用户显式请求。
        
        // 解析注册表公钥
        let registry = Pubkey::from_str(&registry_pubkey_base58)?;
        log::info!("使用现有注册表公钥: {}", registry);
        
        // 创建与注册表公钥匹配的密钥对（如果可能的话）
        // 注意：在实际情况下，我们可能无法恢复私钥，除非有完整的密钥对信息
        // 这里假设registry_pubkey_base58包含完整的密钥对信息
        
        // 初始化注册表
        with_storage_async(|storage| async move {
            let storage_ref = storage.clone();
            let solana_storage = storage_ref.as_any().downcast_ref::<SolanaStorage>()
                .ok_or_else(|| anyhow!("无法获取SolanaStorage实例"))?;
            
            // 这里我们假设用户已经创建了注册表账户，我们只需要初始化它
            // 如果需要创建一个新的注册表，应该由用户显式请求
            
            // 尝试重新获取注册表账户，检查是否真的需要初始化
            match solana_storage.client.rpc_client.get_account(&registry_pubkey) {
                Ok(account) => {
                    if account.data.len() > 0 {
                        log::info!("注册表账户已存在且有数据，长度: {}", account.data.len());
                        return Ok(());
                    }
                    
                    log::info!("注册表账户已存在但没有数据，尝试初始化");
                    // 账户已存在但为空，需要初始化
                    // 这种情况下，我们需要注册表密钥对来签名，但我们可能没有私钥
                    // 可以考虑使用其他方法来初始化注册表，比如创建新的注册表
                    
                    // 创建一个新的注册表账户作为替代
                    let new_registry_keypair = solana_sdk::signature::Keypair::new();
                    log::info!("创建新的注册表公钥: {}", new_registry_keypair.pubkey());
                    
                    let result = solana_storage.client.initialize_registry(&new_registry_keypair);
                    if let Ok(signature) = &result {
                        log::info!("新注册表初始化成功，交易签名: {}", signature);
                        log::info!("新的注册表公钥: {}，请更新配置文件", new_registry_keypair.pubkey());
                    } else if let Err(e) = &result {
                        log::error!("注册表初始化失败: {}", e);
                    }
                    result.map(|_| ())
                },
                Err(e) => {
                    log::warn!("无法获取注册表账户: {}，尝试创建新的注册表", e);
                    
                    // 创建一个新的注册表
                    let new_registry_keypair = solana_sdk::signature::Keypair::new();
                    log::info!("创建新的注册表公钥: {}", new_registry_keypair.pubkey());
                    
                    let result = solana_storage.client.initialize_registry(&new_registry_keypair);
                    if let Ok(signature) = &result {
                        log::info!("新注册表初始化成功，交易签名: {}", signature);
                        log::info!("新的注册表公钥: {}，请更新配置文件", new_registry_keypair.pubkey());
                    } else if let Err(e) = &result {
                        log::error!("注册表初始化失败: {}", e);
                    }
                    result.map(|_| ())
                }
            }
        }).await?;
        
        log::info!("注册表初始化检查完成");
    } else {
        log::info!("注册表已初始化，跳过");
    }
    
    Ok(())
}

// 从文件读取密钥对
fn read_keypair_file(path: &str) -> Result<Keypair> {
    use std::path::Path;

    log::info!("尝试读取密钥文件: {:?}", path);


    let path = Path::new(path);
    
    // 检查文件是否存在
    if !path.exists() {
        return Err(anyhow::anyhow!("密钥文件不存在: {:?}", path.to_str()));
    }
    
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // 尝试解析为JSON数组格式
    if contents.trim().starts_with('[') && contents.trim().ends_with(']') {
        log::info!("检测到JSON数组格式的密钥文件");
        let bytes: Vec<u8> = serde_json::from_str(&contents)?;
        return Keypair::from_bytes(&bytes)
            .map_err(|e| anyhow::anyhow!("从字节创建密钥对失败: {}", e));
    }
    
    // 标准Solana密钥文件通常是JSON数组，不是Keypair对象
    // 尝试直接用bs58解码内容
    log::info!("尝试作为Base58编码解析");
    let content_str = contents.trim();
    match bs58::decode(content_str).into_vec() {
        Ok(bytes) => {
            log::info!("成功解析为Base58编码");
            Keypair::from_bytes(&bytes)
                .map_err(|e| anyhow::anyhow!("从字节创建密钥对失败: {}", e))
        },
        Err(e) => {
            log::warn!("Base58解码失败: {}", e);
            
            // 最后尝试解析为自定义JSON格式
            log::info!("尝试解析为自定义JSON格式");
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&contents) {
                if let Some(arr) = v.as_array() {
                    log::info!("解析为JSON值数组成功");
                    let bytes: Vec<u8> = arr.iter()
                        .filter_map(|v| v.as_u64().map(|n| n as u8))
                        .collect();
                    
                    if !bytes.is_empty() {
                        log::info!("成功提取字节数组，长度: {}", bytes.len());
                        return Keypair::from_bytes(&bytes)
                            .map_err(|e| anyhow::anyhow!("从字节创建密钥对失败: {}", e));
                    }
                }
            }
            
            Err(anyhow::anyhow!("无法从文件内容解析密钥对"))
        }
    }
}
