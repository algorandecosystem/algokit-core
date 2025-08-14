use algokit_transact::Address;
use algokit_utils::{
    clients::{
        app_manager::AppManager,
        asset_manager::{AssetManager, AssetManagerError},
    },
    testing::algorand_fixture,
    transactions::{AssetCreateParams, CommonParams, Composer, EmptySigner, TransactionSender},
};
use rstest::*;
use std::sync::Arc;

/// Test asset information retrieval
#[rstest]
#[tokio::test]
async fn test_get_asset_by_id() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut fixture = algorand_fixture().await?;
    fixture.new_scope().await?;
    let context = fixture.context()?;
    let asset_manager = AssetManager::new(Arc::new(context.algod.clone()));

    // Create test asset
    let asset_id = create_test_asset(&mut fixture).await?;

    // Test successful retrieval
    let asset_info = asset_manager.get_by_id(asset_id).await?;
    assert_eq!(asset_info.asset_id, asset_id);
    assert_eq!(asset_info.total, 1000);
    assert_eq!(asset_info.decimals, 0);
    assert_eq!(asset_info.unit_name, Some("TEST".to_string()));
    assert_eq!(asset_info.asset_name, Some("Test Asset".to_string()));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_get_asset_by_id_nonexistent() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let mut fixture = algorand_fixture().await?;
    fixture.new_scope().await?;
    let context = fixture.context()?;
    let asset_manager = AssetManager::new(Arc::new(context.algod.clone()));

    // Test non-existent asset
    let result = asset_manager.get_by_id(999999999).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AssetManagerError::AlgodClientError(_)
    ));

    Ok(())
}

/// Test account asset information retrieval
#[rstest]
#[tokio::test]
async fn test_get_account_information() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut fixture = algorand_fixture().await?;
    fixture.new_scope().await?;
    let context = fixture.context()?;
    let asset_manager = AssetManager::new(Arc::new(context.algod.clone()));

    let (asset_id, creator_address) = create_test_asset_with_creator(&mut fixture).await?;

    // Test account information for asset creator (should be opted in by default)
    let account_info = asset_manager
        .get_account_information(&creator_address, asset_id)
        .await?;

    assert_eq!(account_info.asset_id, asset_id);
    assert_eq!(account_info.balance, 1000); // Creator gets all initial supply
    assert!(!account_info.frozen);
    assert!(account_info.round > 0);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_get_account_information_not_opted_in()
-> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut fixture = algorand_fixture().await?;
    fixture.new_scope().await?;
    let context = fixture.context()?;
    let asset_manager = AssetManager::new(Arc::new(context.algod.clone()));

    let asset_id = create_test_asset(&mut fixture).await?;
    let test_account = fixture.generate_account(None).await?;

    // Test account information for non-opted-in account should return error
    let result = asset_manager
        .get_account_information(&test_account.account()?.address(), asset_id)
        .await;

    // For non-opted-in accounts, algod returns 404 which becomes an AlgodClientError
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AssetManagerError::AlgodClientError(_)
    ));

    Ok(())
}

/// Test opt-in status checking
/// Helper function to create a test asset and return its ID
async fn create_test_asset(
    fixture: &mut algokit_utils::testing::AlgorandFixture,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let creator = fixture.generate_account(None).await?;
    let context = fixture.context()?;
    let algod_client = Arc::new(context.algod.clone());
    let sender = TransactionSender::new(
        {
            let client = algod_client.clone();
            move || Composer::new(client.clone(), Arc::new(EmptySigner {}))
        },
        AssetManager::new(algod_client.clone()),
        AppManager::new(algod_client.clone()),
    );

    let params = AssetCreateParams {
        common_params: CommonParams {
            sender: creator.account()?.address(),
            signer: Some(Arc::new(creator.clone())),
            ..Default::default()
        },
        total: 1000,
        decimals: Some(0),
        unit_name: Some("TEST".to_string()),
        asset_name: Some("Test Asset".to_string()),
        ..Default::default()
    };

    let result = sender.asset_create(params, None).await?;

    Ok(result.asset_id)
}

/// Helper function to create a test asset and return both asset ID and creator address
async fn create_test_asset_with_creator(
    fixture: &mut algokit_utils::testing::AlgorandFixture,
) -> Result<(u64, Address), Box<dyn std::error::Error + Send + Sync>> {
    let creator = fixture.generate_account(None).await?;
    let creator_address = creator.account()?.address();
    let context = fixture.context()?;
    let algod_client = Arc::new(context.algod.clone());
    let sender = TransactionSender::new(
        {
            let client = algod_client.clone();
            move || Composer::new(client.clone(), Arc::new(EmptySigner {}))
        },
        AssetManager::new(algod_client.clone()),
        AppManager::new(algod_client.clone()),
    );

    let params = AssetCreateParams {
        common_params: CommonParams {
            sender: creator_address.clone(),
            signer: Some(Arc::new(creator.clone())),
            ..Default::default()
        },
        total: 1000,
        decimals: Some(0),
        unit_name: Some("TEST".to_string()),
        asset_name: Some("Test Asset".to_string()),
        ..Default::default()
    };

    let result = sender.asset_create(params, None).await?;

    let asset_id = result.asset_id;
    Ok((asset_id, creator_address))
}
