use algod_client::apis::configuration::Configuration;
use algokit_transact::{PaymentTransactionBuilder, TransactionHeaderBuilder};
use base64::{prelude::BASE64_STANDARD, Engine};
use once_cell::sync::Lazy;
use std::env;

/// Default localnet configuration
pub static ALGOD_CONFIG: Lazy<Configuration> = Lazy::new(|| {
    let host = env::var("ALGORAND_HOST").unwrap_or_else(|_| "http://localhost:4001".to_string());
    let token = env::var("ALGORAND_API_TOKEN").unwrap_or_else(|_| {
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
    });

    Configuration {
        base_path: host,
        user_agent: Some("algod_client_tests/1.0.0".to_string()),
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: Some(algod_client::apis::configuration::ApiKey {
            prefix: None, // No prefix needed for algod API token
            key: token,
        }),
    }
});

/// Global algod client instance - shared across all tests
/// Note: We use a function that returns a reference to avoid circular imports
pub fn get_algod_client() -> &'static algod_client::apis::AlgodClient {
    use algod_client::apis::AlgodClient;
    use std::sync::OnceLock;

    static CLIENT: OnceLock<AlgodClient> = OnceLock::new();
    CLIENT.get_or_init(|| AlgodClient::new(ALGOD_CONFIG.clone()))
}

/// Re-export the test utilities from algokit_transact
pub use algokit_transact::test_utils::{
    AddressMother, TestDataMother, TransactionHeaderMother, TransactionMother,
};

/// Extended transaction builders for localnet testing
pub struct LocalnetTransactionMother;

impl LocalnetTransactionMother {
    /// LocalNet header configuration (using dockernet genesis)
    pub fn localnet_header() -> TransactionHeaderBuilder {
        TransactionHeaderBuilder::default()
            .genesis_id(String::from("dockernet-v1"))
            .genesis_hash(
                BASE64_STANDARD
                    .decode("R4ZGJ8m36vYb6qo6HwKKqb4ZRjP8IZNOCgdp42uJ2So=")
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .fee(1000)
            .to_owned()
    }

    /// Simple localnet transaction header with sender and validity window
    pub fn simple_localnet_header() -> TransactionHeaderBuilder {
        Self::localnet_header()
            .sender(AddressMother::address())
            .first_valid(1000)
            .last_valid(2000)
            .to_owned()
    }

    /// Simple payment transaction for localnet testing
    pub fn simple_payment() -> PaymentTransactionBuilder {
        PaymentTransactionBuilder::default()
            .header(Self::simple_localnet_header().build().unwrap())
            .amount(1000000) // 1 ALGO
            .receiver(AddressMother::neil())
            .to_owned()
    }

    /// Payment transaction with note for localnet testing
    pub fn payment_with_note() -> PaymentTransactionBuilder {
        Self::simple_payment()
            .header(
                Self::simple_localnet_header()
                    .note(b"test payment".to_vec())
                    .build()
                    .unwrap(),
            )
            .to_owned()
    }
}
