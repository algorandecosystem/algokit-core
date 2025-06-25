use algod_client::apis::{configuration::Configuration, transaction_params};
use algod_client_tests::{LocalnetManager, ALGOD_CONFIG};
use std::sync::OnceLock;

/// Global configuration instance - idiomatic Rust pattern for shared test state
static CONFIG: OnceLock<Configuration> = OnceLock::new();

/// Get or initialize the algod client configuration
fn get_config() -> &'static Configuration {
    CONFIG.get_or_init(|| ALGOD_CONFIG.clone())
}

#[tokio::test]
async fn test_get_transaction_params() {
    // Ensure localnet is running
    LocalnetManager::ensure_running()
        .await
        .expect("Failed to start localnet");

    // Call the transaction params endpoint
    let result = transaction_params::transaction_params(get_config()).await;

    // Verify the call succeeded
    assert!(
        result.is_ok(),
        "Get transaction params should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Basic response validation
    assert!(
        !response.genesis_id.is_empty(),
        "Genesis ID should not be empty"
    );
    assert!(
        response.genesis_hash.len() == 32,
        "Genesis hash should be 32 bytes"
    );
    assert!(response.min_fee >= 0, "Min fee should be non-negative");
    assert!(
        response.last_round >= 0,
        "Last round should be non-negative"
    );

    println!("✓ Successfully retrieved transaction parameters");
    println!("  Genesis ID: {}", response.genesis_id);
    println!("  Genesis Hash: {:?}", response.genesis_hash);
    println!("  Min Fee: {}", response.min_fee);
    println!("  Last Round: {}", response.last_round);
    println!("  Consensus Version: {}", response.consensus_version);
}

#[tokio::test]
async fn test_transaction_params_error_handling() {
    // Test with invalid configuration to ensure error handling works
    let mut invalid_config = ALGOD_CONFIG.clone();
    invalid_config.base_path = "http://invalid-host:9999".to_string();

    let result = transaction_params::transaction_params(&invalid_config).await;

    // This should fail due to connection error
    assert!(result.is_err(), "Invalid host should result in error");

    println!("✓ Error handling test passed - correctly failed with invalid host");
}
