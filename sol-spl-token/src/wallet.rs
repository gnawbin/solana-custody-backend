//! 钱包管理模块
//! 
//! 提供系统托管钱包的创建和管理功能

use async_trait::async_trait;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::sync::Arc;

use crate::error::{Result, SolanaError};

/// 钱包管理器
pub struct WalletManager {
    rpc_client: Arc<RpcClient>,
    system_keypair: Keypair,
}

impl WalletManager {
    /// 创建新的钱包管理器
    pub fn new(rpc_url: &str, system_keypair: Keypair) -> Self {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        ));
        
        Self {
            rpc_client,
            system_keypair,
        }
    }
    
    /// 从配置创建钱包管理器
    pub fn from_config(config: &crate::config::SolanaConfig) -> Result<Self> {
        let system_keypair = Keypair::from_base58_string(&config.system_wallet_private_key)
            .map_err(|e| SolanaError::ConfigError(e.to_string()))?;
        
        Ok(Self::new(&config.rpc_url, system_keypair))
    }
    
    /// 创建新用户钱包（系统托管）
    /// 
    /// 为每个用户创建一个新的密钥对，并由系统钱包提供初始资金
    pub async fn create_user_wallet(&self, initial_lamports: u64) -> Result<UserWallet> {
        // 生成新的密钥对
        let user_keypair = Keypair::new();
        let user_pubkey = user_keypair.pubkey();
        
        // 创建账户交易
        let create_account_ix = system_instruction::create_account(
            &self.system_keypair.pubkey(),
            &user_pubkey,
            initial_lamports,
            0, // 空间大小（系统账户）
            &solana_sdk::system_program::id(),
        );
        
        let mut transaction = Transaction::new_with_payer(
            &[create_account_ix],
            Some(&self.system_keypair.pubkey()),
        );
        
        // 获取最近区块哈希
        let recent_blockhash = self.rpc_client
            .get_latest_blockhash()
            .map_err(|e| SolanaError::RpcError(e.to_string()))?;
        
        transaction.sign(&[&self.system_keypair, &user_keypair], recent_blockhash);
        
        // 发送交易
        let signature = self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| SolanaError::SendError(e.to_string()))?;
        
        tracing::info!("Created user wallet: {} with signature: {}", user_pubkey, signature);
        
        Ok(UserWallet {
            keypair: user_keypair,
            pubkey: user_pubkey,
            created_signature: signature,
        })
    }
    
    /// 获取钱包余额
    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        self.rpc_client
            .get_balance(pubkey)
            .map_err(|e| SolanaError::RpcError(e.to_string()))
    }
    
    /// 转账 SOL
    pub async fn transfer_sol(
        &self,
        from_keypair: &Keypair,
        to_pubkey: &Pubkey,
        lamports: u64,
    ) -> Result<String> {
        let transfer_ix = system_instruction::transfer(
            &from_keypair.pubkey(),
            to_pubkey,
            lamports,
        );
        
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
    
    /// 获取系统钱包余额
    pub async fn get_system_balance(&self) -> Result<u64> {
        self.get_balance(&self.system_keypair.pubkey()).await
    }
}

/// 用户钱包信息
#[derive(Debug, Clone)]
pub struct UserWallet {
    /// 用户密钥对（系统托管存储）
    pub keypair: Keypair,
    
    /// 用户公钥
    pub pubkey: Pubkey,
    
    /// 创建交易的签名
    pub created_signature: String,
}

impl UserWallet {
    /// 获取钱包地址（base58编码）
    pub fn get_address(&self) -> String {
        self.pubkey.to_string()
    }
    
    /// 导出私钥（base58编码）
    pub fn export_private_key(&self) -> String {
        self.keypair.to_base58_string()
    }
}

/// 钱包存储 trait
#[async_trait]
pub trait WalletStorage: Send + Sync {
    /// 保存钱包
    async fn save_wallet(&self, user_id: &str, wallet: &UserWallet) -> Result<()>;
    
    /// 获取钱包
    async fn get_wallet(&self, user_id: &str) -> Result<Option<UserWallet>>;
    
    /// 删除钱包
    async fn delete_wallet(&self, user_id: &str) -> Result<()>;
}
