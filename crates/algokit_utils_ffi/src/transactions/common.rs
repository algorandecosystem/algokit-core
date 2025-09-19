use std::sync::Arc;

use algokit_transact::Address;
use algokit_transact::SignedTransaction as RustSignedTransaction;
use algokit_transact::Transaction as RustTransaction;
use algokit_transact_ffi::{SignedTransaction, Transaction};
use algokit_utils::transactions::common::CommonParams as RustCommonParams;
use algokit_utils::transactions::common::TransactionSigner as RustTransactionSigner;
use algokit_utils::transactions::common::TransactionSignerGetter as RustTransactionSignerGetter;
use algokit_utils::transactions::common::TransactionWithSigner as RustTransactionWithSigner;

use async_trait::async_trait;
use derive_more::Debug;
use snafu::Snafu;

// TODO: implement proper errors
#[derive(Debug, Snafu, uniffi::Error)]
pub enum UtilsError {
    #[snafu(display("UtilsError: {message}"))]
    UtilsError { message: String },
}

#[uniffi::export(with_foreign)]
#[async_trait]
pub trait TransactionSigner: Send + Sync {
    async fn sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        indices: Vec<u32>,
    ) -> Result<Vec<SignedTransaction>, UtilsError>;

    async fn sign_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<SignedTransaction, UtilsError>;
}

struct RustTransactionSignerFromFfi {
    ffi_signer: Arc<dyn TransactionSigner>,
}

#[async_trait]
impl RustTransactionSigner for RustTransactionSignerFromFfi {
    async fn sign_transactions(
        &self,
        transactions: &[RustTransaction],
        indices: &[usize],
    ) -> Result<Vec<RustSignedTransaction>, String> {
        let ffi_txns: Result<Vec<Transaction>, _> = transactions
            .iter()
            .map(|t| t.to_owned().try_into())
            .collect();
        let ffi_txns = ffi_txns.map_err(|e| format!("Failed to convert transactions: {}", e))?;

        let ffi_signed_txns = self
            .ffi_signer
            .sign_transactions(ffi_txns, indices.iter().map(|&i| i as u32).collect())
            .await
            .map_err(|e| e.to_string())?;

        let signed_txns: Result<Vec<RustSignedTransaction>, _> = ffi_signed_txns
            .into_iter()
            .map(|st| st.try_into())
            .collect();
        signed_txns.map_err(|e| format!("Failed to convert signed transactions: {}", e))
    }
}

struct FfiTransactionSignerFromRust {
    rust_signer: Arc<dyn RustTransactionSigner>,
}

#[async_trait]
impl TransactionSigner for FfiTransactionSignerFromRust {
    async fn sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        indices: Vec<u32>,
    ) -> Result<Vec<SignedTransaction>, UtilsError> {
        let rust_txns: Result<Vec<RustTransaction>, _> =
            transactions.into_iter().map(|t| t.try_into()).collect();
        let rust_txns = rust_txns.map_err(|e| UtilsError::UtilsError {
            message: format!("Failed to convert transactions: {}", e),
        })?;

        let signed_txns = self
            .rust_signer
            .sign_transactions(
                &rust_txns,
                &indices.iter().map(|&i| i as usize).collect::<Vec<_>>(),
            )
            .await
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })?;

        Ok(signed_txns.into_iter().map(|st| st.into()).collect())
    }

    async fn sign_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<SignedTransaction, UtilsError> {
        let txns = vec![transaction];
        let indices = vec![0u32];
        let mut signed_txns = self.sign_transactions(txns, indices).await?;
        signed_txns.pop().ok_or(UtilsError::UtilsError {
            message: "No signed transaction returned".to_string(),
        })
    }
}

#[uniffi::export(with_foreign)]
pub trait TransactionSignerGetter: Send + Sync {
    fn get_signer(&self, address: String) -> Option<Arc<dyn TransactionSigner>>;
}

pub struct RustTransactionSignerGetterFromFfi {
    pub ffi_signer_getter: Arc<dyn TransactionSignerGetter>,
}

impl RustTransactionSignerGetter for RustTransactionSignerGetterFromFfi {
    fn get_signer(&self, address: Address) -> Option<Arc<dyn RustTransactionSigner>> {
        self.ffi_signer_getter
            .get_signer(address.to_string())
            .map(|ffi_signer| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer })
                    as Arc<dyn RustTransactionSigner>
            })
    }
}

pub struct FfiTransactionSignerGetterFromRust {
    pub rust_signer_getter: Arc<dyn RustTransactionSignerGetter>,
}

impl TransactionSignerGetter for FfiTransactionSignerGetterFromRust {
    fn get_signer(&self, address: String) -> Option<Arc<dyn TransactionSigner>> {
        self.rust_signer_getter
            .get_signer(address.parse().ok()?)
            .map(|rust_signer| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer }) as Arc<dyn TransactionSigner>
            })
    }
}

pub struct TransactionWithSigner {
    pub transaction: Transaction,
    pub signer: Arc<dyn TransactionSigner>,
}

impl TryFrom<TransactionWithSigner> for RustTransactionWithSigner {
    type Error = UtilsError;

    fn try_from(value: TransactionWithSigner) -> Result<Self, Self::Error> {
        let rust_txn: RustTransaction =
            value
                .transaction
                .try_into()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Failed to convert transaction: {}", e),
                })?;

        Ok(RustTransactionWithSigner {
            transaction: rust_txn,
            signer: Arc::new(RustTransactionSignerFromFfi {
                ffi_signer: value.signer,
            }),
        })
    }
}

impl TryFrom<RustTransactionWithSigner> for TransactionWithSigner {
    type Error = UtilsError;

    fn try_from(value: RustTransactionWithSigner) -> Result<Self, Self::Error> {
        let ffi_txn: Transaction =
            value
                .transaction
                .try_into()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Failed to convert transaction: {}", e),
                })?;

        Ok(TransactionWithSigner {
            transaction: ffi_txn,
            signer: Arc::new(FfiTransactionSignerFromRust {
                rust_signer: value.signer,
            }),
        })
    }
}

#[derive(Debug, uniffi::Record)]
pub struct CommonParams {
    pub sender: String,
    #[debug(skip)]
    #[uniffi(default = None)]
    pub signer: Option<Arc<dyn TransactionSigner>>,
    #[uniffi(default = None)]
    pub rekey_to: Option<String>,
    #[uniffi(default = None)]
    pub note: Option<Vec<u8>>,
    #[uniffi(default = None)]
    pub lease: Option<Vec<u8>>,
    #[uniffi(default = None)]
    pub static_fee: Option<u64>,
    #[uniffi(default = None)]
    pub extra_fee: Option<u64>,
    #[uniffi(default = None)]
    pub max_fee: Option<u64>,
    #[uniffi(default = None)]
    pub validity_window: Option<u64>,
    #[uniffi(default = None)]
    pub first_valid_round: Option<u64>,
    #[uniffi(default = None)]
    pub last_valid_round: Option<u64>,
}

impl From<RustCommonParams> for CommonParams {
    fn from(params: RustCommonParams) -> Self {
        CommonParams {
            sender: params.sender.as_str().to_string(),
            signer: params.signer.map(|rust_signer| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer }) as Arc<dyn TransactionSigner>
            }),
            rekey_to: params.rekey_to.map(|a| a.as_str().to_string()),
            note: params.note,
            lease: params.lease.map(|l| l.to_vec()),
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
        }
    }
}

impl TryFrom<CommonParams> for RustCommonParams {
    type Error = UtilsError;

    fn try_from(params: CommonParams) -> Result<Self, Self::Error> {
        Ok(RustCommonParams {
            sender: params.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {e}"),
            })?,
            signer: params.signer.map(|ffi_signer| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer })
                    as Arc<dyn RustTransactionSigner>
            }),
            rekey_to: params
                .rekey_to
                .map(|a| {
                    a.parse().map_err(|e| UtilsError::UtilsError {
                        message: format!("Invalid rekey address: {e}"),
                    })
                })
                .transpose()?,
            note: params.note,
            lease: params
                .lease
                .map(|l| l.try_into().expect("Invalid lease format")),
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
        })
    }
}
