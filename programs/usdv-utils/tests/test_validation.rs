#[cfg(test)]
mod tests {

    use usdv_utils::*;

    #[test]
    fn test_deposit_amount_validation() {
        // Valid amounts
        assert!(validate_deposit_amount(MIN_DEPOSIT_AMOUNT).is_ok());
        assert!(validate_deposit_amount(MAX_DEPOSIT_AMOUNT).is_ok());
        assert!(validate_deposit_amount(1_000_000).is_ok()); // 1 USDC
        
        // Invalid amounts
        assert_eq!(
            validate_deposit_amount(0),
            Err(ValidationError::ZeroAmount)
        );
        
        assert_eq!(
            validate_deposit_amount(MIN_DEPOSIT_AMOUNT - 1),
            Err(ValidationError::AmountTooSmall {
                amount: MIN_DEPOSIT_AMOUNT - 1,
                min: MIN_DEPOSIT_AMOUNT,
            })
        );
        
        assert_eq!(
            validate_deposit_amount(MAX_DEPOSIT_AMOUNT + 1),
            Err(ValidationError::AmountTooLarge {
                amount: MAX_DEPOSIT_AMOUNT + 1,
                max: MAX_DEPOSIT_AMOUNT,
            })
        );
    }

    #[test]
    fn test_burn_amount_validation() {
        let user_balance = 1_000_000; // 1 USDv
        
        // Valid amounts
        assert!(validate_burn_amount(500_000, user_balance).is_ok());
        assert!(validate_burn_amount(user_balance, user_balance).is_ok());
        
        // Invalid amounts
        assert_eq!(
            validate_burn_amount(0, user_balance),
            Err(ValidationError::ZeroAmount)
        );
        
        assert_eq!(
            validate_burn_amount(user_balance + 1, user_balance),
            Err(ValidationError::InsufficientBalance {
                required: user_balance + 1,
                available: user_balance,
            })
        );
    }

    #[test]
    fn test_supply_limit_validation() {
        let current_supply = MAX_TOTAL_SUPPLY - 1000;
        
        // Valid mint (within limit)
        assert!(validate_mint_supply_limit(500, current_supply).is_ok());
        assert!(validate_mint_supply_limit(1000, current_supply).is_ok());
        
        // Invalid mint (exceeds limit)
        assert_eq!(
            validate_mint_supply_limit(1001, current_supply),
            Err(ValidationError::SupplyLimitExceeded {
                requested: current_supply + 1001,
                max: MAX_TOTAL_SUPPLY,
            })
        );
    }

    #[test]
    fn test_pubkey_validation() {
        let valid_pubkey = Pubkey::new_unique();
        assert!(validate_pubkey(&valid_pubkey).is_ok());
        
        let system_program = solana_sdk::system_program::id();
        assert_eq!(
            validate_pubkey(&system_program),
            Err(ValidationError::InvalidPublicKey(
                "System program ID not allowed".to_string()
            ))
        );
    }

    #[test]
    fn test_decimals_validation() {
        // Valid decimals
        assert!(validate_mint_decimals(6).is_ok());
        
        // Invalid decimals
        assert_eq!(
            validate_mint_decimals(9),
            Err(ValidationError::InvalidDecimals(9))
        );
        assert_eq!(
            validate_mint_decimals(0),
            Err(ValidationError::InvalidDecimals(0))
        );
    }

    #[test]
    fn test_comprehensive_validations() {
        // Valid deposit operation
        assert!(validate_deposit_operation(
            1_000_000,      // 1 USDC deposit
            2_000_000,      // 2 USDC user balance
            1_000_000_000   // 1B current supply
        ).is_ok());
        
        // Invalid deposit operation (insufficient balance)
        assert!(validate_deposit_operation(
            2_000_000,      // 2 USDC deposit
            1_000_000,      // 1 USDC user balance  
            1_000_000_000   // 1B current supply
        ).is_err());
        
        // Valid burn operation
        assert!(validate_burn_operation(
            1_000_000,      // 1 USDv burn
            2_000_000,      // 2 USDv user balance
            5_000_000       // 5 USDC vault balance
        ).is_ok());
        
        // Invalid burn operation (insufficient vault balance)
        assert!(validate_burn_operation(
            2_000_000,      // 2 USDv burn
            3_000_000,      // 3 USDv user balance
            1_000_000       // 1 USDC vault balance
        ).is_err());
    }
}
