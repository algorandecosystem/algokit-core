use algokit_utils::{
    AlgorandClient, clients::TestNetDispenserApiClient, transactions::utils::wait_for_confirmation,
};

/// This test is ignored by default because it requires a TestNet dispenser API token.
#[tokio::test]
#[ignore]
async fn test_testnet_dispenser() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = dotenvy::dotenv();

    let algorand_client = AlgorandClient::testnet(None);

    // Generate a random account using the account manager
    let test_account_address = {
        let mut account_manager = algorand_client.account_manager().lock().await;
        account_manager.random().address()
    };

    let dispenser_client = TestNetDispenserApiClient::new(None)?;

    let fund_amount = 100_000u64; // 0.1 Algo
    let fund_response: algokit_utils::clients::DispenserFundResponse = dispenser_client
        .fund(&test_account_address, fund_amount)
        .await?;

    assert_eq!(fund_response.amount, fund_amount);
    assert!(!fund_response.transaction_id.is_empty());

    wait_for_confirmation(
        algorand_client.client().algod(),
        &fund_response.transaction_id,
        100,
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

    println!("✅ TestNet dispenser test completed successfully!");
    println!("   - Generated account: {}", test_account_address);
    println!("   - Funded amount: {} microAlgos", fund_amount);
    println!("   - Fund transaction ID: {}", fund_response.transaction_id);
    println!(
        "   - Account balance after funding: {} microAlgos",
        account_info.amount
    );

    Ok(())
}
