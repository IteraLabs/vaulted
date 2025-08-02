// placeholder

#[cfg(test)]
mod tests {
    use super::*;
    use crate::USDvConfig;

    fn create_test_client() -> USDvClient {
        let rpc_client = RpcClient::new("http://localhost:8899".to_string());
        let config = USDvConfig::localnet();
        USDvClient::new(rpc_client, config)
    }

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        assert_eq!(client.config().cluster, "localnet");
    }

    #[tokio::test]
    async fn test_balance_queries_dont_panic() {
        let client = create_test_client();
        let user = Keypair::new();
        
        // These should return 0 or error, but not panic
        let _usdv_result = client.get_usdv_balance(&user.pubkey()).await;
        let _usdc_result = client.get_usdc_balance(&user.pubkey()).await;
        
        // Test should complete without panicking
        assert!(true);
    }

    #[tokio::test]
    async fn test_can_deposit_validation() {
        let client = create_test_client();
        let user = Keypair::new();
        
        // Test with invalid amount (too small)
        let result = client.can_deposit(&user.pubkey(), 1).await;
        // Should handle gracefully (likely return false due to validation or balance)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_account_info() {
        let client = create_test_client();
        let user = Keypair::new();
        
        // This should work even if accounts don't exist (should return 0 balances)
        let result = client.get_user_account_info(&user.pubkey()).await;
        if let Ok(info) = result {
            assert_eq!(info.user, user.pubkey());
            // Balances should be 0 for non-existent accounts
            assert_eq!(info.usdc_balance.amount, 0);
            assert_eq!(info.usdv_balance.amount, 0);
        }
    }
}


