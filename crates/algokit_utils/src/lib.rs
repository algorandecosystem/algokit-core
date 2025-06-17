use algod_api::AlgodClient;

use algod_api::TransactionParams;
use algokit_http_client_trait::HttpError;
use algokit_transact::Address;
use algokit_transact::FeeParams;
use algokit_transact::PaymentTransactionFields;
use algokit_transact::SignedTransaction;
use algokit_transact::Transaction;
use algokit_transact::TransactionHeader;
use base64::DecodeError;
use base64::{Engine as _, engine::general_purpose};
use derive_more::Debug;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
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

// TODO: TransactionWithSigner
#[derive(Debug)]
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

type TxnSigner = Arc<dyn Fn(&[Transaction], &[usize]) -> Vec<SignedTransaction> + Send + Sync>;
type TxnSignerGetter = Arc<dyn Fn(Address) -> Option<TxnSigner> + Send + Sync>;

pub struct Composer {
    transactions: Vec<ComposerTxn>,
    algod_client: AlgodClient,
    get_signer_fn: TxnSignerGetter,
    built_group: Option<Vec<Transaction>>,
}

impl Composer {
    pub fn new(algod_client: AlgodClient, get_signer: Option<TxnSignerGetter>) -> Self {
        Composer {
            transactions: Vec::new(),
            algod_client: algod_client,
            get_signer_fn: get_signer.unwrap_or(Arc::new(|_| None)),
            built_group: None,
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
            get_signer_fn: Arc::new(|_| None),
            built_group: None,
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

    pub async fn build(&mut self) -> Result<&mut Self, HttpError> {
        let suggested_params = self.get_suggested_params().await?;

        let default_header = TransactionHeader {
            fee: Some(suggested_params.fee),
            genesis_id: Some(suggested_params.genesis_id),
            genesis_hash: Some(
                general_purpose::STANDARD
                    .decode(suggested_params.genesis_hash)
                    .map_err(|e: DecodeError| {
                        HttpError::HttpError(format!("Failed to decode genesis hash: {}", e))
                    })?
                    .try_into()
                    .map_err(|e| {
                        HttpError::HttpError(format!("Failed to convert genesis hash: {:?}", e))
                    })?,
            ),
            // The rest of these fields are set further down per txn
            first_valid: 0,
            last_valid: 0,
            sender: Address::default(),
            rekey_to: None,
            note: None,
            lease: None,
            group: None,
        };

        self.built_group = Some(
            self.transactions
                .iter()
                .map(|composer_txn| {
                    let alread_formed_txn = matches!(composer_txn, ComposerTxn::Transaction(_));

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

                    if !alread_formed_txn {
                        let common_params = composer_txn.common_params();
                        let header = transaction.header_mut();

                        header.sender = common_params.sender;
                        header.rekey_to = common_params.rekey_to;
                        header.note = common_params.note;
                        header.lease = common_params.lease;

                        transaction
                            .assign_fee(FeeParams {
                                fee_per_byte: suggested_params.fee,
                                min_fee: suggested_params.min_fee,
                                extra_fee: common_params.extra_fee,
                                max_fee: common_params.max_fee,
                            })
                            .map_err(|e| HttpError::HttpError(e.to_string()))?;
                    }

                    Ok(transaction)
                })
                .collect::<Result<Vec<Transaction>, HttpError>>()?,
        );

        Ok(self)
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
        assert!(composer.add_payment(payment_params).is_ok());
        assert!(composer.build().await.is_ok());
        assert!(composer.built_group().is_some());
    }
}
