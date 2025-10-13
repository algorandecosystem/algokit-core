use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture};
use rand::{Rng, distributions::Alphanumeric};
use rstest::rstest;
use std::sync::Arc;

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

#[rstest]
#[tokio::test]
async fn environment_is_used_in_preference_to_kmd(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let algorand_fixture = algorand_fixture.await?;

    // Generate a random account name for the first account (will use KMD)
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

    // Create an account via KMD
    let account = account_manager.from_environment(&name, None).await?;

    // Convert the secret key to mnemonic
    let mnemonic = algokit_utils::common::mnemonic::from_key(&account.secret_key)?;

    // Set environment variable for a second account name
    let name2 = "TEST";
    let env_var_name = format!("{}_MNEMONIC", name2);

    // SAFETY: We're in a test environment and managing the lifecycle of this environment variable
    // We clean it up immediately after use
    unsafe {
        std::env::set_var(&env_var_name, &mnemonic);
    }

    // Create a second account manager instance to test environment precedence
    let account2 = account_manager.from_environment(name2, None).await?;

    // Clean up environment variable
    // SAFETY: We set this variable above and are now cleaning it up
    unsafe {
        std::env::remove_var(&env_var_name);
    }

    // The accounts should have the same address and secret key
    assert_eq!(
        account.address(),
        account2.address(),
        "Both accounts should have the same address"
    );

    assert_eq!(
        account.secret_key, account2.secret_key,
        "Both accounts should have the same secret key"
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn rekeyed_account_is_retrievable(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut algorand_fixture = algorand_fixture.await?;

    // Generate two accounts: one to be rekeyed and one to rekey to
    let rekeyed = algorand_fixture
        .generate_account(Some(crate::common::TestAccountConfig {
            initial_funds: 1_000_000,
            ..Default::default()
        }))
        .await?; // 1 ALGO
    let rekey_to = algorand_fixture
        .generate_account(Some(crate::common::TestAccountConfig {
            initial_funds: 100_000,
            ..Default::default()
        }))
        .await?; // 0.1 ALGO

    let mut account_manager = algorand_fixture
        .algorand_client
        .account_manager()
        .lock()
        .unwrap();

    // Perform the rekey operation
    account_manager
        .rekey_account(
            rekeyed.account().address(),
            rekey_to.account().address(),
            Some(Arc::new(rekey_to.clone())),
            None,
        )
        .await?;

    // Get account information to verify the rekey
    let account_info = account_manager
        .get_information(&rekeyed.account().address())
        .await?;

    // Verify the account address is still the original
    assert_eq!(
        account_info.address,
        rekeyed.account().address().to_string(),
        "Account address should remain the same"
    );

    // Verify the auth-addr is now set to the rekey_to address
    assert_eq!(
        account_info.auth_addr,
        Some(rekey_to.account().address().to_string()),
        "Auth address should be set to the rekey_to address"
    );

    Ok(())
}
