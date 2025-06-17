use std::sync::Arc;

use algokit_http_client_trait::{HttpClient, HttpError};

#[cfg(feature = "default_http_client")]
use algokit_http_client_trait::DefaultHttpClient;

pub struct AlgodClient {
    http_client: Arc<dyn HttpClient>,
}

impl AlgodClient {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        AlgodClient { http_client }
    }

    #[cfg(feature = "default_http_client")]
    pub fn testnet() -> Self {
        AlgodClient {
            http_client: Arc::new(DefaultHttpClient::new(
                "https://testnet-api.4160.nodely.dev",
            )),
        }
    }

    pub async fn get_suggested_params(&self) -> Result<String, HttpError> {
        let path = "/v2/transactions/params".to_string();
        self.http_client.json(path).await
    }
}
