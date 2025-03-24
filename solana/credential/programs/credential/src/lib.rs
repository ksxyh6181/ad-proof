use anchor_lang::prelude::*;

declare_id!("CGZST4ic7TB5Mr71LvCBPKV92kMqrSyzzWW6Sge4FqaV");

#[program]
pub mod credential {
    use super::*;

    /// 初始化凭证注册表
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.credential_count = 0;
        
        msg!("凭证注册表已初始化");
        Ok(())
    }

    /// 注册新凭证
    pub fn register_credential(
        ctx: Context<RegisterCredential>, 
        hash: String,
        credential_type: String,
        issuer: String,
        issue_date: i64,
        metadata_uri: String,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let credential = &mut ctx.accounts.credential;

        // 设置凭证数据
        credential.hash = hash;
        credential.credential_type = credential_type;
        credential.issuer = issuer;
        credential.issue_date = issue_date;
        credential.metadata_uri = metadata_uri;
        credential.owner = ctx.accounts.authority.key();
        credential.revoked = false;
        credential.registry = registry.key();
        
        // 保存种子值，用于后续查找
        credential.seeds = registry.credential_count.to_le_bytes();

        // 更新注册表计数
        registry.credential_count += 1;

        msg!("凭证已注册: {}", credential.hash);
        Ok(())
    }

    /// 验证凭证状态
    pub fn verify_credential(ctx: Context<VerifyCredential>) -> Result<()> {
        let credential = &ctx.accounts.credential;
        
        // 检查凭证是否已撤销
        if credential.revoked {
            return err!(ErrorCode::CredentialRevoked);
        }
        
        msg!("凭证有效: {}", credential.hash);
        Ok(())
    }

    /// 撤销凭证
    pub fn revoke_credential(ctx: Context<RevokeCredential>) -> Result<()> {
        let credential = &mut ctx.accounts.credential;
        
        // 检查凭证是否已撤销
        if credential.revoked {
            return err!(ErrorCode::CredentialRevoked);
        }
        
        // 设置撤销状态
        credential.revoked = true;
        
        msg!("凭证已撤销: {}", credential.hash);
        Ok(())
    }

    /// 更新凭证元数据
    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        new_metadata_uri: String
    ) -> Result<()> {
        let credential = &mut ctx.accounts.credential;
        
        // 更新元数据URI
        credential.metadata_uri = new_metadata_uri;
        
        msg!("凭证元数据已更新: {}", credential.hash);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Registry::LEN
    )]
    pub registry: Account<'info, Registry>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(hash: String, credential_type: String, issuer: String, issue_date: i64, metadata_uri: String)]
pub struct RegisterCredential<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    
    #[account(
        init,
        payer = authority,
        space = Credential::LEN,
        seeds = [
            b"credential", 
            registry.key().as_ref(), 
            &registry.credential_count.to_le_bytes()
        ],
        bump
    )]
    pub credential: Account<'info, Credential>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyCredential<'info> {
    pub registry: Account<'info, Registry>,
    
    #[account(
        seeds = [b"credential", registry.key().as_ref(), &credential.seeds],
        bump,
        constraint = credential.registry == registry.key(),
    )]
    pub credential: Account<'info, Credential>,
}

#[derive(Accounts)]
pub struct RevokeCredential<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    
    #[account(
        mut,
        seeds = [b"credential", registry.key().as_ref(), &credential.seeds],
        bump,
        constraint = credential.registry == registry.key(),
        constraint = !credential.revoked @ ErrorCode::CredentialRevoked,
        constraint = credential.owner == authority.key() @ ErrorCode::Unauthorized,
    )]
    pub credential: Account<'info, Credential>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    pub registry: Account<'info, Registry>,
    
    #[account(
        mut,
        seeds = [b"credential", registry.key().as_ref(), &credential.seeds],
        bump,
        constraint = credential.registry == registry.key(),
        constraint = !credential.revoked @ ErrorCode::CredentialRevoked,
        constraint = credential.owner == authority.key() @ ErrorCode::Unauthorized,
    )]
    pub credential: Account<'info, Credential>,
    
    pub authority: Signer<'info>,
}

#[account]
pub struct Registry {
    pub authority: Pubkey,       // 注册表管理员
    pub credential_count: u64,   // 凭证计数
}

impl Registry {
    pub const LEN: usize = 8 + // discriminator
                          32 + // authority
                          8;   // credential_count
}

#[account]
pub struct Credential {
    pub hash: String,            // 凭证哈希
    pub credential_type: String, // 凭证类型
    pub issuer: String,          // 发行者
    pub issue_date: i64,         // 发行日期
    pub metadata_uri: String,    // 元数据URI (IPFS或其他存储)
    pub owner: Pubkey,           // 拥有者
    pub revoked: bool,           // 是否已撤销
    pub registry: Pubkey,        // 注册表引用
    pub seeds: [u8; 8],          // 存储该凭证的种子
}

impl Credential {
    pub const LEN: usize = 8 +   // discriminator
                          64 +   // hash (最大长度)
                          32 +   // credential_type (最大长度)
                          64 +   // issuer (最大长度)
                          8 +    // issue_date
                          128 +  // metadata_uri (最大长度)
                          32 +   // owner
                          1 +    // revoked
                          32 +   // registry
                          8;     // seeds
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    
    #[msg("Credential has been revoked")]
    CredentialRevoked,
    
    #[msg("Credential not found")]
    CredentialNotFound,
}
