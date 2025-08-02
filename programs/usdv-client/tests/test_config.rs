#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let mainnet = USDvConfig::mainnet();
        assert_eq!(mainnet.cluster, "mainnet-beta");
        assert_eq!(mainnet.rpc_url(), "https://api.mainnet-beta.solana.com");

        let devnet = USDvConfig::devnet();
        assert_eq!(devnet.cluster, "devnet");
        assert_eq!(devnet.rpc_url(), "https://api.devnet.solana.com");

        let localnet = USDvConfig::localnet();
        assert_eq!(localnet.cluster, "localnet");
        assert_eq!(localnet.rpc_url(), "http://localhost:8899");
    }

    #[test]
    fn test_custom_config() {
        let program_id = Pubkey::new_unique();
        let program_state = Pubkey::new_unique();
        let usdc_mint = Pubkey::new_unique();
        let usdv_mint = Pubkey::new_unique();
        
        let config = USDvConfig::custom(
            program_id,
            program_state,
            usdc_mint,
            usdv_mint,
            "custom-cluster".to_string(),
        );

        assert_eq!(config.program_id, program_id);
        assert_eq!(config.cluster, "custom-cluster");
        assert_eq!(config.rpc_url(), "custom-cluster");
    }
}

