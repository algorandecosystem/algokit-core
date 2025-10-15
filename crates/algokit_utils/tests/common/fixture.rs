use super::indexer_helpers::wait_for_indexer_transaction;
use crate::common::logging::init_test_logging;
use algod_client::AlgodClient;
use algokit_transact::{Address, Transaction};
use algokit_utils::clients::SigningAccount;
use algokit_utils::clients::account_manager::EnsureFundedParams;
use algokit_utils::clients::algorand_client::AlgorandClientParams;
use algokit_utils::transactions::TransactionComposerConfig;
use algokit_utils::{AlgoConfig, AlgorandClient, ClientManager, PaymentParams};
use indexer_client::IndexerClient;
use kmd_client::KmdClient;
use rstest::*;
use std::sync::Arc;

/// Test account configuration
#[derive(Debug, Clone)]
pub struct TestAccountConfig {
    /// Initial funding amount in microALGOs (default: 10 ALGO = 10,000,000 microALGOs)
    pub initial_funds: u64,
    /// Optional note for funding transaction
    pub funding_note: Option<String>,
}

impl Default for TestAccountConfig {
    fn default() -> Self {
        Self {
            initial_funds: 10_000_000, // 10 ALGO
            funding_note: None,
        }
    }
}

pub struct AlgorandFixture {
    pub algod: Arc<AlgodClient>,
    pub indexer: Arc<IndexerClient>,
    pub kmd: Arc<KmdClient>,
    pub algorand_client: Arc<AlgorandClient>,
    pub test_account: SigningAccount,
}

pub type AlgorandFixtureResult = Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct TransactionResult {
    pub transaction: Transaction,
    pub tx_id: String,
    pub signed_bytes: Vec<u8>,
}

impl AlgorandFixture {
    pub async fn new(
        params: &AlgorandClientParams,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let algod =
            Arc::new(ClientManager::get_algod_client(&params.client_config.algod_config).unwrap());
        let indexer = Arc::new(
            ClientManager::get_indexer_client(
                &params.client_config.indexer_config.clone().unwrap(),
            )
            .unwrap(),
        );
        let kmd = Arc::new(
            ClientManager::get_kmd_client(
                params
                    .client_config
                    .kmd_config
                    .as_ref()
                    .expect("KMD config required for localnet tests"),
            )
            .unwrap(),
        );

        #[allow(clippy::arc_with_non_send_sync)]
        let algorand_client = Arc::new(AlgorandClient::new(params));

        let test_account = Self::generate_account_internal(
            &algorand_client,
            Some(TestAccountConfig {
                initial_funds: 10_000_000,
                funding_note: Some("AlgorandFixture test account".to_string()),
            }),
        )
        .await
        .map_err(|e| format!("Failed to create test account: {}", e))?;

        Ok(Self {
            algod,
            indexer,
            kmd,
            algorand_client,
            test_account,
        })
    }

    async fn generate_account_internal(
        algorand_client: &Arc<AlgorandClient>,
        config: Option<TestAccountConfig>,
    ) -> Result<SigningAccount, Box<dyn std::error::Error + Send + Sync>> {
        // Generate new account using ed25519_dalek
        let test_account = SigningAccount::generate();
        let test_account_address = test_account.account().address();
        let config = config.unwrap_or_default();

        Self::fund_account(algorand_client, &test_account_address, &config).await?;

        algorand_client
            .set_signer(test_account_address, Arc::new(test_account.clone()))
            .await;

        Ok(test_account)
    }

    async fn fund_account(
        algorand_client: &Arc<AlgorandClient>,
        account_address: &Address,
        config: &TestAccountConfig,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut account_manager = algorand_client.account_manager().lock().await;
        account_manager
            .ensure_funded_from_environment(
                account_address,
                &EnsureFundedParams {
                    min_spending_balance: config.initial_funds,
                    note: Some(
                        config
                            .funding_note
                            .clone()
                            .unwrap_or_else(|| "Funding test account".to_string())
                            .as_bytes()
                            .to_vec(),
                    ),
                    ..Default::default()
                },
                None,
            )
            .await?;

        Ok(())
    }

    pub async fn generate_account(
        &self,
        config: Option<TestAccountConfig>,
    ) -> Result<SigningAccount, Box<dyn std::error::Error + Send + Sync>> {
        Self::generate_account_internal(&self.algorand_client, config).await
    }
}

impl AlgorandFixture {
    /// Waits for a transaction to appear in the indexer
    pub async fn wait_for_indexer_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        wait_for_indexer_transaction(&self.indexer, transaction_id, None).await?;
        Ok(())
    }
}

#[fixture]
pub async fn algorand_fixture(
    #[default(None)] composer_config: Option<TransactionComposerConfig>,
) -> AlgorandFixtureResult {
    let client_config = ClientManager::get_config_from_environment_or_localnet();
    AlgorandFixture::new(&AlgorandClientParams {
        client_config,
        composer_config,
    })
    .await
}
