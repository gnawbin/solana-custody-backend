//! Solana SPL Token 集成模块
//! 
//! 提供以下功能：
//! 1. 钱包创建和管理（系统托管）
//! 2. SPL Token 余额查询和转账
//! 3. 稳定币购买和代币转换
//! 4. 代币转账到外部钱包

pub mod error;
pub mod wallet;
pub mod token;
pub mod swap;
pub mod config;

pub use error::SolanaError;
pub use wallet::WalletManager;
pub use token::TokenManager;
pub use swap::SwapManager;
pub use config::SolanaConfig;

/// 重新导出常用的Solana类型
pub use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};
pub use spl_token::instruction as token_instruction;
pub use spl_associated_token_account::instruction as associated_token_instruction;
