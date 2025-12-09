// src/config.rs
#[derive(serde::Deserialize, Clone)]
pub struct Config{
 pub environment: String,
    pub rpc: RpcConfig,
    pub database_url: String,
    pub encryption_key: String,
    pub treasury_secret_key_base58: String,
    pub signer_service_url: String,
}
#[derive(serde::Deserialize, Clone)]
pub struct RpcConfig {
    pub devnet: String,
    pub testnet: String,
    pub mainnet: String,
}

impl Config {
    pub fn rpc_url(&self) -> String {
        match self.environment.as_str() {
            "devnet" => self.rpc.devnet.clone(),
            "testnet" => self.rpc.testnet.clone(),
            _ => self.rpc.mainnet.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_url_devnet() {
        let config = Config {
            environment: "devnet".to_string(),
            rpc: RpcConfig {
                devnet: "https://api.devnet.solana.com".to_string(),
                testnet: "https://api.testnet.solana.com".to_string(),
                mainnet: "https://api.mainnet.solana.com".to_string(),
            },
            database_url: "".to_string(),
            encryption_key: "".to_string(),
            treasury_secret_key_base58: "".to_string(),
            signer_service_url: "".to_string(),
        };
        assert_eq!(config.rpc_url(), "https://api.devnet.solana.com");
    }

    #[test]
    fn test_rpc_url_testnet() {
        let config = Config {
            environment: "testnet".to_string(),
            rpc: RpcConfig {
                devnet: "https://api.devnet.solana.com".to_string(),
                testnet: "https://api.testnet.solana.com".to_string(),
                mainnet: "https://api.mainnet.solana.com".to_string(),
            },
            database_url: "".to_string(),
            encryption_key: "".to_string(),
            treasury_secret_key_base58: "".to_string(),
            signer_service_url: "".to_string(),
        };
        assert_eq!(config.rpc_url(), "https://api.testnet.solana.com");
    }

    #[test]
    fn test_rpc_url_mainnet() {
        let config = Config {
            environment: "mainnet".to_string(),
            rpc: RpcConfig {
                devnet: "https://api.devnet.solana.com".to_string(),
                testnet: "https://api.testnet.solana.com".to_string(),
                mainnet: "https://api.mainnet.solana.com".to_string(),
            },
            database_url: "".to_string(),
            encryption_key: "".to_string(),
            treasury_secret_key_base58: "".to_string(),
            signer_service_url: "".to_string(),
        };
        assert_eq!(config.rpc_url(), "https://api.mainnet.solana.com");
    }

    #[test]
    fn test_rpc_url_default_to_mainnet() {
        let config = Config {
            environment: "unknown".to_string(),
            rpc: RpcConfig {
                devnet: "https://api.devnet.solana.com".to_string(),
                testnet: "https://api.testnet.solana.com".to_string(),
                mainnet: "https://api.mainnet.solana.com".to_string(),
            },
            database_url: "".to_string(),
            encryption_key: "".to_string(),
            treasury_secret_key_base58: "".to_string(),
            signer_service_url: "".to_string(),
        };
        assert_eq!(config.rpc_url(), "https://api.mainnet.solana.com");
    }
}
