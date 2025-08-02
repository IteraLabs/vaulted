//! # USDv Utilities Library
//!
//! Common utilities and helper functions used across the USDv stablecoin ecosystem.
//!
//! ## Features
//!
//! - **Mathematical Operations**: Safe arithmetic with overflow protection
//! - **Address Derivation**: PDA and associated token account utilities
//! - **Constants**: Program-wide constants and configuration values
//! - **Validation**: Input validation and sanitization functions

pub mod constants;
pub mod math;
pub mod pda;
pub mod validation;

pub use constants::*;
pub use validation::*;

// Re-export commonly used types
pub use solana_sdk::pubkey::Pubkey;

