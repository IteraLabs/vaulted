//! Input validation and sanitization utilities
use crate::constants::{MIN_DEPOSIT_AMOUNT, MAX_DEPOSIT_AMOUNT, MAX_TOTAL_SUPPLY};
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

/// Result type for validation operations
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Validation error types
#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("Invalid amount: {amount}. Must be between {min} and {max}")]
    InvalidAmount { amount: u64, min: u64, max: u64 },
    
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),
    
    #[error("Amount too small: {amount}. Minimum is {min}")]
    AmountTooSmall { amount: u64, min: u64 },
    
    #[error("Amount too large: {amount}. Maximum is {max}")]
    AmountTooLarge { amount: u64, max: u64 },
    
    #[error("Zero amount not allowed")]
    ZeroAmount,
    
    #[error("Supply limit exceeded: {requested} would exceed maximum {max}")]
    SupplyLimitExceeded { requested: u64, max: u64 },
    
    #[error("Invalid decimals: {0}. Expected 6 for USDC/USDv")]
    InvalidDecimals(u8),
    
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u64, available: u64 },
}

/// Validate deposit amount
/// 
/// # Arguments
/// * `amount` - Amount to validate (in base units)
/// 
/// # Returns
/// * `ValidationResult<()>` - Success or validation error
pub fn validate_deposit_amount(amount: u64) -> ValidationResult<()> {
    if amount == 0 {
        return Err(ValidationError::ZeroAmount);
    }
    
    if amount < MIN_DEPOSIT_AMOUNT {
        return Err(ValidationError::AmountTooSmall {
            amount,
            min: MIN_DEPOSIT_AMOUNT,
        });
    }
    
    if amount > MAX_DEPOSIT_AMOUNT {
        return Err(ValidationError::AmountTooLarge {
            amount,
            max: MAX_DEPOSIT_AMOUNT,
        });
    }
    
    Ok(())
}

/// Validate burn amount
/// 
/// # Arguments
/// * `amount` - Amount to validate (in base units)
/// * `user_balance` - User's current balance
/// 
/// # Returns
/// * `ValidationResult<()>` - Success or validation error
pub fn validate_burn_amount(amount: u64, user_balance: u64) -> ValidationResult<()> {
    if amount == 0 {
        return Err(ValidationError::ZeroAmount);
    }
    
    if amount > user_balance {
        return Err(ValidationError::InsufficientBalance {
            required: amount,
            available: user_balance,
        });
    }
    
    Ok(())
}

/// Validate that a mint operation won't exceed total supply limit
/// 
/// # Arguments
/// * `mint_amount` - Amount to mint
/// * `current_supply` - Current total supply
/// 
/// # Returns
/// * `ValidationResult<()>` - Success or validation error
pub fn validate_mint_supply_limit(mint_amount: u64, current_supply: u64) -> ValidationResult<()> {
    let new_supply = current_supply.saturating_add(mint_amount);
    
    if new_supply > MAX_TOTAL_SUPPLY {
        return Err(ValidationError::SupplyLimitExceeded {
            requested: new_supply,
            max: MAX_TOTAL_SUPPLY,
        });
    }
    
    Ok(())
}

/// Validate public key format
/// 
/// # Arguments
/// * `pubkey` - Public key to validate
/// 
/// # Returns
/// * `ValidationResult<()>` - Success or validation error
pub fn validate_pubkey(pubkey: &Pubkey) -> ValidationResult<()> {
    // Check if it's the system program (invalid for most use cases)
    if *pubkey == solana_sdk::system_program::id() {
        return Err(ValidationError::InvalidPublicKey(
            "System program ID not allowed".to_string()
        ));
    }
    
    Ok(())
}

/// Validate mint decimals
/// 
/// # Arguments
/// * `decimals` - Number of decimals to validate
/// 
/// # Returns
/// * `ValidationResult<()>` - Success or validation error  
pub fn validate_mint_decimals(decimals: u8) -> ValidationResult<()> {
    if decimals != 6 {
        return Err(ValidationError::InvalidDecimals(decimals));
    }
    
    Ok(())
}

/// Validate that two amounts match (for 1:1 peg verification)
/// 
/// # Arguments
/// * `amount1` - First amount
/// * `amount2` - Second amount
/// * `context` - Context for error message
/// 
/// # Returns
/// * `ValidationResult<()>` - Success or validation error
pub fn validate_amounts_match(amount1: u64, amount2: u64, _context: &str) -> ValidationResult<()> {
    if amount1 != amount2 {
        return Err(ValidationError::InvalidAmount {
            amount: amount2,
            min: amount1,
            max: amount1,
        });
    }
    
    Ok(())
}

/// Comprehensive validation for deposit operation
/// 
/// # Arguments
/// * `amount` - Deposit amount
/// * `user_usdc_balance` - User's USDC balance
/// * `current_supply` - Current USDv supply
/// 
/// # Returns
/// * `ValidationResult<()>` - Success or validation error
pub fn validate_deposit_operation(
    amount: u64,
    user_usdc_balance: u64,
    current_supply: u64,
) -> ValidationResult<()> {
    validate_deposit_amount(amount)?;
    
    if amount > user_usdc_balance {
        return Err(ValidationError::InsufficientBalance {
            required: amount,
            available: user_usdc_balance,
        });
    }
    
    validate_mint_supply_limit(amount, current_supply)?;
    
    Ok(())
}

/// Comprehensive validation for burn operation
/// 
/// # Arguments
/// * `amount` - Burn amount
/// * `user_usdv_balance` - User's USDv balance
/// * `vault_usdc_balance` - Vault's USDC balance
/// 
/// # Returns
/// * `ValidationResult<()>` - Success or validation error
pub fn validate_burn_operation(
    amount: u64,
    user_usdv_balance: u64,
    vault_usdc_balance: u64,
) -> ValidationResult<()> {
    validate_burn_amount(amount, user_usdv_balance)?;
    
    if amount > vault_usdc_balance {
        return Err(ValidationError::InsufficientBalance {
            required: amount,
            available: vault_usdc_balance,
        });
    }
    
    Ok(())
}

