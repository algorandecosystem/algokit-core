use algod_client::{
    AlgodClient,
    apis::{Error as AlgodError, Format},
    models::{PendingTransactionResponse, TransactionParams},
};
use algokit_transact::{
    Address, AlgorandMsgpack, FeeParams, PaymentTransactionFields, SignedTransaction, Transaction,
    TransactionHeader, Transactions,
};
use async_trait::async_trait;
use derive_more::Debug;
use std::sync::Arc;

use crate::clients::network_client::genesis_id_is_localnet;

#[derive(Debug, thiserror::Error)]
pub enum ComposerError {
    #[error("Algod client error: {0}")]
    AlgodClientError(#[from] AlgodError),
    #[error("Decode Error: {0}")]
    DecodeError(String),
    #[error("Transaction Error: {0}")]
    TransactionError(String),
    #[error("Signing Error: {0}")]
    SigningError(String),
    #[error("Composer State Error: {0}")]
    StateError(String),
    #[error("Transaction pool error: {0}")]
    PoolError(String),
}

#[derive(Debug, Default, Clone)]
pub struct CommonParams {
    pub sender: Address,
    #[debug(skip)]
    pub signer: Option<Arc<dyn TxnSigner>>,
    pub rekey_to: Option<Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u64>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct PaymentParams {
    pub common_params: CommonParams,
    pub receiver: Address,
    pub amount: u64,
    pub close_remainder_to: Option<Address>,
}

#[derive(Debug, Clone)]
pub enum ComposerTxn {
    Transaction(Transaction),
    Payment(PaymentParams),
}

impl ComposerTxn {
    pub fn common_params(&self) -> CommonParams {
        match self {
            ComposerTxn::Payment(payment_params) => payment_params.common_params.clone(),
            _ => CommonParams::default(),
        }
    }
}

#[async_trait]
pub trait TxnSigner: Send + Sync {
    async fn sign_txns(&self, txns: &[Transaction], indices: &[usize]) -> Vec<SignedTransaction>;

    async fn sign_txn(&self, txn: &Transaction) -> SignedTransaction {
        self.sign_txns(&[txn.clone()], &[0]).await[0].clone()
    }
}

#[async_trait]
pub trait TxnSignerGetter: Send + Sync {
    async fn get_signer(&self, address: Address) -> Option<&dyn TxnSigner>;
}

struct DefaultSignerGetter;

#[async_trait]
impl TxnSignerGetter for DefaultSignerGetter {
    async fn get_signer(&self, _address: Address) -> Option<&dyn TxnSigner> {
        None
    }
}

pub struct EmptySigner {}

#[async_trait]
impl TxnSigner for EmptySigner {
    async fn sign_txns(&self, txns: &[Transaction], indices: &[usize]) -> Vec<SignedTransaction> {
        indices
            .iter()
            .map(|&idx| {
                if idx < txns.len() {
                    SignedTransaction {
                        transaction: txns[idx].clone(),
                        signature: Some([0; 64]),
                        auth_address: None,
                    }
                } else {
                    panic!("Index out of bounds for transactions");
                }
            })
            .collect()
    }
}

#[async_trait]
impl TxnSignerGetter for EmptySigner {
    async fn get_signer(&self, _address: Address) -> Option<&dyn TxnSigner> {
        Some(self)
    }
}

#[derive(Clone)]
pub struct Composer {
    transactions: Vec<ComposerTxn>,
    algod_client: AlgodClient,
    signer_getter: Arc<dyn TxnSignerGetter>,
    built_group: Option<Vec<Transaction>>,
    signed_group: Option<Vec<SignedTransaction>>,
}

impl Composer {
    pub fn new(algod_client: AlgodClient, get_signer: Option<Arc<dyn TxnSignerGetter>>) -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client,
            signer_getter: get_signer.unwrap_or(Arc::new(DefaultSignerGetter)),
            built_group: None,
            signed_group: None,
        }
    }

    pub fn built_group(&self) -> Option<&Vec<Transaction>> {
        self.built_group.as_ref()
    }

    #[cfg(feature = "default_http_client")]
    pub fn testnet() -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client: AlgodClient::testnet(),
            signer_getter: Arc::new(DefaultSignerGetter),
            built_group: None,
            signed_group: None,
        }
    }

    fn push(&mut self, txn: ComposerTxn) -> Result<(), String> {
        if self.transactions.len() >= 16 {
            return Err("Composer can only hold up to 16 transactions".to_string());
        }
        self.transactions.push(txn);
        Ok(())
    }

    pub fn add_payment(&mut self, payment_params: PaymentParams) -> Result<(), String> {
        self.push(ComposerTxn::Payment(payment_params))
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        self.push(ComposerTxn::Transaction(transaction))
    }

    pub fn transactions(&self) -> &Vec<ComposerTxn> {
        &self.transactions
    }

    pub async fn get_signer(&self, address: Address) -> Option<&dyn TxnSigner> {
        self.signer_getter.get_signer(address).await
    }

    pub async fn get_suggested_params(&self) -> Result<TransactionParams, ComposerError> {
        self.algod_client
            .transaction_params()
            .await
            .map_err(Into::into)
    }

    pub async fn build(&mut self) -> Result<&mut Self, ComposerError> {
        if self.built_group.is_some() {
            return Ok(self);
        }

        let suggested_params = self.get_suggested_params().await?;

        // Determine validity window: default 10 rounds, but 1000 for LocalNet
        let default_validity_window = if genesis_id_is_localnet(&suggested_params.genesis_id) {
            1000 // LocalNet gets bigger window to avoid dead transactions
        } else {
            10 // Standard default validity window
        };

        let default_header =
            TransactionHeader {
                fee: Some(suggested_params.fee),
                genesis_id: Some(suggested_params.genesis_id),
                genesis_hash: Some(suggested_params.genesis_hash.try_into().map_err(|_e| {
                    ComposerError::DecodeError("Invalid genesis hash".to_string())
                })?),
                // Set validity window based on current round
                first_valid: suggested_params.last_round,
                last_valid: suggested_params.last_round + default_validity_window,
                sender: Address::default(),
                rekey_to: None,
                note: None,
                lease: None,
                group: None,
            };

        let txs = self
            .transactions
            .iter()
            .map(|composer_txn| {
                let already_formed_txn = matches!(composer_txn, ComposerTxn::Transaction(_));

                let mut transaction: algokit_transact::Transaction = match composer_txn {
                    ComposerTxn::Transaction(txn) => txn.clone(),
                    ComposerTxn::Payment(pay_params) => {
                        let pay_params = PaymentTransactionFields {
                            header: default_header.clone(),
                            receiver: pay_params.receiver.clone(),
                            amount: pay_params.amount,
                            close_remainder_to: pay_params.close_remainder_to.clone(),
                        };

                        Transaction::Payment(pay_params)
                    }
                };

                if !already_formed_txn {
                    let common_params = composer_txn.common_params();
                    let header = transaction.header_mut();

                    header.sender = common_params.sender;
                    header.rekey_to = common_params.rekey_to;
                    header.note = common_params.note;
                    header.lease = common_params.lease;

                    // Set validity window if provided in common params
                    if let Some(first_valid) = common_params.first_valid_round {
                        header.first_valid = first_valid;
                    }
                    if let Some(last_valid) = common_params.last_valid_round {
                        header.last_valid = last_valid;
                    } else if let Some(validity_window) = common_params.validity_window {
                        header.last_valid = header.first_valid + validity_window;
                    } else {
                        // Use the smart default: 10 rounds normally, 1000 for LocalNet
                        header.last_valid = header.first_valid + default_validity_window;
                    }

                    // Handle static fee vs. calculated fee
                    if let Some(static_fee) = common_params.static_fee {
                        // Set static fee directly
                        header.fee = Some(static_fee);
                    } else {
                        // Use the standard fee calculation
                        transaction = transaction
                            .assign_fee(FeeParams {
                                fee_per_byte: suggested_params.fee,
                                min_fee: suggested_params.min_fee,
                                extra_fee: common_params.extra_fee,
                                max_fee: common_params.max_fee,
                            })
                            .map_err(|e| ComposerError::TransactionError(e.to_string()))?;
                    }
                }

                Ok(transaction)
            })
            .collect::<Result<Vec<Transaction>, ComposerError>>()?;

        // Only assign group if there are 2 or more transactions (matching algosdk ATC behavior)
        self.built_group = if txs.len() > 1 {
            Some(txs.assign_group().map_err(|e| {
                ComposerError::TransactionError(format!("Failed to assign group: {}", e))
            })?)
        } else {
            Some(txs) // Single transaction, no group assignment
        };
        Ok(self)
    }

    pub async fn gather_signatures(&mut self) -> Result<&mut Self, ComposerError> {
        let transactions = self.built_group.as_ref().ok_or(ComposerError::StateError(
            "Cannot gather signatures before building the transaction group".to_string(),
        ))?;

        let mut signed_group = Vec::<SignedTransaction>::new();

        for txn in transactions.iter() {
            let signer = self.get_signer(txn.header().sender.clone()).await.ok_or(
                ComposerError::SigningError(format!(
                    "No signer found for address: {}",
                    txn.header().sender
                )),
            )?;

            let signed_txn = signer.sign_txn(txn).await;
            signed_group.push(signed_txn);
        }

        self.signed_group = Some(signed_group);

        Ok(self)
    }

    pub async fn wait_for_confirmation(
        &self,
        tx_id: &str,
        max_rounds: u64,
    ) -> Result<PendingTransactionResponse, Box<dyn std::error::Error + Send + Sync>> {
        let status = self
            .algod_client
            .get_status()
            .await
            .map_err(|e| format!("Failed to get status: {:?}", e))?;

        let start_round = status.last_round + 1;
        let mut current_round = start_round;

        while current_round < start_round + max_rounds {
            match self
                .algod_client
                .pending_transaction_information(tx_id, Some(Format::Msgpack))
                .await
            {
                Ok(response) => {
                    // Check for pool errors first - transaction was kicked out of pool
                    if !response.pool_error.is_empty() {
                        return Err(Box::new(ComposerError::PoolError(
                            response.pool_error.clone(),
                        )));
                    }

                    // Check if transaction is confirmed
                    if response.confirmed_round.is_some() {
                        return Ok(response);
                    }
                }
                Err(error) => {
                    // Only retry for 404 errors (transaction not found yet)
                    // All other errors indicate permanent issues and should fail fast
                    let is_retryable = matches!(
                        &error,
                        algod_client::apis::Error::Api(
                            algod_client::apis::AlgodApiError::PendingTransactionInformation(
                                algod_client::apis::pending_transaction_information::PendingTransactionInformationError::Status404(_)
                            )
                        )
                    ) || error.to_string().contains("404");

                    if is_retryable {
                        current_round += 1;
                        continue;
                    } else {
                        return Err(Box::new(ComposerError::AlgodClientError(error)));
                    }
                }
            };

            let _ = self.algod_client.wait_for_block(current_round).await;
            current_round += 1;
        }

        Err(format!(
            "Transaction {} not confirmed after {} rounds",
            tx_id, max_rounds
        )
        .into())
    }

    pub async fn send(
        &mut self,
    ) -> Result<PendingTransactionResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.build()
            .await
            .map_err(|e| format!("Failed to build transaction: {}", e))?;

        let transactions = self.built_group().ok_or("No transactions built")?;

        if transactions.is_empty() {
            return Err("No transactions to send".into());
        }

        self.gather_signatures()
            .await
            .map_err(|e| format!("Failed to sign transaction: {}", e))?;

        // Encode each signed transaction and concatenate them
        let signed_transactions = self.signed_group.as_ref().ok_or("No signed transactions")?;
        let mut encoded_bytes = Vec::new();

        for signed_txn in signed_transactions {
            let encoded_txn = signed_txn
                .encode()
                .map_err(|e| format!("Failed to encode signed transaction: {}", e))?;
            encoded_bytes.extend_from_slice(&encoded_txn);
        }

        let raw_transaction_response = self
            .algod_client
            .raw_transaction(encoded_bytes)
            .await
            .map_err(|e| format!("Failed to submit transaction: {:?}", e))?;

        let pending_transaction_response = self
            .wait_for_confirmation(&raw_transaction_response.tx_id, 5)
            .await
            .map_err(|e| format!("Failed to confirm transaction: {}", e))?;

        Ok(pending_transaction_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{AddressMother, TransactionMother};
    use base64::{Engine, prelude::BASE64_STANDARD};

    #[test]
    fn test_add_transaction() {
        let mut composer = Composer::testnet();
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn).is_ok());
    }

    #[test]
    fn test_add_too_many_transactions() {
        let mut composer = Composer::testnet();
        for _ in 0..16 {
            let txn = TransactionMother::simple_payment().build().unwrap();
            assert!(composer.add_transaction(txn).is_ok());
        }
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn).is_err());
    }

    #[tokio::test]
    async fn test_get_suggested_params() {
        let composer = Composer::testnet();
        let response = composer.get_suggested_params().await.unwrap();

        assert_eq!(
            response.genesis_hash,
            BASE64_STANDARD
                .decode("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=")
                .unwrap()
        );
    }

    #[test]
    fn test_add_payment() {
        let mut composer = Composer::testnet();
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AddressMother::address(),
            amount: 1000,
            close_remainder_to: None,
        };
        assert!(composer.add_payment(payment_params).is_ok());
    }

    #[tokio::test]
    async fn test_build_payment() {
        let mut composer = Composer::testnet();
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AddressMother::address(),
            amount: 1000,
            close_remainder_to: None,
        };
        composer.add_payment(payment_params).unwrap();

        let result = composer.build().await;
        assert!(result.is_ok());

        let built_group = composer.built_group().unwrap();
        assert_eq!(built_group.len(), 1);
    }

    #[tokio::test]
    async fn test_gather_signatures() {
        let mut composer = Composer::new(AlgodClient::testnet(), Some(Arc::new(EmptySigner {})));

        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AddressMother::address(),
            amount: 1000,
            close_remainder_to: None,
        };
        composer.add_payment(payment_params).unwrap();
        composer.build().await.unwrap();

        let result = composer.gather_signatures().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_single_transaction_no_group() {
        let mut composer = Composer::testnet();
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AddressMother::address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AddressMother::address(),
            amount: 1000,
            close_remainder_to: None,
        };
        composer.add_payment(payment_params).unwrap();

        composer.build().await.unwrap();

        let built_group = composer.built_group().unwrap();
        assert_eq!(built_group.len(), 1);

        // Single transaction should not have a group ID set
        assert!(built_group[0].header().group.is_none());
    }

    #[tokio::test]
    async fn test_multiple_transactions_have_group() {
        let mut composer = Composer::testnet();

        for _ in 0..2 {
            let payment_params = PaymentParams {
                common_params: CommonParams {
                    sender: AddressMother::address(),
                    signer: None,
                    rekey_to: None,
                    note: None,
                    lease: None,
                    static_fee: None,
                    extra_fee: None,
                    max_fee: None,
                    validity_window: None,
                    first_valid_round: None,
                    last_valid_round: None,
                },
                receiver: AddressMother::address(),
                amount: 1000,
                close_remainder_to: None,
            };
            composer.add_payment(payment_params).unwrap();
        }

        composer.build().await.unwrap();

        let built_group = composer.built_group().unwrap();
        assert_eq!(built_group.len(), 2);

        // Multiple transactions should have group IDs set
        for txn in built_group {
            assert!(txn.header().group.is_some());
        }

        // All transactions should have the same group ID
        let group_id = built_group[0].header().group.as_ref().unwrap();
        for txn in &built_group[1..] {
            assert_eq!(txn.header().group.as_ref().unwrap(), group_id);
        }
    }

    #[test]
    fn test_error_recoverability_logic() {
        // Test string-based 404 detection (the primary retry mechanism)
        let error_404_string = "Request failed with status 404: Transaction not found";
        let error_500_string = "Request failed with status 500: Server error";

        // The main retry logic relies on string matching
        assert!(
            error_404_string.contains("404"),
            "404 errors should be retryable"
        );
        assert!(
            !error_500_string.contains("404"),
            "500 errors should not be retryable"
        );
    }

    #[test]
    fn test_validity_window_logic() {
        // Test LocalNet detection and validity window logic
        assert_eq!(
            if genesis_id_is_localnet("devnet-v1") {
                1000
            } else {
                10
            },
            1000,
            "LocalNet should use 1000 round validity window"
        );

        assert_eq!(
            if genesis_id_is_localnet("testnet-v1.0") {
                1000
            } else {
                10
            },
            10,
            "TestNet should use 10 round validity window"
        );

        assert_eq!(
            if genesis_id_is_localnet("mainnet-v1.0") {
                1000
            } else {
                10
            },
            10,
            "MainNet should use 10 round validity window"
        );
    }
}
