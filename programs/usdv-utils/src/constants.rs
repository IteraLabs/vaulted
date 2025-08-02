//! Program-wide constants and configuration values

use solana_sdk::pubkey::Pubkey;

/// USDC token decimals (6 decimals)
pub const USDC_DECIMALS: u8 = 6;

/// USDv token decimals (6 decimals to match USDC)
pub const USDV_DECIMALS: u8 = 6;

/// Maximum total supply of USDv tokens (1 billion with 6 decimals)
pub const MAX_TOTAL_SUPPLY: u64 = 1_000_000_000 * 10_u64.pow(USDV_DECIMALS as u32);

/// Minimum deposit amount (0.01 USDC)
pub const MIN_DEPOSIT_AMOUNT: u64 = 10_000; // 0.01 * 10^6

/// Maximum deposit amount (1 million USDC)
pub const MAX_DEPOSIT_AMOUNT: u64 = 1_000_000 * 10_u64.pow(USDC_DECIMALS as u32);

/// Seed for vault authority PDA
pub const VAULT_AUTHORITY_SEED: &[u8] = b"vault_authority";

/// Seed for program state PDA
pub const PROGRAM_STATE_SEED: &[u8] = b"program_state";

/// Maximum number of retries for RPC calls
pub const MAX_RPC_RETRIES: u32 = 3;

/// Default timeout for transactions (30 seconds)
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

/// Program version
pub const PROGRAM_VERSION: (u8, u8, u8) = (0, 1, 0);

/// Mainnet USDC mint address
pub const MAINNET_USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

/// Devnet USDC mint address  
pub const DEVNET_USDC_MINT: &str = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";

/// Account space calculations
pub mod account_space {
    /// Space required for ProgramState account (in bytes)
    pub const PROGRAM_STATE: usize = 8 + // discriminator
        1 +  // is_initialized
        32 + // admin
        32 + // usdc_mint
        32 + // usdv_mint
        32 + // vault_authority
        1 +  // vault_bump
        8 +  // total_usdv_supply
        8;   // total_usdc_deposits
}

/// Fee calculations (for future use)
pub mod fees {
    /// Basis points for potential fees (currently 0)
    pub const FEE_BASIS_POINTS: u16 = 0;
    
    /// Maximum fee basis points (1% = 100 basis points)
    pub const MAX_FEE_BASIS_POINTS: u16 = 100;
}

