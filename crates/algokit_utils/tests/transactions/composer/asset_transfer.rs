use crate::common::init_test_logging;
use algokit_utils::testing::*;
use algokit_utils::transactions::composer::{AssetOptInParams, AssetTransferParams, SendParams};
use algokit_utils::{AssetCreateParams, CommonParams};
use std::sync::Arc;

#[tokio::test]
async fn test_asset_transfer_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");
    let context = fixture.context().expect("Failed to get context");

    let mut asset_create_composer = context.composer.clone();
    let asset_creator = context.test_account.clone();
    let asset_creator_address = asset_creator.account().unwrap().address();
    asset_create_composer
        .add_asset_create(AssetCreateParams {
            common_params: CommonParams {
                sender: asset_creator_address.clone(),
                signer: Some(Arc::new(asset_creator)),
                ..Default::default()
            },
            total: 10,
            decimals: Some(0),
            default_frozen: Some(false),
            asset_name: None,
            unit_name: None,
            url: None,
            metadata_hash: None,
            manager: None,
            reserve: None,
            freeze: None,
            clawback: None,
        })
        .expect("Failed to add asset create");
    let asset_create_id = asset_create_composer
        .send(Some(SendParams {
            max_rounds_to_wait_for_confirmation: Some(10),
        }))
        .await
        .expect("Failed to send asset create")
        .confirmations
        .first()
        .expect("Asset create group should have at least one transaction")
        .asset_index
        .unwrap();

    let mut asset_transfer_composer = context.composer.clone();
    let asset_user = fixture
        .generate_account(None)
        .await
        .expect("Failed to generate account");
    let asset_user_address = asset_user.account().unwrap().address();
    asset_transfer_composer
        .add_asset_opt_in(AssetOptInParams {
            common_params: CommonParams {
                sender: asset_user_address.clone(),
                signer: Some(Arc::new(asset_user)),
                ..Default::default()
            },
            asset_id: asset_create_id,
        })
        .expect("Failed to add asset opt in");
    asset_transfer_composer
        .add_asset_transfer(AssetTransferParams {
            common_params: CommonParams {
                sender: asset_creator_address.clone(),
                ..Default::default()
            },
            asset_id: asset_create_id,
            receiver: asset_user_address.clone(),
            amount: 1,
        })
        .expect("Failed to add asset transfer");
    let asset_transfer_result = asset_transfer_composer
        .send(Some(SendParams {
            max_rounds_to_wait_for_confirmation: Some(10),
        }))
        .await
        .expect("Failed to send asset transfer");

    match &asset_transfer_result
        .confirmations
        .get(0)
        .unwrap()
        .txn
        .transaction
    {
        algokit_transact::Transaction::AssetTransfer(asset_opt_in_fields) => {
            assert_eq!(
                asset_opt_in_fields.header.sender,
                asset_user_address.clone(),
                "Account opting in should be the asset user"
            );
            assert_eq!(
                asset_opt_in_fields.receiver,
                asset_user_address.clone(),
                "Sender and receiver should be the same for opt-in"
            );
            assert_eq!(
                asset_opt_in_fields.asset_id,
                asset_create_id.clone(),
                "Asset ID should match the created asset"
            );
            assert_eq!(
                asset_opt_in_fields.amount, 0,
                "Amount should be 0 for opt-in"
            );
        }
        _ => panic!("Transaction should be an asset transfer transaction"),
    }
    match &asset_transfer_result
        .confirmations
        .get(1)
        .unwrap()
        .txn
        .transaction
    {
        algokit_transact::Transaction::AssetTransfer(asset_transfer_fields) => {
            assert_eq!(
                asset_transfer_fields.header.sender, asset_creator_address,
                "Sender should be the asset creator"
            );
            assert_eq!(
                asset_transfer_fields.receiver, asset_user_address,
                "Receiver should be the asset user"
            );
            assert_eq!(
                asset_transfer_fields.asset_id, asset_create_id,
                "Asset ID should match the created asset"
            );
            assert_eq!(asset_transfer_fields.amount, 1, "Amount should be 1");
        }
        _ => panic!("Transaction should be an asset transfer transaction"),
    }
}
