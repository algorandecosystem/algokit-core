use std::sync::Arc;

use algokit_transact::SignedTransaction as RustSignedTransaction;
use algokit_transact::Transaction as RustTransaction;
use algokit_transact_ffi::{SignedTransaction, Transaction};
use algokit_utils::transactions::common::TransactionSigner as RustTransactionSigner;
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

struct FfiTransactionSigner {
    ffi_signer: Arc<dyn TransactionSigner>,
}

#[async_trait]
impl RustTransactionSigner for FfiTransactionSigner {
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

    async fn sign_transaction(
        &self,
        transaction: &RustTransaction,
    ) -> Result<RustSignedTransaction, String> {
        self.sign_transactions(&[transaction.clone()], &[0])
            .await
            .map(|txns| txns[0].clone())
    }
}
