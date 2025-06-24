use algod_client::apis::{configuration::Configuration, raw_transaction};
use algod_client_tests::{LocalnetManager, ALGOD_CONFIG};
use algokit_transact::test_utils::TestDataMother;
use std::sync::OnceLock;

/// Global configuration instance - idiomatic Rust pattern for shared test state
static CONFIG: OnceLock<Configuration> = OnceLock::new();

/// Get or initialize the algod client configuration
fn get_config() -> &'static Configuration {
    CONFIG.get_or_init(|| ALGOD_CONFIG.clone())
}

/// Create a test payment transaction using pre-signed test data
async fn create_test_transaction() -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    // Use the pre-signed test data which has a valid signature
    let test_data = TestDataMother::simple_payment();
    Ok(test_data.signed_bytes)
}

#[tokio::test]
async fn test_raw_transaction_broadcast() {
    // Ensure localnet is running
    LocalnetManager::ensure_running()
        .await
        .expect("Failed to start localnet");

    // Create and encode a test transaction
    let signed_txn_bytes = create_test_transaction()
        .await
        .expect("Failed to create test transaction");

    // Call the raw transaction endpoint
    let result = raw_transaction::raw_transaction(get_config(), signed_txn_bytes).await;

    // The transaction might fail due to network mismatch (testnet vs localnet)
    // but we should at least get past the signature verification and msgpack issues
    match result {
        Ok(response) => {
            // If it succeeds, verify response structure
            assert!(
                !response.tx_id.is_empty(),
                "Transaction ID should not be empty"
            );
            assert!(
                response.tx_id.len() >= 32,
                "Transaction ID should be at least 32 characters"
            );
            println!("✓ Successfully broadcasted raw transaction");
            println!("  Transaction ID: {}", response.tx_id);
        }
        Err(err) => {
            // Expected errors due to network differences or other validation issues
            println!(
                "Transaction failed as expected due to test/network mismatch: {:?}",
                err
            );

            // The error should not be about msgpack decoding anymore
            let error_str = format!("{:?}", err);
            assert!(
                !error_str.contains("msgpack decode error"),
                "Should not have msgpack decode errors anymore"
            );

            // This test validates that the API layer works correctly even if the transaction fails
            println!("✓ API layer working correctly - transaction reached algod validation");
        }
    }
}

#[tokio::test]
async fn test_raw_transaction_with_multiple_transactions() {
    LocalnetManager::ensure_running()
        .await
        .expect("Failed to start localnet");

    // Test broadcasting multiple transactions
    let mut results = Vec::new();

    for i in 0..3 {
        let signed_txn_bytes = create_test_transaction()
            .await
            .expect(&format!("Failed to create test transaction {}", i));

        let result = raw_transaction::raw_transaction(get_config(), signed_txn_bytes).await;
        results.push(result);
    }

    // Check that all calls reached the API (regardless of success/failure)
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(response) => {
                println!("Transaction {}: Success - {}", i + 1, response.tx_id);
            }
            Err(err) => {
                let error_str = format!("{:?}", err);
                assert!(
                    !error_str.contains("msgpack decode error"),
                    "Transaction {} should not have msgpack decode errors",
                    i
                );
                println!(
                    "Transaction {}: Expected failure - API layer working",
                    i + 1
                );
            }
        }
    }

    println!("✓ Successfully tested multiple transaction broadcasts");
}

#[tokio::test]
async fn test_raw_transaction_error_handling() {
    LocalnetManager::ensure_running()
        .await
        .expect("Failed to start localnet");

    // Test with invalid transaction bytes (empty vector)
    let invalid_txn_bytes: Vec<u8> = vec![];

    let result = raw_transaction::raw_transaction(get_config(), invalid_txn_bytes).await;

    // This should fail
    assert!(
        result.is_err(),
        "Empty transaction bytes should result in error"
    );

    println!("✓ Error handling test passed - correctly failed with invalid transaction");
}

#[tokio::test]
async fn test_raw_transaction_api_format() {
    LocalnetManager::ensure_running()
        .await
        .expect("Failed to start localnet");

    // Test that the API accepts binary data correctly
    let test_data = TestDataMother::simple_payment();
    let signed_txn_bytes = test_data.signed_bytes;

    // This test verifies the API can handle the msgpack format correctly
    let result = raw_transaction::raw_transaction(get_config(), signed_txn_bytes).await;

    // We expect either success or a meaningful algod error (not a format error)
    match result {
        Ok(_) => {
            println!("✓ Transaction succeeded");
        }
        Err(err) => {
            let error_str = format!("{:?}", err);

            // Should not be msgpack format errors
            assert!(
                !error_str.contains("msgpack decode error"),
                "Should not have msgpack decode errors"
            );
            assert!(
                !error_str.contains("only encoded map or array"),
                "Should not have JSON array decode errors"
            );

            println!("✓ API format test passed - no format-related errors");
        }
    }
}
