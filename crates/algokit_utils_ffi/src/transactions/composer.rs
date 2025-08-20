use std::sync::Arc;

use crate::transactions::common::{RustTransactionSignerGetterFromFfi, TransactionSignerGetter};
use algod_client::AlgodClient as RustAlgodClient;
use algokit_utils::transactions::composer::Composer as RustComposer;
use ffi_mutex::FfiMutex as Mutex;

#[derive(uniffi::Object)]
pub struct AlgodClient {
    inner_algod_client: Mutex<RustAlgodClient>,
}

#[derive(uniffi::Object)]
pub struct Composer {
    inner_composer: Mutex<RustComposer>,
}

#[uniffi::export]
impl Composer {
    #[uniffi::constructor]
    pub fn new(
        algod_client: Arc<AlgodClient>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Self {
        let rust_signer_getter = RustTransactionSignerGetterFromFfi {
            ffi_signer_getter: signer_getter,
        };

        let rust_algod_client = algod_client.inner_algod_client.blocking_lock();

        let rust_composer = RustComposer::new(
            Arc::new(rust_algod_client.clone()),
            Arc::new(rust_signer_getter),
        );

        Composer {
            inner_composer: Mutex::new(rust_composer),
        }
    }

    pub fn add_payment(&self, params: super::payment::PaymentParams) -> Result<(), String> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_payment(params.try_into()?)
            .map_err(|e| e.to_string())
    }

    pub async fn send(&self) -> Result<Vec<String>, String> {
        let mut composer = self.inner_composer.blocking_lock();
        let result = composer.send(None).await.map_err(|e| e.to_string())?;
        Ok(result.transaction_ids)
    }
}
