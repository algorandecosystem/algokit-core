use std::sync::Arc;

use algokit_transact::SignedTransaction as RustSignedTransaction;
use algokit_transact::Transaction as RustTransaction;
use algokit_transact_ffi::{SignedTransaction, Transaction};
use algokit_utils::transactions::common::TransactionSigner as RustTransactionSigner;
use algokit_utils::transactions::common::TransactionWithSigner as RustTransactionWithSigner;

use async_trait::async_trait;

#[uniffi::export(with_foreign)]
#[async_trait]
pub trait TransactionSigner: Send + Sync {
    async fn sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        indices: Vec<u8>,
    ) -> Result<Vec<SignedTransaction>, String>;

    async fn sign_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<SignedTransaction, String> {
        let result = self.sign_transactions(vec![transaction], vec![0]).await?;
        Ok(result[0].clone())
    }
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
            .sign_transactions(ffi_txns, indices.iter().map(|&i| i as u8).collect())
            .await?;

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
        indices: Vec<u8>,
    ) -> Result<Vec<SignedTransaction>, String> {
        let rust_txns: Result<Vec<RustTransaction>, _> =
            transactions.into_iter().map(|t| t.try_into()).collect();
        let rust_txns = rust_txns.map_err(|e| format!("Failed to convert transactions: {}", e))?;

        let signed_txns = self
            .rust_signer
            .sign_transactions(
                &rust_txns,
                &indices.iter().map(|&i| i as usize).collect::<Vec<_>>(),
            )
            .await?;

        let ffi_signed_txns: Result<Vec<SignedTransaction>, _> =
            signed_txns.into_iter().map(|st| st.try_into()).collect();
        ffi_signed_txns.map_err(|e| format!("Failed to convert signed transactions: {}", e))
    }
}

pub struct TransactionWithSigner {
    pub transaction: Transaction,
    pub signer: Arc<dyn TransactionSigner>,
}

impl TryFrom<TransactionWithSigner> for RustTransactionWithSigner {
    type Error = String;

    fn try_from(value: TransactionWithSigner) -> Result<Self, Self::Error> {
        let rust_txn: RustTransaction = value
            .transaction
            .try_into()
            .map_err(|e| format!("Failed to convert transaction: {}", e))?;

        Ok(RustTransactionWithSigner {
            transaction: rust_txn,
            signer: Arc::new(RustTransactionSignerFromFfi {
                ffi_signer: value.signer,
            }),
        })
    }
}

impl TryFrom<RustTransactionWithSigner> for TransactionWithSigner {
    type Error = String;

    fn try_from(value: RustTransactionWithSigner) -> Result<Self, Self::Error> {
        let ffi_txn: Transaction = value
            .transaction
            .try_into()
            .map_err(|e| format!("Failed to convert transaction: {}", e))?;

        Ok(TransactionWithSigner {
            transaction: ffi_txn,
            signer: Arc::new(FfiTransactionSignerFromRust {
                rust_signer: value.signer,
            }),
        })
    }
}
