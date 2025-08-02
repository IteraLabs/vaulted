//! Type definitions for USDv client operations

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// Program state information returned by the client
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct ProgramStateInfo {
    /// Whether the program has been initialized
    pub is_initialized: bool,
    /// Admin public key
    pub admin: Pubkey,
    /// USDC mint address
    pub usdc_mint: Pubkey,
    /// USDv mint address
    pub usdv_mint: Pubkey,
    /// Vault authority PDA
    pub vault_authority: Pubkey,
    /// Vault authority bump seed
    pub vault_bump: u8,
    /// Total USDv tokens in circulation
    pub total_usdv_supply: u64,
    /// Total USDC deposited in vault
    pub total_usdc_deposits: u64,
}

impl ProgramStateInfo {
    /// Check if the 1:1 peg is maintained
    pub fn is_peg_maintained(&self) -> bool {
        self.total_usdv_supply == self.total_usdc_deposits
    }

    /// Get the collateralization ratio
    pub fn collateralization_ratio(&self) -> Option<f64> {
        if self.total_usdv_supply == 0 {
            return Some(f64::INFINITY);
        }
        Some(self.total_usdc_deposits as f64 / self.total_usdv_supply as f64)
    }

    /// Check if the program is healthy
    pub fn is_healthy(&self) -> bool {
        self.is_initialized && self.is_peg_maintained()
    }
}

/// Token balance information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenBalance {
    /// Token mint address
    pub mint: Pubkey,
    /// Token account address
    pub account: Pubkey,
    /// Balance in base units
    pub amount: u64,
    /// Number of decimal places
    pub decimals: u8,
    /// UI amount (human readable)
    pub ui_amount: f64,
}

impl TokenBalance {
    /// Create new token balance
    pub fn new(mint: Pubkey, account: Pubkey, amount: u64, decimals: u8) -> Self {
        let ui_amount = amount as f64 / 10_f64.powi(decimals as i32);
        
        Self {
            mint,
            account,
            amount,
            decimals,
            ui_amount,
        }
    }

    /// Check if balance is zero
    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }

    /// Check if balance is sufficient for amount
    pub fn is_sufficient(&self, required: u64) -> bool {
        self.amount >= required
    }
}

/// User account information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAccountInfo {
    /// User's public key
    pub user: Pubkey,
    /// USDC balance information
    pub usdc_balance: TokenBalance,
    /// USDv balance information
    pub usdv_balance: TokenBalance,
}

impl UserAccountInfo {
    /// Create new user account info
    pub fn new(
        user: Pubkey,
        usdc_balance: TokenBalance,
        usdv_balance: TokenBalance,
    ) -> Self {
        Self {
            user,
            usdc_balance,
            usdv_balance,
        }
    }

    /// Check if user can deposit the specified amount
    pub fn can_deposit(&self, amount: u64) -> bool {
        self.usdc_balance.is_sufficient(amount)
    }

    /// Check if user can burn the specified amount
    pub fn can_burn(&self, amount: u64) -> bool {
        self.usdv_balance.is_sufficient(amount)
    }

    /// Get total USD value (assuming 1:1 peg)
    pub fn total_usd_value(&self) -> f64 {
        self.usdc_balance.ui_amount + self.usdv_balance.ui_amount
    }
}

/// Transaction status information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionInfo {
    /// Transaction signature
    pub signature: String,
    /// Transaction status
    pub status: TransactionStatus,
    /// Block height when confirmed
    pub block_height: Option<u64>,
    /// Confirmation time
    pub confirmation_time: Option<i64>,
    /// Transaction fee in lamports
    pub fee: Option<u64>,
}

/// Transaction status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// Transaction is pending
    Pending,
    /// Transaction confirmed
    Confirmed,
    /// Transaction finalized
    Finalized,
    /// Transaction failed
    Failed(String),
}

/// Deposit operation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositResult {
    /// Transaction information
    pub transaction: TransactionInfo,
    /// Amount of USDC deposited
    pub usdc_deposited: u64,
    /// Amount of USDv minted
    pub usdv_minted: u64,
    /// New user USDC balance
    pub new_usdc_balance: u64,
    /// New user USDv balance
    pub new_usdv_balance: u64,
}

/// Burn operation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BurnResult {
    /// Transaction information
    pub transaction: TransactionInfo,
    /// Amount of USDv burned
    pub usdv_burned: u64,
    /// Amount of USDC withdrawn
    pub usdc_withdrawn: u64,
    /// New user USDC balance
    pub new_usdc_balance: u64,
    /// New user USDv balance
    pub new_usdv_balance: u64,
}

/// System health information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Program state information
    pub program_state: ProgramStateInfo,
    /// Whether system is healthy
    pub is_healthy: bool,
    /// Total value locked in USD
    pub total_value_locked: f64,
    /// Collateralization ratio
    pub collateralization_ratio: f64,
    /// Last update timestamp
    pub last_updated: i64,
}

impl SystemHealth {
    /// Create new system health info
    pub fn new(program_state: ProgramStateInfo) -> Self {
        let is_healthy = program_state.is_healthy();
        let total_value_locked = program_state.total_usdc_deposits as f64 / 1_000_000.0; // Convert to USDC
        let collateralization_ratio = program_state.collateralization_ratio().unwrap_or(0.0);
        let last_updated = chrono::Utc::now().timestamp();

        Self {
            program_state,
            is_healthy,
            total_value_locked,
            collateralization_ratio,
            last_updated,
        }
    }

    /// Check if system is at risk
    pub fn is_at_risk(&self) -> bool {
        !self.is_healthy || self.collateralization_ratio < 1.0
    }

    /// Get risk level description
    pub fn risk_level(&self) -> &'static str {
        if !self.is_healthy {
            "CRITICAL"
        } else if self.collateralization_ratio < 0.95 {
            "HIGH"
        } else if self.collateralization_ratio < 1.0 {
            "MEDIUM"
        } else {
            "LOW"
        }
    }
}

/// Configuration for client operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Maximum number of retries for failed operations
    pub max_retries: u32,
    /// Retry delay in milliseconds
    pub retry_delay_ms: u64,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Whether to use confirmed commitment level
    pub use_confirmed_commitment: bool,
    /// Whether to skip preflight checks
    pub skip_preflight: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay_ms: 1000,
            timeout_seconds: 30,
            use_confirmed_commitment: true,
            skip_preflight: false,
        }
    }
}
