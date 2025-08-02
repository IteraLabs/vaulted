//! Error definitions for the USDv stablecoin program
use thiserror::Error;

// For client-side errors (off-chain)
#[derive(Error, Debug)]
pub enum USDvClientError {
    #[error("Solana client error: {0}")]
    SolanaClientError(#[from] solana_client::client_error::ClientError),

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u64, available: u64 },

}
