use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture};
use rand::{Rng, distributions::Alphanumeric};
use rstest::rstest;

#[rstest]
#[tokio::test]
async fn test_from_environment(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;

    // Generate a random account name
    let account_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let mut account_manager = algorand_fixture
        .algorand_client
        .account_manager()
        .lock()
        .unwrap();

    // Get the account from environment (will create a new one on LocalNet)
    let account_address = account_manager
        .from_environment(&account_name, None)
        .await?;

    // Get account information
    let account_info = account_manager.get_information(&account_address).await?;

    // Assert that the account has a balance greater than 0
    assert!(
        account_info.amount > 0,
        "Account balance should be greater than 0, got: {}",
        account_info.amount
    );

    Ok(())
}
