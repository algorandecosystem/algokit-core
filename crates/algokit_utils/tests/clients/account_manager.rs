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
    let account = account_manager
        .from_environment(&account_name, None)
        .await?;

    let account_address = account.address();

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

#[rstest]
#[tokio::test]
async fn same_account_is_subsequently_retrieved(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;

    // Generate a random account name
    let name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let mut account_manager = algorand_fixture
        .algorand_client
        .account_manager()
        .lock()
        .unwrap();

    // Get the account from environment twice with the same name
    let account = account_manager.from_environment(&name, None).await?;
    let account2 = account_manager.from_environment(&name, None).await?;

    // The accounts are different instances but should have the same address
    assert_eq!(
        account.address(),
        account2.address(),
        "Both calls should return the same account address"
    );

    assert_eq!(
        account.secret_key, account2.secret_key,
        "Both calls should return the same secret key"
    );

    Ok(())
}
