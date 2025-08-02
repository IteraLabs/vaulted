//! # USDv Stablecoin Client Library
//!
//! A comprehensive client library for the USDv stablecoin program on Solana.
//!
//! ## Features
//!
//! - **Easy Integration**: Simple API for common operations
//! - **Type Safety**: Rust type system ensures correctness
//! - **Async/Await**: Modern async programming support
//! - **Comprehensive Testing**: Full test coverage
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use usdv_client::{USDvClient, USDvConfig};
//! use solana_client::rpc_client::RpcClient;
//! use solana_sdk::signature::Keypair;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
//!     let payer = Keypair::new();
//!     
//!     let config = USDvConfig::devnet();
//!     let client = USDvClient::new(rpc_client, config);
//!     
//!     // Deposit 100 USDC and mint 100 USDv
//!     let signature = client.deposit_and_mint(&payer, 100_000_000).await?;
//!     println!("Transaction: {}", signature);
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod config;  
pub mod types;
pub use client::*;

/// Current version of the USDv client library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// USDv program ID on mainnet
pub const MAINNET_PROGRAM_ID: &str = "USDVCoinProgram11111111111111111111111111111";

/// USDv program ID on devnet  
pub const DEVNET_PROGRAM_ID: &str = "USDVCoinProgram11111111111111111111111111111";

