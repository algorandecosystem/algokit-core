use algokit_transact::Address;
use reqwest::Client;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use snafu::Snafu;
use std::time::Duration;

const DISPENSER_BASE_URL: &str = "https://api.dispenser.algorandfoundation.tools";
const DEFAULT_DISPENSER_REQUEST_TIMEOUT: u64 = 15;
const DISPENSER_ACCESS_TOKEN_KEY: &str = "ALGOKIT_DISPENSER_ACCESS_TOKEN";

/// The TestNet Dispenser API response when funding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispenserFundResponse {
    /// The ID of the transaction that was issued to fund the account.
    pub transaction_id: String,
    /// The number of µAlgo that was funded.
    pub amount: u64,
}

/// The TestNet Dispenser API response when getting the current limit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispenserLimitResponse {
    /// The limit, in µAlgo, that you can currently fund.
    pub amount: u64,
}

/// Errors that can occur when using the TestNet Dispenser API client.
#[derive(Debug, Snafu)]
pub enum DispenserError {
    #[snafu(display(
        "Can't init AlgoKit TestNet Dispenser API client because neither environment variable {DISPENSER_ACCESS_TOKEN_KEY} nor the authToken were provided"
    ))]
    MissingAuthToken,

    #[snafu(display("Error processing dispenser API request: {message}"))]
    ApiError { message: String },

    #[snafu(display("HTTP request failed: {source}"))]
    RequestError { source: reqwest::Error },

    #[snafu(display("Failed to parse response: {source}"))]
    ParseError { source: reqwest::Error },
}

#[derive(Deserialize)]
struct ErrorResponse {
    #[serde(rename = "code")]
    code: Option<String>,
    #[serde(rename = "message")]
    message: Option<String>,
}

#[derive(Serialize)]
struct FundRequest {
    receiver: String,
    amount: u64,
    #[serde(rename = "assetID")]
    asset_id: u64,
}

#[derive(Deserialize)]
struct FundResponseDto {
    #[serde(rename = "txID")]
    tx_id: String,
    amount: u64,
}

#[derive(Serialize)]
struct RefundRequest {
    #[serde(rename = "refundTransactionID")]
    refund_transaction_id: String,
}

#[derive(Deserialize)]
struct LimitResponseDto {
    amount: u64,
}

static ALGO_ASSET_ID: u64 = 0;

/// The parameters to construct a TestNet Dispenser API client.
#[derive(Debug, Clone)]
pub struct TestNetDispenserApiClientParams {
    /// The authentication token, otherwise an `ALGOKIT_DISPENSER_ACCESS_TOKEN` environment variable
    /// will be used. If neither is provided, an error is returned.
    pub auth_token: Option<String>,
    /// The request timeout in seconds. If not provided, a default value of 15 seconds is used.
    pub request_timeout: Option<u64>,
}

/// `TestNetDispenserApiClient` is a client that provides methods to interact with the
/// [Algorand TestNet Dispenser API](https://github.com/algorandfoundation/algokit/blob/main/docs/testnet_api.md).
/// It allows you to fund an address with Algo, refund a transaction, and get the funding limit for the Algo asset.
pub struct TestNetDispenserApiClient {
    auth_token: String,
    http_client: Client,
}

impl TestNetDispenserApiClient {
    /// Create a new TestNet Dispenser API client.
    ///
    /// # Parameters
    /// * `params` - Optional TestNetDispenserApiClientParams parameters.
    ///
    /// # Returns
    /// A new `TestNetDispenserApiClient` instance or an error if the auth token cannot be determined.
    pub fn new(params: Option<TestNetDispenserApiClientParams>) -> Result<Self, DispenserError> {
        use std::env;

        let auth_token = if let Some(ref p) = params {
            p.auth_token.clone()
        } else {
            env::var(DISPENSER_ACCESS_TOKEN_KEY).ok()
        };

        let auth_token = auth_token.ok_or(DispenserError::MissingAuthToken)?;

        let request_timeout = params.and_then(|p| p.request_timeout);

        let timeout_secs = request_timeout.unwrap_or(DEFAULT_DISPENSER_REQUEST_TIMEOUT);

        // Use the default Rust HTTP client
        let http_client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(|e| DispenserError::RequestError { source: e })?;

        Ok(Self {
            auth_token,
            http_client,
        })
    }

    async fn process_dispenser_request<TRequest: Serialize, TResponse: DeserializeOwned>(
        &self,
        url_suffix: &str,
        method: &str,
        body: Option<&TRequest>,
    ) -> Result<TResponse, DispenserError> {
        let url = format!("{}/{}", DISPENSER_BASE_URL, url_suffix);

        let mut request = match method {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            _ => {
                return Err(DispenserError::ApiError {
                    message: format!("Unsupported HTTP method: {}", method),
                });
            }
        };

        request = request.bearer_auth(&self.auth_token);

        if let Some(body_content) = body {
            request = request.json(body_content);
        }

        let response = request
            .send()
            .await
            .map_err(|e| DispenserError::RequestError { source: e })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_response = response.json::<ErrorResponse>().await;

            if let Ok(error_response) = error_response {
                if status == 400 {
                    return Err(DispenserError::ApiError {
                        message: error_response.message.unwrap_or_default(),
                    });
                } else {
                    return Err(DispenserError::ApiError {
                        message: error_response.code.unwrap_or_default(),
                    });
                }
            }

            return Err(DispenserError::ApiError {
                message: format!("Error processing dispenser API request: {}", status),
            });
        }

        response
            .json::<TResponse>()
            .await
            .map_err(|e| DispenserError::ParseError { source: e })
    }

    /// Sends a funding request to the dispenser API to fund the specified address with the given amount of Algo.
    ///
    /// # Parameters
    /// * `address` - The address to fund
    /// * `amount` - The amount of µAlgo to fund
    ///
    /// # Returns
    /// `DispenserFundResponse` - An object containing the transaction ID and funded amount
    pub async fn fund(
        &self,
        address: &Address,
        amount: u64,
    ) -> Result<DispenserFundResponse, DispenserError> {
        let request_body = FundRequest {
            receiver: address.to_string(),
            amount,
            asset_id: ALGO_ASSET_ID,
        };

        let response: FundResponseDto = self
            .process_dispenser_request(
                &format!("fund/{}", ALGO_ASSET_ID),
                "POST",
                Some(&request_body),
            )
            .await?;

        Ok(DispenserFundResponse {
            transaction_id: response.tx_id,
            amount: response.amount,
        })
    }

    /// Sends a refund request to the dispenser API for the specified refundTxnId.
    pub async fn refund(&self, refund_txn_id: &str) -> Result<(), DispenserError> {
        let request_body = RefundRequest {
            refund_transaction_id: refund_txn_id.to_string(),
        };

        self.process_dispenser_request::<RefundRequest, ()>("refund", "POST", Some(&request_body))
            .await?;

        Ok(())
    }

    /// Sends a request to the dispenser API to get the funding limit for the Algo asset.
    ///
    /// # Returns
    /// `DispenserLimitResponse` - An object containing the funding limit amount
    pub async fn get_limit(&self) -> Result<DispenserLimitResponse, DispenserError> {
        let response = self
            .process_dispenser_request::<(), LimitResponseDto>(
                &format!("fund/{}/limit", ALGO_ASSET_ID),
                "GET",
                None,
            )
            .await?;

        Ok(DispenserLimitResponse {
            amount: response.amount,
        })
    }
}
