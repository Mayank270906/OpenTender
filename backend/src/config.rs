use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub contract_id: String,
    pub network: String,
    pub rpc_url: String,
    pub encryption_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
            contract_id: env::var("CONTRACT_ID")
                .expect("CONTRACT_ID must be set"),
            network: env::var("NETWORK")
                .unwrap_or_else(|_| "testnet".to_string()),
            rpc_url: env::var("RPC_URL")
                .unwrap_or_else(|_| "https://soroban-testnet.stellar.org".to_string()),
            encryption_key: env::var("ENCRYPTION_KEY")
                .unwrap_or_else(|_| "default-32-char-encryption-key!".to_string()),
        }
    }
}