use algod_api::AlgodClient;

use algod_api::TransactionParams;
use algokit_http_client_trait::HttpError;
use algokit_transact::Address;
use algokit_transact::SignedTransaction;
use algokit_transact::Transaction;
use derive_more::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct CommonParams {
    pub sender: Address,
    #[debug(skip)]
    pub signer: Option<TxnSigner>,
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

#[derive(Debug)]
pub struct PaymentParams {
    pub common_params: CommonParams,
    pub receiver: Address,
    pub amount: u64,
    pub close_remainder_to: Option<Address>,
}

#[derive(Debug)]
pub enum ComposerTxn {
    Transaction(Transaction),
    SignedTransaction(SignedTransaction),
    Payment(PaymentParams),
}

type TxnSigner = Arc<dyn Fn(&[Transaction], &[usize]) -> Vec<SignedTransaction> + Send + Sync>;
type TxnSignerGetter = Arc<dyn Fn(Address) -> Option<TxnSigner> + Send + Sync>;

pub struct Composer {
    transactions: Vec<ComposerTxn>,
    algod_client: AlgodClient,
    get_signer_fn: TxnSignerGetter,
}

impl Composer {
    pub fn new(algod_client: AlgodClient, get_signer: Option<TxnSignerGetter>) -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client: algod_client,
            get_signer_fn: get_signer.unwrap_or(Arc::new(|_| None)),
        }
    }

    #[cfg(feature = "default_http_client")]
    pub fn testnet() -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client: AlgodClient::testnet(),
            get_signer_fn: Arc::new(|_| None),
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

    pub fn get_signer(&self, address: Address) -> Option<TxnSigner> {
        (self.get_signer_fn)(address)
    }

    // TODO: Use Fn defined in ComposerConfig
    pub async fn get_suggested_params(&self) -> Result<TransactionParams, HttpError> {
        self.algod_client.transaction_params().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{AddressMother, TransactionMother};
    use tokio;

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
            "SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI="
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
}
