//! Main client implementation for USDv stablecoin operations

// use crate::{USDvConfig, USDvError, Result, InstructionBuilder, ProgramStateInfo, utils};
use usdv_results::USDvError;
use crate::config::USDvConfig;
use crate::types::ProgramStateInfo;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use std::sync::Arc;

/// Main client for interacting with USDv stablecoin program
pub struct USDvClient {
    /// Solana RPC client
    rpc_client: Arc<RpcClient>,
    /// USDv program configuration
    config: USDvConfig,
    /// Instruction builder
    instruction_builder: InstructionBuilder,
    /// Client configuration
    client_config: crate::types::ClientConfig,
}

impl USDvClient {
    /// Create a new USDv client
    pub fn new(rpc_client: RpcClient, config: USDvConfig) -> Self {
        let instruction_builder = InstructionBuilder::new(config.program_id);
        
        Self {
            rpc_client: Arc::new(rpc_client),
            config,
            instruction_builder,
            client_config: Default::default(),
        }
    }

    /// Create a new USDv client with custom configuration
    pub fn with_config(
        rpc_client: RpcClient,
        config: USDvConfig,
        client_config: crate::types::ClientConfig,
    ) -> Self {
        let instruction_builder = InstructionBuilder::new(config.program_id);
        
        Self {
            rpc_client: Arc::new(rpc_client),
            config,
            instruction_builder,
            client_config,
        }
    }

    /// Get the RPC client
    pub fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    /// Get the program configuration
    pub fn config(&self) -> &USDvConfig {
        &self.config
    }

    /// Initialize the USDv program (admin only)
    pub async fn initialize(
        &self,
        admin: &Keypair,
        usdc_mint: &Pubkey,
    ) -> Result<Signature> {
        // Generate keypairs for new accounts
        let program_state = Keypair::new();
        let usdv_mint = Keypair::new();

        // Derive vault authority PDA
        let (vault_authority, _) = Pubkey::find_program_address(
            &[b"vault_authority"],
            &self.config.program_id,
        );

        // Build initialize instruction
        let instruction = self.instruction_builder.initialize(
            &admin.pubkey(),
            &program_state.pubkey(),
            &usdv_mint.pubkey(),
            &vault_authority,
            usdc_mint,
        )?;

        // Send transaction
        self.send_transaction_with_signers(
            &[instruction],
            &[admin, &program_state, &usdv_mint],
        ).await
    }

    /// Deposit USDC and mint USDv tokens
    pub async fn deposit_and_mint(
        &self,
        user: &Keypair,
        amount: u64,
    ) -> Result<Signature> {
        // Validate amount first
        usdv_utils::validate_deposit_amount(amount)
            .map_err(|e| USDvError::InvalidAmount(e.to_string()))?;

        // Build instruction
        let instruction = self.instruction_builder.deposit_and_mint(
            &self.config.program_state,
            &self.config.usdc_mint,
            &self.config.usdv_mint,
            &user.pubkey(),
            amount,
        )?;

        // Send transaction
        self.send_transaction_with_signers(&[instruction], &[user]).await
    }

    /// Burn USDv tokens and withdraw USDC
    pub async fn burn_and_withdraw(
        &self,
        user: &Keypair,
        amount: u64,
    ) -> Result<Signature> {
        // Get user's current USDv balance for validation
        let usdv_balance = self.get_usdv_balance(&user.pubkey()).await?;
        
        // Validate burn amount
        usdv_utils::validate_burn_amount(amount, usdv_balance)
            .map_err(|e| USDvError::InvalidAmount(e.to_string()))?;

        // Build instruction
        let instruction = self.instruction_builder.burn_and_withdraw(
            &self.config.program_state,
            &self.config.usdv_mint,
            &self.config.usdc_mint,
            &user.pubkey(),
            amount,
        )?;

        // Send transaction
        self.send_transaction_with_signers(&[instruction], &[user]).await
    }

    /// Update program state (admin only)
    pub async fn update_program_state(
        &self,
        admin: &Keypair,
        new_admin: Option<&Pubkey>,
    ) -> Result<Signature> {
        // Build instruction
        let instruction = self.instruction_builder.update_program_state(
            &self.config.program_state,
            &admin.pubkey(),
            new_admin,
        )?;

        // Send transaction
        self.send_transaction_with_signers(&[instruction], &[admin]).await
    }

    /// Get program state information
    pub async fn get_program_state(&self) -> Result<ProgramStateInfo> {
        let account = self.rpc_client
            .get_account(&self.config.program_state)
            .map_err(USDvError::SolanaClientError)?;
            
        // Skip the 8-byte discriminator and deserialize
        if account.data.len() < 8 {
            return Err(USDvError::SerializationError("Account data too short".to_string()));
        }

        let state_data = &account.data[8..];
        let state: ProgramStateInfo = borsh::from_slice(state_data)
            .map_err(|e| USDvError::SerializationError(e.to_string()))?;

        Ok(state)
    }

    /// Get user's USDv token balance
    pub async fn get_usdv_balance(&self, user: &Pubkey) -> Result<u64> {
        let token_account = get_associated_token_address(user, &self.config.usdv_mint);
        
        match self.rpc_client.get_token_account_balance(&token_account) {
            Ok(balance) => {
                balance.amount.parse()
                    .map_err(|_| USDvError::SerializationError("Invalid balance format".to_string()))
            }
            Err(e) => {
                // Check if it's an account not found error
                if e.to_string().contains("could not find account") {
                    Ok(0) // Account doesn't exist, balance is 0
                } else {
                    Err(USDvError::SolanaClientError(e))
                }
            }
        }
    }

    /// Get user's USDC token balance
    pub async fn get_usdc_balance(&self, user: &Pubkey) -> Result<u64> {
        let token_account = get_associated_token_address(user, &self.config.usdc_mint);
        
        match self.rpc_client.get_token_account_balance(&token_account) {
            Ok(balance) => {
                balance.amount.parse()
                    .map_err(|_| USDvError::SerializationError("Invalid balance format".to_string()))
            }
            Err(e) => {
                // Check if it's an account not found error
                if e.to_string().contains("could not find account") {
                    Ok(0) // Account doesn't exist, balance is 0
                } else {
                    Err(USDvError::SolanaClientError(e))
                }
            }
        }
    }

    /// Get user account information (both USDC and USDv balances)
    pub async fn get_user_account_info(&self, user: &Pubkey) -> Result<crate::types::UserAccountInfo> {
        let usdc_balance = self.get_usdc_balance(user).await?;
        let usdv_balance = self.get_usdv_balance(user).await?;

        let usdc_account = get_associated_token_address(user, &self.config.usdc_mint);
        let usdv_account = get_associated_token_address(user, &self.config.usdv_mint);

        let usdc_token_balance = crate::types::TokenBalance::new(
            self.config.usdc_mint,
            usdc_account,
            usdc_balance,
            6, // USDC decimals
        );

        let usdv_token_balance = crate::types::TokenBalance::new(
            self.config.usdv_mint,
            usdv_account,
            usdv_balance,
            6, // USDv decimals
        );

        Ok(crate::types::UserAccountInfo::new(
            *user,
            usdc_token_balance,
            usdv_token_balance,
        ))
    }

    /// Get system health information
    pub async fn get_system_health(&self) -> Result<crate::types::SystemHealth> {
        let program_state = self.get_program_state().await?;
        Ok(crate::types::SystemHealth::new(program_state))
    }

    /// Check if user can perform deposit
    pub async fn can_deposit(&self, user: &Pubkey, amount: u64) -> Result<bool> {
        // Validate amount
        if usdv_utils::validate_deposit_amount(amount).is_err() {
            return Ok(false);
        }

        // Check user balance
        let usdc_balance = self.get_usdc_balance(user).await?;
        if usdc_balance < amount {
            return Ok(false);
        }

        // Check system health
        let health = self.get_system_health().await?;
        if !health.is_healthy {
            return Ok(false);
        }

        Ok(true)
    }

    /// Check if user can perform burn
    pub async fn can_burn(&self, user: &Pubkey, amount: u64) -> Result<bool> {
        // Check user balance
        let usdv_balance = self.get_usdv_balance(user).await?;
        if usdv_balance < amount {
            return Ok(false);
        }

        // Validate amount
        if usdv_utils::validate_burn_amount(amount, usdv_balance).is_err() {
            return Ok(false);
        }

        // Check vault has enough USDC
        let program_state = self.get_program_state().await?;
        if program_state.total_usdc_deposits < amount {
            return Ok(false);
        }

        Ok(true)
    }

    /// Estimate transaction fee
    pub async fn estimate_fee(&self, instruction: &Instruction) -> Result<u64> {
        let payer = Keypair::new(); // Dummy payer for estimation
        let recent_blockhash = self.rpc_client
            .get_latest_blockhash()
            .map_err(USDvError::SolanaClientError)?;

        let transaction = Transaction::new_signed_with_payer(
            &[instruction.clone()],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        utils::estimate_transaction_fee(&self.rpc_client, &transaction).await
    }

    /// Health check for the client connection
    pub async fn health_check(&self) -> Result<()> {
        utils::health_check_rpc(&self.rpc_client).await
    }

    /// Get network information
    pub async fn get_network_info(&self) -> Result<utils::NetworkInfo> {
        utils::get_network_info(&self.rpc_client).await
    }

    // Private helper methods

    /// Send a transaction with signers and proper error handling
    async fn send_transaction_with_signers(
        &self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<Signature> {
        let recent_blockhash = self.rpc_client
            .get_latest_blockhash()
            .map_err(USDvError::SolanaClientError)?;
        
        let mut transaction = Transaction::new_with_payer(
            instructions,
            Some(&signers[0].pubkey()),
        );
        
        transaction.sign(signers, recent_blockhash);
        
        // Use retry mechanism for robustness
        let commitment = if self.client_config.use_confirmed_commitment {
            CommitmentConfig::confirmed()
        } else {
            CommitmentConfig::processed()
        };

        usdv_utils::send_and_confirm_transaction_with_retries(
            &self.rpc_client,
            &transaction,
            commitment,
            self.client_config.max_retries,
        ).await
    }
}

