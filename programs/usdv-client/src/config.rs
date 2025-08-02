
pub use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Configuration for USDv stablecoin program
#[derive(Debug, Clone)]
pub struct USDvConfig {
    /// Program ID for the USDv program
    pub program_id: Pubkey,
    /// Program state account public key
    pub program_state: Pubkey,
    /// USDC mint public key
    pub usdc_mint: Pubkey,
    /// USDv mint public key
    pub usdv_mint: Pubkey,
    /// Network cluster name
    pub cluster: String,
}

impl USDvConfig {
    /// Create configuration for mainnet deployment
    pub fn mainnet() -> Self {
        Self {
            program_id: Pubkey::from_str("USDvCoinProgram11111111111111111111111111111").unwrap(),
            program_state: Pubkey::from_str("USDvState1111111111111111111111111111111111").unwrap(),
            usdc_mint: Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(), // Real USDC mint
            usdv_mint: Pubkey::from_str("USDvMint11111111111111111111111111111111111").unwrap(),
            cluster: "mainnet-beta".to_string(),
        }
    }

    /// Create configuration for devnet deployment
    pub fn devnet() -> Self {
        Self {
            program_id: Pubkey::from_str("USDvCoinProgram11111111111111111111111111111").unwrap(),
            program_state: Pubkey::from_str("USDvState1111111111111111111111111111111111").unwrap(),
            usdc_mint: Pubkey::from_str("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU").unwrap(), // Devnet USDC
            usdv_mint: Pubkey::from_str("USDvMint11111111111111111111111111111111111").unwrap(),
            cluster: "devnet".to_string(),
        }
    }

    /// Create configuration for local testing
    pub fn localnet() -> Self {
        Self {
            program_id: Pubkey::from_str("USDvCoinProgram11111111111111111111111111111").unwrap(),
            program_state: Pubkey::from_str("USDvState1111111111111111111111111111111111").unwrap(),
            usdc_mint: Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(), // Wrapped SOL for testing
            usdv_mint: Pubkey::from_str("USDvMint11111111111111111111111111111111111").unwrap(),
            cluster: "localnet".to_string(),
        }
    }

    /// Create custom configuration
    pub fn custom(
        program_id: Pubkey,
        program_state: Pubkey,
        usdc_mint: Pubkey,
        usdv_mint: Pubkey,
        cluster: String,
    ) -> Self {
        Self {
            program_id,
            program_state,
            usdc_mint,
            usdv_mint,
            cluster,
        }
    }

    /// Get RPC URL for the configured cluster
    pub fn rpc_url(&self) -> String {
        match self.cluster.as_str() {
            "mainnet-beta" => "https://api.mainnet-beta.solana.com".to_string(),
            "devnet" => "https://api.devnet.solana.com".to_string(),
            "testnet" => "https://api.testnet.solana.com".to_string(),
            "localnet" => "http://localhost:8899".to_string(),
            _ => self.cluster.clone(),
        }
    }
}

