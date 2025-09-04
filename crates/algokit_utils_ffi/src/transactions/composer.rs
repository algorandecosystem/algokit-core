use std::sync::Arc;

use crate::transactions::common::{
    RustTransactionSignerGetterFromFfi, TransactionSignerGetter, UtilsError,
};
use algod_client::AlgodClient as RustAlgodClient;
use algokit_http_client::HttpClient;
use algokit_utils::transactions::composer::Composer as RustComposer;
use tokio::sync::Mutex;

#[derive(uniffi::Object)]
pub struct AlgodClient {
    inner_algod_client: Mutex<RustAlgodClient>,
}

#[uniffi::export]
impl AlgodClient {
    #[uniffi::constructor]
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        let algod_client = RustAlgodClient::new(http_client);
        AlgodClient {
            inner_algod_client: Mutex::new(algod_client),
        }
    }
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

    pub fn add_payment(&self, params: super::payment::PaymentParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_payment(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn add_asset_freeze(&self, params: super::asset_freeze::AssetFreezeParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_asset_freeze(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub async fn send(&self) -> Result<Vec<String>, UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        let result = composer
            .send(None)
            .await
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })?;
        Ok(result.transaction_ids)
    }

    pub async fn build(&self) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .build(None)
            .await
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })?;

        Ok(())
    }
}
