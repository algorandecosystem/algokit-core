use std::sync::Arc;

use crate::transactions::common::{
    RustTransactionSignerGetterFromFfi, TransactionSignerGetter, UtilsError,
};
use algod_client::AlgodClient as RustAlgodClient;
use algokit_http_client::HttpClient;
use algokit_utils::transactions::{ComposerParams, composer::Composer as RustComposer};
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

// NOTE: This struct is a temporary placeholder until we have a proper algod_api_ffi crate with the fully typed response
#[derive(uniffi::Record)]
pub struct TempSendResponse {
    pub transaction_ids: Vec<String>,
    pub app_ids: Vec<Option<u64>>,
}

#[derive(uniffi::Object)]
pub struct Composer {
    inner_composer: Mutex<RustComposer>,
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait ComposerTrait: Send + Sync {
    fn add_payment(&self, params: super::payment::PaymentParams) -> Result<(), UtilsError>;
    fn add_app_create(&self, params: super::app_call::AppCreateParams) -> Result<(), UtilsError>;
    fn add_app_call(&self, params: super::app_call::AppCallParams) -> Result<(), UtilsError>;
    fn add_app_call_method_call(
        &self,
        params: super::app_call::AppCallMethodCallParams,
    ) -> Result<(), UtilsError>;
    async fn send(&self) -> Result<TempSendResponse, UtilsError>;
    async fn build(&self) -> Result<(), UtilsError>;
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

        let rust_composer = RustComposer::new(ComposerParams {
            algod_client: Arc::new(rust_algod_client.clone()),
            signer_getter: Arc::new(rust_signer_getter),
            composer_config: None,
        });

        Composer {
            inner_composer: Mutex::new(rust_composer),
        }
    }
}

#[uniffi::export]
#[async_trait::async_trait]
impl ComposerTrait for Composer {
    fn add_payment(&self, params: super::payment::PaymentParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_payment(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    async fn send(&self) -> Result<TempSendResponse, UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        let result = composer
            .send(None)
            .await
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })?;
        Ok(TempSendResponse {
            transaction_ids: result.transaction_ids,
            app_ids: result.confirmations.iter().map(|c| c.app_id).collect(),
        })
    }

    async fn build(&self) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer.build().await.map_err(|e| UtilsError::UtilsError {
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn add_app_create(&self, params: super::app_call::AppCreateParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_create(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    fn add_app_call(&self, params: super::app_call::AppCallParams) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_call(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    fn add_app_call_method_call(
        &self,
        params: super::app_call::AppCallMethodCallParams,
    ) -> Result<(), UtilsError> {
        let mut composer = self.inner_composer.blocking_lock();
        composer
            .add_app_call_method_call(params.try_into()?)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }
}
