//! # USDv Stablecoin Program
//!
//! A fully-collateralized stablecoin pegged 1:1 to USDC on Solana.
//!
//! ## Overview
//!
//! USDv maintains a 1:1 peg with USDC through direct collateralization:
//! - Users deposit USDC to mint USDv tokens (1:1 ratio)
//! - Users burn USDv tokens to withdraw USDC (1:1 ratio)
//! - Only users can initiate burn operations (not the program)
//! - All USDC is securely stored in a PDA vault

use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub use state::*;

declare_id!("USDvCoinProgram1111111111111111111111111111");

/// Main program entry point
#[program]
pub mod usdv_program {
    /// Initialize the USDv stablecoin program
    /// 
    /// Creates the USDv mint with program as mint authority.
    /// This is a one-time setup function that must be called by an admin.
    pub fn initialize(
        ctx: Context<Initialize>,
        usdc_mint_key: Pubkey,
    ) -> Result<()> {
        instructions::initialize(ctx, usdc_mint_key)
    }

    /// Deposit USDc and mint USDv tokens (1:1 ratio)
    pub fn deposit_and_mint(
        ctx: Context<DepositAndMint>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_and_mint(ctx, amount)
    }

    /// Burn USDv tokens and withdraw USDc (1:1 ratio)
    pub fn burn_and_withdraw(
        ctx: Context<BurnAndWithdraw>,
        amount: u64,
    ) -> Result<()> {
        instructions::burn_and_withdraw(ctx, amount)
    }

    /// Update program parameters (admin only)
    pub fn update_program_state(
        ctx: Context<UpdateProgramState>,
        new_admin: Option<Pubkey>,
    ) -> Result<()> {
        instructions::update_program_state(ctx, new_admin)
    }
}

