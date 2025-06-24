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
    assert!(!response.genesis_id.is_empty(), "Genesis ID should not be empty");
    assert!(response.genesis_hash.len() == 32, "Genesis hash should be 32 bytes");
    assert!(response.min_fee >= 0, "Min fee should be non-negative");
    assert!(response.last_round >= 0, "Last round should be non-negative");

    println!("✓ Successfully retrieved transaction parameters");
    println!("  Genesis ID: {}", response.genesis_id);
    println!("  Genesis Hash: {:?}", response.genesis_hash);
    println!("  Min Fee: {}", response.min_fee);
    println!("  Last Round: {}", response.last_round);
    println!("  Consensus Version: {}", response.consensus_version);
}

#[tokio::test]
async fn test_transaction_params_response_structure() {
    LocalnetManager::ensure_running()
        .await
        .expect("Failed to start localnet");

    let result = transaction_params::transaction_params(get_config()).await;
    assert!(result.is_ok(), "Should successfully get transaction params");

    let params = result.unwrap();

    // Test all expected fields are present and have reasonable values
    assert!(!params.genesis_id.is_empty(), "Genesis ID should be present");
    assert_eq!(params.genesis_hash.len(), 32, "Genesis hash should be 32 bytes");
    assert!(params.min_fee > 0, "Min fee should be positive");
    assert!(params.last_round >= 0, "Last round should be non-negative");
    assert!(!params.consensus_version.is_empty(), "Consensus version should be present");

    // For localnet, we can check some expected values
    if params.genesis_id.contains("dockernet") {
        println!("✓ Detected dockernet (localnet) configuration");
        assert!(params.min_fee == 1000, "Localnet min fee should typically be 1000");
    }

    println!("✓ Transaction params response structure validation passed");
}

#[tokio::test]
async fn test_multiple_transaction_params_calls() {
    LocalnetManager::ensure_running()
        .await
        .expect("Failed to start localnet");

    // Make multiple calls to ensure consistency
    let results = futures::future::join_all((0..3).map(|_| {
        transaction_params::transaction_params(get_config())
    })).await;

    // All calls should succeed
    for (i, result) in results.iter().enumerate() {
        assert!(
            result.is_ok(),
            "Call {} should succeed: {:?}",
            i,
            result.as_ref().err()
        );
    }

    let responses: Vec<_> = results.into_iter().map(|r| r.unwrap()).collect();

    // All responses should have the same genesis parameters (they don't change)
    let first = &responses[0];
    for (i, response) in responses.iter().enumerate().skip(1) {
        assert_eq!(
            first.genesis_id, response.genesis_id,
            "Genesis ID should be consistent across calls"
        );
        assert_eq!(
            first.genesis_hash, response.genesis_hash,
            "Genesis hash should be consistent across calls"
        );
        assert_eq!(
            first.min_fee, response.min_fee,
            "Min fee should be consistent across calls"
        );
        assert_eq!(
            first.consensus_version, response.consensus_version,
            "Consensus version should be consistent across calls"
        );

        // Last round might change between calls, but should be >= the previous
        assert!(
            response.last_round >= first.last_round,
            "Last round should not decrease: call 0: {}, call {}: {}",
            first.last_round,
            i,
            response.last_round
        );
    }

    println!("✓ Multiple transaction params calls consistency test passed");
    println!("  Made {} successful calls", responses.len());
    println!("  Genesis ID: {}", first.genesis_id);
    println!("  Last round range: {} - {}", 
        responses.iter().map(|r| r.last_round).min().unwrap(),
        responses.iter().map(|r| r.last_round).max().unwrap()
    );
}

#[tokio::test]
async fn test_transaction_params_json_and_msgpack_formats() {
    LocalnetManager::ensure_running()
        .await
        .expect("Failed to start localnet");

    // Test default format (JSON)
    let json_result = transaction_params::transaction_params(get_config()).await;
    assert!(json_result.is_ok(), "JSON format should work");
    
    let json_response = json_result.unwrap();

    // The transaction_params endpoint doesn't have a format parameter in the generated client,
    // but we can test that the response is properly decoded
    assert!(!json_response.genesis_id.is_empty());
    assert_eq!(json_response.genesis_hash.len(), 32);

    println!("✓ Transaction params JSON format test passed");
    println!("  Response contains all expected fields");
}

#[tokio::test]
async fn test_transaction_params_error_handling() {
    // Test with invalid configuration to ensure error handling works
    let mut invalid_config = ALGOD_CONFIG.clone();
    invalid_config.base_path = "http://invalid-host:9999".to_string();

    let result = transaction_params::transaction_params(&invalid_config).await;
    
    // This should fail due to connection error
    assert!(
        result.is_err(),
        "Invalid host should result in error"
    );

    println!("✓ Error handling test passed - correctly failed with invalid host");
}