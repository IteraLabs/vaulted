//! Program state definitions and account structures

use anchor_lang::prelude::*;

/// Global program state account
/// 
/// Stores configuration, statistics, and administrative information
/// for the USDv stablecoin program.
#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    /// Whether the program has been initialized
    pub is_initialized: bool,
    
    /// Admin public key (can update program parameters)
    pub admin: Pubkey,
    
    /// USDc mint address that USDv is pegged to
    pub usdc_mint: Pubkey,
    
    /// USDv mint address (controlled by program)
    pub usdv_mint: Pubkey,
    
    /// Vault authority PDA public key
    pub vault_authority: Pubkey,
    
    /// Bump seed for vault authority PDA
    pub vault_bump: u8,
    
    /// Total USDv tokens in circulation
    pub total_usdv_supply: u64,
    
    /// Total USDc deposited in vault
    pub total_usdc_deposits: u64,
}

impl ProgramState {
    /// Seed for deriving the vault authority PDA
    pub const VAULT_AUTHORITY_SEED: &'static [u8] = b"vault_authority";
    
    /// Check if the 1:1 peg is maintained
    pub fn is_peg_maintained(&self) -> bool {
        self.total_usdv_supply == self.total_usdc_deposits
    }
    
    /// Get the collateralization ratio (should always be 1.0 for healthy state)
    pub fn collateralization_ratio(&self) -> Option<f64> {
        if self.total_usdv_supply == 0 {
            return Some(f64::INFINITY);
        }
        
        Some(self.total_usdc_deposits as f64 / self.total_usdv_supply as f64)
    }
    
    /// Calculate maximum additional USDv that can be minted
    pub fn max_additional_mint(&self, vault_usdc_balance: u64) -> u64 {
        vault_usdc_balance.saturating_sub(self.total_usdv_supply)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_state_peg_maintenance() {
        let mut state = ProgramState {
            is_initialized: true,
            admin: Pubkey::new_unique(),
            usdc_mint: Pubkey::new_unique(),
            usdv_mint: Pubkey::new_unique(),
            vault_authority: Pubkey::new_unique(),
            vault_bump: 255,
            total_usdv_supply: 1000,
            total_usdc_deposits: 1000,
        };

        // Test 1:1 peg maintenance
        assert!(state.is_peg_maintained());
        assert_eq!(state.collateralization_ratio(), Some(1.0));

        // Test broken peg
        state.total_usdc_deposits = 900;
        assert!(!state.is_peg_maintained());
        assert_eq!(state.collateralization_ratio(), Some(0.9));

        // Test over-collateralized
        state.total_usdc_deposits = 1100;
        assert!(!state.is_peg_maintained());
        assert_eq!(state.collateralization_ratio(), Some(1.1));
    }

    #[test]
    fn test_max_additional_mint() {
        let state = ProgramState {
            is_initialized: true,
            admin: Pubkey::new_unique(),
            usdc_mint: Pubkey::new_unique(),
            usdv_mint: Pubkey::new_unique(),
            vault_authority: Pubkey::new_unique(),
            vault_bump: 255,
            total_usdv_supply: 500,
            total_usdc_deposits: 500,
        };

        assert_eq!(state.max_additional_mint(1000), 500);
        assert_eq!(state.max_additional_mint(500), 0);
        assert_eq!(state.max_additional_mint(300), 0);
    }
}
