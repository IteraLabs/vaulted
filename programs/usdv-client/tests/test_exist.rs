// placeholder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
        println!("USDv Client Library version: {}", VERSION);
    }

    #[test]
    fn test_program_ids_valid() {
        use std::str::FromStr;
        
        let mainnet_id = Pubkey::from_str(MAINNET_PROGRAM_ID);
        assert!(mainnet_id.is_ok());
        
        let devnet_id = Pubkey::from_str(DEVNET_PROGRAM_ID);
        assert!(devnet_id.is_ok());
        
        println!("Mainnet Program ID: {}", MAINNET_PROGRAM_ID);
        println!("Devnet Program ID: {}", DEVNET_PROGRAM_ID);
    }
}
