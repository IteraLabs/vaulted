//! Instruction implementations for USDv stablecoin program

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer, MintTo, Burn},
};

use usdv_results::USDvError;
use crate::ProgramState;

/// Initialize the USDv stablecoin program
pub fn initialize(
    ctx: Context<Initialize>,
    usdc_mint_key: Pubkey,
) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    
    // Ensure program hasn't been initialized yet
    require!(!program_state.is_initialized, USDvError::AlreadyInitialized);
    
    // Initialize program state
    program_state.is_initialized = true;
    program_state.admin = ctx.accounts.admin.key();
    program_state.usdc_mint = usdc_mint_key;
    program_state.usdv_mint = ctx.accounts.usdv_mint.key();
    program_state.vault_authority = ctx.accounts.vault_authority.key();
    program_state.vault_bump = ctx.bumps.vault_authority;
    program_state.total_usdv_supply = 0;
    program_state.total_usdc_deposits = 0;
    
    msg!("USDv Stablecoin program initialized successfully");
    msg!("USDc Mint: {}", usdc_mint_key);
    msg!("USDv Mint: {}", ctx.accounts.usdv_mint.key());
    
    Ok(())
}

/// Deposit USDC and mint USDv tokens (1:1 ratio)
pub fn deposit_and_mint(
    ctx: Context<DepositAndMint>,
    amount: u64,
) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    
    // Ensure program is initialized
    require!(program_state.is_initialized, USDvError::NotInitialized);
    
    // Verify USDc mint matches
    require!(
        ctx.accounts.usdc_mint.key() == program_state.usdc_mint,
        USDvError::InvalidUSDCMint
    );
    
    // Verify user has sufficient USDc balance
    require!(
        ctx.accounts.user_usdc_account.amount >= amount,
        USDvError::InsufficientUSDCBalance
    );
    
    // Transfer USDC from user to vault
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_usdc_account.to_account_info(),
            to: ctx.accounts.vault_usdc_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amount)?;
    
    // Mint USDv tokens to user (1:1 ratio)
    let vault_authority_bump = program_state.vault_bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        ProgramState::VAULT_AUTHORITY_SEED,
        &[vault_authority_bump],
    ]];
    
    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.usdv_mint.to_account_info(),
            to: ctx.accounts.user_usdv_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        },
        signer_seeds,
    );
    token::mint_to(mint_ctx, amount)?;
    
    // Update program state
    program_state.total_usdv_supply = program_state.total_usdv_supply
        .checked_add(amount)
        .ok_or(USDvError::ArithmeticOverflow)?;
    program_state.total_usdc_deposits = program_state.total_usdc_deposits
        .checked_add(amount)
        .ok_or(USDvError::ArithmeticOverflow)?;
    
    msg!("Successfully deposited {} USDC and minted {} USDv", amount, amount);
    
    Ok(())
}

/// Burn USDv tokens and withdraw USDC (1:1 ratio)
pub fn burn_and_withdraw(
    ctx: Context<BurnAndWithdraw>,
    amount: u64,
) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    
    // Ensure program is initialized
    require!(program_state.is_initialized, USDvError::NotInitialized);
    
    // Verify user has sufficient USDv balance
    require!(
        ctx.accounts.user_usdv_account.amount >= amount,
        USDvError::InsufficientUSDvBalance
    );
    
    // Burn USDv tokens from user account
    let burn_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.usdv_mint.to_account_info(),
            from: ctx.accounts.user_usdv_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::burn(burn_ctx, amount)?;
    
    // Transfer USDC from vault back to user (1:1 ratio)
    let vault_authority_bump = program_state.vault_bump;
    let signer_seeds: &[&[&[u8]]] = &[&[
        ProgramState::VAULT_AUTHORITY_SEED,
        &[vault_authority_bump],
    ]];
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_usdc_account.to_account_info(),
            to: ctx.accounts.user_usdc_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, amount)?;
    
    // Update program state
    program_state.total_usdv_supply = program_state.total_usdv_supply
        .checked_sub(amount)
        .ok_or(USDvError::ArithmeticOverflow)?;
    program_state.total_usdc_deposits = program_state.total_usdc_deposits
        .checked_sub(amount)
        .ok_or(USDvError::ArithmeticOverflow)?;
    
    msg!("Successfully burned {} USDv and withdrew {} USDC", amount, amount);
    
    Ok(())
}

/// Update program parameters (admin only)
pub fn update_program_state(
    ctx: Context<UpdateProgramState>,
    new_admin: Option<Pubkey>,
) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    
    // Only current admin can update
    require!(
        ctx.accounts.admin.key() == program_state.admin,
        USDvError::Unauthorized
    );
    
    if let Some(new_admin_key) = new_admin {
        program_state.admin = new_admin_key;
        msg!("Admin updated to: {}", new_admin_key);
    }
    
    Ok(())
}

// Account validation structs

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + ProgramState::INIT_SPACE
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = vault_authority,
        mint::freeze_authority = vault_authority,
    )]
    pub usdv_mint: Account<'info, Mint>,
    
    #[account(
        seeds = [ProgramState::VAULT_AUTHORITY_SEED],
        bump
    )]
    /// CHECK: This is a PDA used as authority
    pub vault_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositAndMint<'info> {
    #[account(
        mut,
        constraint = program_state.is_initialized @ USDvError::NotInitialized
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        constraint = usdc_mint.key() == program_state.usdc_mint @ USDvError::InvalidUSDCMint
    )]
    pub usdc_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        constraint = usdv_mint.key() == program_state.usdv_mint
    )]
    pub usdv_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = user,
    )]
    pub user_usdc_account: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = usdv_mint,
        associated_token::authority = user,
    )]
    pub user_usdv_account: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = usdc_mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_usdc_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [ProgramState::VAULT_AUTHORITY_SEED],
        bump = program_state.vault_bump
    )]
    /// CHECK: This is a PDA used as authority
    pub vault_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BurnAndWithdraw<'info> {
    #[account(
        mut,
        constraint = program_state.is_initialized @ USDvError::NotInitialized
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        mut,
        constraint = usdv_mint.key() == program_state.usdv_mint
    )]
    pub usdv_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = usdv_mint,
        associated_token::authority = user,
    )]
    pub user_usdv_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = program_state.usdc_mint,
        associated_token::authority = user,
    )]
    pub user_usdc_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = program_state.usdc_mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_usdc_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [ProgramState::VAULT_AUTHORITY_SEED],
        bump = program_state.vault_bump
    )]
    /// CHECK: This is a PDA used as authority
    pub vault_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateProgramState<'info> {
    #[account(
        mut,
        constraint = program_state.admin == admin.key() @ USDvError::Unauthorized
    )]
    pub program_state: Account<'info, ProgramState>,
    
    pub admin: Signer<'info>,
}
