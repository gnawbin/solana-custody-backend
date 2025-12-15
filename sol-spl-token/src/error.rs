//! Solana 集成错误类型

use thiserror::Error;

/// Solana 集成错误
#[derive(Error, Debug)]
pub enum SolanaError {
    /// RPC 连接错误
    #[error("RPC connection error: {0}")]
    RpcError(String),

    /// 交易签名错误
    #[error("Transaction signing error: {0}")]
    SignError(String),

    /// 交易发送错误
    #[error("Transaction send error: {0}")]
    SendError(String),

    /// 交易确认错误
    #[error("Transaction confirmation error: {0}")]
    ConfirmationError(String),

    /// 账户不存在
    #[error("Account not found: {0}")]
    AccountNotFound(String),

    /// 余额不足
    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),

    /// Token 账户不存在
    #[error("Token account not found: {0}")]
    TokenAccountNotFound(String),

    /// 配置错误
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// 钱包创建错误
    #[error("Wallet creation error: {0}")]
    WalletCreationError(String),

    /// 代币转账错误
    #[error("Token transfer error: {0}")]
    TokenTransferError(String),

    /// 交换错误
    #[error("Swap error: {0}")]
    SwapError(String),

    /// 序列化/反序列化错误
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// 其他错误
    #[error("Other error: {0}")]
    Other(String),
}

impl From<solana_client::client_error::ClientError> for SolanaError {
    fn from(err: solana_client::client_error::ClientError) -> Self {
        SolanaError::RpcError(err.to_string())
    }
}

impl From<solana_sdk::signature::SignerError> for SolanaError {
    fn from(err: solana_sdk::signature::SignerError) -> Self {
        SolanaError::SignError(err.to_string())
    }
}

impl From<serde_json::Error> for SolanaError {
    fn from(err: serde_json::Error) -> Self {
        SolanaError::SerializationError(err.to_string())
    }
}

impl From<std::io::Error> for SolanaError {
    fn from(err: std::io::Error) -> Self {
        SolanaError::Other(err.to_string())
    }
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, SolanaError>;
