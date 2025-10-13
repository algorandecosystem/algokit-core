use algod_client::{AlgodClient, models::PendingTransactionResponse};
use algokit_utils::{
    AlgorandClient, clients::TestNetDispenserApiClient, transactions::composer::ComposerError,
};
use std::sync::Arc;

/// Test happy path for TestNet dispenser client:
/// 1. Generate a random account using the account manager
/// 2. Use the dispenser to fund 1 ALGO to the newly created account
/// 3. Call algod account info to verify the account has 1 ALGO
/// 4. Refund the ALGO using the dispenser client
///
/// This test is ignored by default because it requires a TestNet dispenser API token.
/// To run this test, use: `cargo test test_testnet_dispenser_happy_path -- --ignored`
#[tokio::test]
#[ignore]
async fn test_testnet_dispenser_happy_path() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    // Load environment variables from .env file if it exists
    let _ = dotenvy::dotenv();

    // Create a TestNet AlgorandClient
    let algorand_client = AlgorandClient::testnet(None);

    // Generate a random account using the account manager
    let test_account_address = {
        let mut account_manager = algorand_client.account_manager().lock().unwrap();
        account_manager.random()
    };

    // Create TestNet dispenser client (assumes ALGOKIT_DISPENSER_ACCESS_TOKEN env var is set)
    let dispenser_client = TestNetDispenserApiClient::new(None)?;

    // Fund 0.1 ALGO (100,000 microAlgos) to the test account
    let fund_amount = 100_000u64;
    let fund_response = dispenser_client
        .fund(&test_account_address, fund_amount)
        .await?;

    // Verify the funding response
    assert_eq!(fund_response.amount, fund_amount);
    assert!(!fund_response.transaction_id.is_empty());

    // Wait for the funding transaction to be confirmed
    wait_for_confirmation(
        algorand_client.client().algod(),
        &fund_response.transaction_id,
        10,
    )
    .await?;

    // Get account information from algod to verify the account has 1 ALGO
    let account_info = algorand_client
        .client()
        .algod()
        .account_information(&test_account_address.to_string(), None, None)
        .await?;

    // Verify the account has at least 1 ALGO (account might have slightly more due to rewards)
    assert!(
        account_info.amount >= fund_amount,
        "Account should have at least {} microAlgos, but has {}",
        fund_amount,
        account_info.amount
    );

    println!("✅ TestNet dispenser happy path test completed successfully!");
    println!("   - Generated account: {}", test_account_address);
    println!("   - Funded amount: {} microAlgos", fund_amount);
    println!("   - Fund transaction ID: {}", fund_response.transaction_id);
    println!(
        "   - Account balance after funding: {} microAlgos",
        account_info.amount
    );

    Ok(())
}

// TODO: refactor this to a common method
async fn wait_for_confirmation(
    algod_client: Arc<AlgodClient>,
    tx_id: &str,
    max_rounds_to_wait: u32,
) -> Result<PendingTransactionResponse, ComposerError> {
    let status = algod_client
        .get_status()
        .await
        .map_err(|e| ComposerError::TransactionError {
            message: format!("Failed to get status: {:?}", e),
        })?;

    let start_round = status.last_round + 1;
    let mut current_round = start_round;

    while current_round < start_round + max_rounds_to_wait as u64 {
        match algod_client.pending_transaction_information(tx_id).await {
            Ok(response) => {
                // Check for pool errors first - transaction was kicked out of pool
                if !response.pool_error.is_empty() {
                    return Err(ComposerError::PoolError {
                        message: format!(
                            "Transaction {} was rejected; pool error: {}",
                            tx_id,
                            response.pool_error.clone()
                        ),
                    });
                }

                // Check if transaction is confirmed
                if response.confirmed_round.is_some() {
                    return Ok(response);
                }
            }
            Err(error) => {
                // Only retry for 404 errors (transaction not found yet)
                // All other errors indicate permanent issues and should fail fast
                let is_retryable = matches!(
                        &error,
                        algod_client::apis::Error::Api {
                            source: algod_client::apis::AlgodApiError::PendingTransactionInformation {
                                error: algod_client::apis::pending_transaction_information::PendingTransactionInformationError::Status404(_)
                            }
                        }
                    ) || error.to_string().contains("404");

                if is_retryable {
                    current_round += 1;
                    continue;
                } else {
                    return Err(ComposerError::AlgodClientError { source: error });
                }
            }
        };

        let _ = algod_client.wait_for_block(current_round).await;
        current_round += 1;
    }

    Err(ComposerError::MaxWaitRoundExpired {
        message: format!(
            "Transaction {} unconfirmed after {} rounds",
            tx_id, max_rounds_to_wait
        ),
    })
}
