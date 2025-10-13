use algokit_utils::{AlgorandClient, clients::TestNetDispenserApiClient};

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

    // Wait a moment for the transaction to be confirmed
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

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

    // Refund the ALGO using the dispenser client
    dispenser_client
        .refund(&fund_response.transaction_id)
        .await?;

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
