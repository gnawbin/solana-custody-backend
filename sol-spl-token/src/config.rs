//! Solana 配置模块

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// Solana 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    /// RPC 端点 URL
    pub rpc_url: String,
    
    /// WebSocket 端点 URL
    pub ws_url: String,
    
    /// 网络类型 (mainnet-beta, testnet, devnet, localhost)
    pub network: String,
    
    /// 系统钱包私钥（base58编码）
    pub system_wallet_private_key: String,
    
    /// 默认稳定币 mint 地址（如 USDC）
    pub default_stablecoin_mint: String,
    
    /// 目标代币 mint 地址
    pub target_token_mint: String,
    
    /// 交易确认超时（秒）
    pub confirmation_timeout_secs: u64,
    
    /// 最大重试次数
    pub max_retries: u32,
}

impl Default for SolanaConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://api.devnet.solana.com".to_string(),
            ws_url: "wss://api.devnet.solana.com".to_string(),
            network: "devnet".to_string(),
            system_wallet_private_key: "".to_string(),
            default_stablecoin_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(), // USDC
            target_token_mint: "".to_string(),
            confirmation_timeout_secs: 30,
            max_retries: 3,
        }
    }
}

impl SolanaConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self, crate::error::SolanaError> {
        let config = envy::prefixed("SOLANA_")
            .from_env::<Self>()
            .map_err(|e| crate::error::SolanaError::ConfigError(e.to_string()))?;
        
        Ok(config)
    }
    
    /// 获取系统钱包公钥
    pub fn get_system_wallet_pubkey(&self) -> Result<Pubkey, crate::error::SolanaError> {
        if self.system_wallet_private_key.is_empty() {
            return Err(crate::error::SolanaError::ConfigError(
                "System wallet private key is not set".to_string(),
            ));
        }
        
        let keypair = solana_sdk::signature::Keypair::from_base58_string(&self.system_wallet_private_key)
            .map_err(|e| crate::error::SolanaError::ConfigError(e.to_string()))?;
        
        Ok(keypair.pubkey())
    }
    
    /// 获取稳定币 mint 地址
    pub fn get_stablecoin_mint(&self) -> Result<Pubkey, crate::error::SolanaError> {
        Pubkey::from_str(&self.default_stablecoin_mint)
            .map_err(|e| crate::error::SolanaError::ConfigError(e.to_string()))
    }
    
    /// 获取目标代币 mint 地址
    pub fn get_target_token_mint(&self) -> Result<Pubkey, crate::error::SolanaError> {
        if self.target_token_mint.is_empty() {
            return Err(crate::error::SolanaError::ConfigError(
                "Target token mint is not set".to_string(),
            ));
        }
        
        Pubkey::from_str(&self.target_token_mint)
            .map_err(|e| crate::error::SolanaError::ConfigError(e.to_string()))
    }
}

use std::str::FromStr;
