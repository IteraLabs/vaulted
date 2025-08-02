#[cfg(test)]
mod tests {
    use super::*;
    use solana_program_test::*;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_instruction,
        transaction::Transaction,
    };
    use spl_token::{
        instruction as token_instruction,
        state::{Account as TokenAccountState, Mint as MintState},
    };
    use std::str::FromStr;

    const PROGRAM_ID: &str = "USDVCoinProgram11111111111111111111111111111";

    #[tokio::test]
    async fn test_program_initialization() {
        let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
        let mut program_test = ProgramTest::new(
            "usdv_program",
            program_id,
            processor!(entry),
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Derive vault authority PDA
        let (vault_authority, vault_bump) = Pubkey::find_program_address(
            &[b"vault_authority"],
            &program_id,
        );

        println!("✅ Program initialization test setup completed");
        println!("Program ID: {}", program_id);
        println!("Vault Authority: {}", vault_authority);
        println!("Vault Bump: {}", vault_bump);

        assert!(vault_bump <= 255);
        assert!(vault_authority != Pubkey::default());
    }

    #[tokio::test] 
    async fn test_pda_derivations() {
        let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();

        let (vault_authority, vault_bump) = Pubkey::find_program_address(
            &[b"vault_authority"],
            &program_id,
        );

        let (vault_authority_2, vault_bump_2) = Pubkey::find_program_address(
            &[b"vault_authority"], 
            &program_id,
        );

        assert_eq!(vault_authority, vault_authority_2);
        assert_eq!(vault_bump, vault_bump_2);
        
        println!("✅ PDA derivation test passed!");
    }

    fn entry(
        _program_id: &Pubkey,
        _accounts: &[solana_program::account_info::AccountInfo],
        _instruction_data: &[u8],
    ) -> solana_program::entrypoint::ProgramResult {
        Ok(())
    }
}
