//! SPL Token 管理模块
//! 
//! 提供 SPL Token 的余额查询、转账、关联账户创建等功能

use async_trait::async_trait;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{
    instruction::{transfer, TransferCheckedParams},
    state::Account as TokenAccount,
};
use std::sync::Arc;

use crate::error::{Result, SolanaError};

/// Token 管理器
pub struct TokenManager {
    rpc_client: Arc<RpcClient>,
}

impl TokenManager {
    /// 创建新的 Token 管理器
    pub fn new(rpc_url: &str) -> Self {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        ));
        
        Self { rpc_client }
    }
    
    /// 从配置创建 Token 管理器
    pub fn from_config(config: &crate::config::SolanaConfig) -> Self {
        Self::new(&config.rpc_url)
    }
    
    /// 获取 Token 余额
    pub async fn get_token_balance(
        &self,
        token_account: &Pubkey,
    ) -> Result<u64> {
        let account_data = self.rpc_client
            .get_account_data(token_account)
            .map_err(|e| SolanaError::AccountNotFound(e.to_string()))?;
        
        let token_account = TokenAccount::unpack(&account_data)
            .map_err(|e| SolanaError::TokenAccountNotFound(e.to_string()))?;
        
        Ok(token_account.amount)
    }
    
    /// 获取关联 Token 账户地址
    pub fn get_associated_token_address(
        &self,
        wallet: &Pubkey,
        token_mint: &Pubkey,
    ) -> Pubkey {
        get_associated_token_address(wallet, token_mint)
    }
    
    /// 创建关联 Token 账户（如果不存在）
    pub async fn create_associated_token_account_if_needed(
        &self,
        payer: &Keypair,
        wallet: &Pubkey,
        token_mint: &Pubkey,
    ) -> Result<Pubkey> {
        let associated_token_account = self.get_associated_token_address(wallet, token_mint);
        
        // 检查账户是否已存在
        match self.rpc_client.get_account(&associated_token_account) {
            Ok(_) => {
                tracing::debug!("Associated token account already exists: {}", associated_token_account);
                return Ok(associated_token_account);
            }
            Err(_) => {
                // 账户不存在，需要创建
                tracing::info!("Creating associated token account for wallet: {}, mint: {}", wallet, token_mint);
            }
        }
        
        // 创建关联 Token 账户的指令
        let create_ix = spl_associated_token_account::instruction::create_associated_token_account(
            &payer.pubkey(),
            wallet,
            token_mint,
            &spl_token::id(),
        );
        
        let mut transaction = Transaction::new_with_payer(
            &[create_ix],
            Some(&payer.pubkey()),
        );
        
        let recent_blockhash = self.rpc_client
            .get_latest_blockhash()
            .map_err(|e| SolanaError::RpcError(e.to_string()))?;
        
        transaction.sign(&[payer], recent_blockhash);
        
        let signature = self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| SolanaError::SendError(e.to_string()))?;
        
        tracing::info!("Created associated token account: {} with signature: {}", associated_token_account, signature);
        
        Ok(associated_token_account)
    }
    
    /// 转账 SPL Token
    pub async fn transfer_token(
        &self,
        from_keypair: &Keypair,
        from_token_account: &Pubkey,
        to_token_account: &Pubkey,
        token_mint: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> Result<String> {
        // 检查发送方余额
        let balance = self.get_token_balance(from_token_account).await?;
        if balance < amount {
            return Err(SolanaError::InsufficientBalance(format!(
                "Insufficient token balance: have {}, need {}",
                balance, amount
            )));
        }
        
        // 创建转账指令
        let transfer_ix = transfer(
            &spl_token::id(),
            from_token_account,
            to_token_account,
            &from_keypair.pubkey(),
            &[],
            amount,
        )?;
        
        let mut transaction = Transaction::new_with_payer(
            &[transfer_ix],
            Some(&from_keypair.pubkey()),
        );
        
        let recent_blockhash = self.rpc_client
            .get_latest_blockhash()
            .map_err(|e| SolanaError::RpcError(e.to_string()))?;
        
        transaction.sign(&[from_keypair], recent_blockhash);
        
        let signature = self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| SolanaError::SendError(e.to_string()))?;
        
        Ok(signature.to_string())
    }
    
    /// 转账 Token 到外部钱包
    pub async fn transfer_token_to_external(
        &self,
        from_keypair: &Keypair,
        to_wallet: &Pubkey,
        token_mint: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> Result<String> {
        let from_token_account = self.get_associated_token_address(&from_keypair.pubkey(), token_mint);
        let to_token_account = self.get_associated_token_address(to_wallet, token_mint);
        
        // 确保接收方的关联 Token 账户存在
        self.create_associated_token_account_if_needed(from_keypair, to_wallet, token_mint).await?;
        
        // 执行转账
        self.transfer_token(
            from_keypair,
            &from_token_account,
            &to_token_account,
            token_mint,
            amount,
            decimals,
        ).await
    }
    
    /// 获取 Token 元数据（通过 Token List 或自定义配置）
    pub async fn get_token_metadata(
        &self,
        token_mint: &Pubkey,
    ) -> Result<TokenMetadata> {
        // 这里可以集成 Token List API 或从配置中获取
        // 目前返回基本元数据
        Ok(TokenMetadata {
            mint: *token_mint,
            symbol: "UNKNOWN".to_string(),
            name: "Unknown Token".to_string(),
            decimals: 9, // Solana 默认小数位数
            logo_uri: None,
        })
    }
}

/// Token 元数据
#[derive(Debug, Clone)]
pub struct TokenMetadata {
    /// Token mint 地址
    pub mint: Pubkey,
    
    /// Token 符号
    pub symbol: String,
    
    /// Token 名称
    pub name: String,
    
    /// 小数位数
    pub decimals: u8,
    
    /// Logo URI
    pub logo_uri: Option<String>,
}

/// Token 存储 trait
#[async_trait]
pub trait TokenStorage: Send + Sync {
    /// 保存用户 Token 信息
    async fn save_user_token(
        &self,
        user_id: &str,
        token_mint: &Pubkey,
        token_account: &Pubkey,
    ) -> Result<()>;
    
    /// 获取用户 Token 账户
    async fn get_user_token_account(
        &self,
        user_id: &str,
        token_mint: &Pubkey,
    ) -> Result<Option<Pubkey>>;
}
