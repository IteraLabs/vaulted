//! Error definitions for the USDv stablecoin program
use anchor_lang::prelude::*;

#[error_code]
pub enum USDvError {
    #[msg("Invalid USDC mint address")]
    InvalidUSDCMint,
    
    #[msg("Insufficient USDC balance")]
    InsufficientUSDCBalance,
    
    #[msg("Insufficient USDv balance to burn")]
    InsufficientUSDvBalance,
    
    #[msg("Invalid vault authority")]
    InvalidVaultAuthority,
    
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    
    #[msg("Unauthorized: Only admin can perform this action")]
    Unauthorized,
    
    #[msg("Program already initialized")]
    AlreadyInitialized,
    
    #[msg("Program not initialized")]
    NotInitialized,
    
    #[msg("Invalid peg ratio - must maintain 1:1")]
    InvalidPegRatio,
    
    #[msg("Vault balance insufficient for withdrawal")]
    InsufficientVaultBalance,
    
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    
    #[msg("Account validation failed")]
    AccountValidationError,
    
    // Convert client/network errors to simple program errors
    #[msg("Solana client operation failed")]
    SolanaClientError,

    #[msg("Invalid public key format")]
    InvalidPublicKey,

    #[msg("Insufficient balance for operation")]
    InsufficientBalance,

    #[msg("Transaction execution failed")]
    TransactionFailed,

    #[msg("Required account not found")]
    AccountNotFound,

    #[msg("Invalid mint address provided")]
    InvalidMintAddress,

    #[msg("Invalid amount specified")]
    InvalidAmount,

    #[msg("Data serialization failed")]
    SerializationError,

    #[msg("Network operation failed")]
    NetworkError,

    #[msg("Configuration error occurred")]
    ConfigurationError,

    #[msg("Operation timed out")]
    TimeoutError,

    #[msg("Rate limit exceeded")]
    RateLimitExceeded,

    #[msg("Internal system error")]
    InternalError,
}

impl From<USDvError> for ProgramError {
    fn from(e: USDvError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

