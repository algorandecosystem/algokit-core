use std::sync::Arc;

use crate::{AlgoConfig, ClientManager, Composer, TxnSigner, TxnSignerGetter};
use algod_client::AlgodClient;
use algod_client_tests::{NetworkType, TestAccount, TestAccountConfig, TestAccountManager};
use algokit_transact::{Address, SignedTransaction, Transaction};
use async_trait::async_trait;

pub struct AlgorandFixture {
    config: AlgoConfig,
    context: Option<AlgorandTestContext>,
}

pub struct AlgorandTestContext {
    pub algod: AlgodClient,

    pub composer: Composer,

    pub test_account: TestAccount,

    pub account_manager: TestAccountManager,
}

#[derive(Debug)]
pub struct TransactionResult {
    pub transaction: Transaction,
    pub tx_id: String,
    pub signed_bytes: Vec<u8>,
}

// TODO: We will need to implement account signer to replace this struct
// It is only here initially to get the tests working
#[derive(Clone)]
pub struct TestAccountSigner {
    pub test_account: TestAccount,
}

impl TestAccountSigner {
    fn sign_single_transaction(&self, transaction: &Transaction) -> SignedTransaction {
        // For now, let's just use the sign_transaction method and decode the result
        // This is not the most efficient, but it will work
        let signed_bytes = self
            .test_account
            .sign_transaction(transaction)
            .expect("Failed to sign transaction");

        // The signed_bytes contain a full SignedTransaction encoded as msgpack
        // We'll decode it to get the SignedTransaction
        use algokit_transact::AlgorandMsgpack;
        SignedTransaction::decode(&signed_bytes).expect("Failed to decode signed transaction")
    }
}

#[async_trait]
impl TxnSigner for TestAccountSigner {
    async fn sign_txns(&self, txns: &[Transaction], indices: &[usize]) -> Vec<SignedTransaction> {
        indices
            .iter()
            .map(|&idx| {
                if idx < txns.len() {
                    // We'll create a SignedTransaction that matches what TestAccount.sign_transaction does
                    // but without encoding it to bytes
                    self.sign_single_transaction(&txns[idx])
                } else {
                    panic!("Index out of bounds for transactions");
                }
            })
            .collect()
    }
}

#[async_trait]
impl TxnSignerGetter for TestAccountSigner {
    async fn get_signer(&self, address: Address) -> Option<&dyn TxnSigner> {
        let test_account_address = self
            .test_account
            .address()
            .expect("Failed to get test account address");
        if address == test_account_address {
            Some(self)
        } else {
            None
        }
    }
}

impl AlgorandFixture {
    pub fn new(config: AlgoConfig) -> Self {
        Self {
            config,
            context: None,
        }
    }

    pub fn context(
        &self,
    ) -> Result<&AlgorandTestContext, Box<dyn std::error::Error + Send + Sync>> {
        self.context
            .as_ref()
            .ok_or_else(|| "Context not initialized; make sure to call fixture.new_scope() before accessing context.".into())
    }

    pub async fn new_scope(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let algod = ClientManager::get_algod_client(&self.config.algod_config);

        let mut account_manager = TestAccountManager::new(algod.clone());

        let test_account_config = TestAccountConfig {
            initial_funds: 10_000_000,
            suppress_log: false,
            network_type: NetworkType::LocalNet,
            funding_note: Some("AlgorandFixture test account".to_string()),
        };

        let test_account = account_manager
            .get_test_account(Some(test_account_config))
            .await
            .map_err(|e| format!("Failed to create test account: {}", e))?;

        let signer = TestAccountSigner {
            test_account: test_account.clone(),
        };

        let composer = Composer::new(algod.clone(), Some(Arc::new(signer)));

        self.context = Some(AlgorandTestContext {
            algod,
            composer,
            test_account,
            account_manager,
        });

        Ok(())
    }

    pub async fn generate_account(
        &mut self,
        config: Option<TestAccountConfig>,
    ) -> Result<TestAccount, Box<dyn std::error::Error + Send + Sync>> {
        let context = self
            .context
            .as_mut()
            .ok_or_else(|| "Context not initialized; call new_scope() first")?;

        let account = context
            .account_manager
            .get_test_account(config)
            .await
            .map_err(|e| format!("Failed to generate account: {}", e))?;

        Ok(account)
    }
}

pub async fn algorand_fixture() -> Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>>
{
    let config = ClientManager::get_config_from_environment_or_localnet();
    Ok(AlgorandFixture::new(config))
}

pub async fn algorand_fixture_with_config(
    config: AlgoConfig,
) -> Result<AlgorandFixture, Box<dyn std::error::Error + Send + Sync>> {
    Ok(AlgorandFixture::new(config))
}
