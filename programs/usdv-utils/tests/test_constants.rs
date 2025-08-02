#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use usdv_utils::*;

    #[test]
    fn test_constants_validity() {
        // Test decimal consistency
        assert_eq!(USDC_DECIMALS, USDV_DECIMALS);
        
        // Test amount limits
        assert!(MIN_DEPOSIT_AMOUNT < MAX_DEPOSIT_AMOUNT);
        assert!(MAX_TOTAL_SUPPLY > MAX_DEPOSIT_AMOUNT);
        
        // Test account space
        assert!(account_space::PROGRAM_STATE > 0);
        assert!(account_space::PROGRAM_STATE < 10240); // Solana account size limit

    }

    #[test]
    fn test_mint_addresses() {
        // Test that mint addresses are valid pubkeys
        let mainnet_mint = Pubkey::from_str(MAINNET_USDC_MINT);
        assert!(mainnet_mint.is_ok());
        
        let devnet_mint = Pubkey::from_str(DEVNET_USDC_MINT);
        assert!(devnet_mint.is_ok());
        
        // They should be different
        assert_ne!(mainnet_mint.unwrap(), devnet_mint.unwrap());
    }

    #[test]
    fn test_fee_calculations() {
        assert!(fees::FEE_BASIS_POINTS <= fees::MAX_FEE_BASIS_POINTS);
        assert!(fees::MAX_FEE_BASIS_POINTS <= 10000); // 100% max
    }
}

