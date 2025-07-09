use crate::common::init_test_logging;
use algokit_utils::testing::*;
use algokit_utils::transactions::composer::{AssetOptInParams, AssetTransferParams};
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

    let asset_receiver = fixture
        .generate_account(None)
        .await
        .expect("Failed to create foo");

    let context = fixture.context().expect("Failed to get context");
    let asa_creator_address = context.test_account.address().unwrap();
    let mut composer = context.composer.clone();

    composer
        .add_asset_create(AssetCreateParams {
            common_params: CommonParams {
                sender: asa_creator_address.clone(),
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
    let asa_create_result = composer
        .send()
        .await
        .expect("Failed to send asset create transaction");

    let asset_receiver_address = asset_receiver.address().unwrap();
    let mut composer = context.composer.clone();
    composer
        .add_asset_opt_in(AssetOptInParams {
            common_params: CommonParams {
                sender: asset_receiver_address.clone(),
                signer: Some(Arc::new(asset_receiver.clone())),
                ..Default::default()
            },
            asset_id: asa_create_result.asset_index.unwrap(),
        })
        .expect("Failed to add asset opt-in");
    let _ = composer.send().await.expect("Failed to opt-in");

    let mut composer = context.composer.clone();
    composer
        .add_asset_transfer(AssetTransferParams {
            common_params: CommonParams {
                sender: asa_creator_address.clone(),
                ..Default::default()
            },
            asset_id: asa_create_result.asset_index.unwrap(),
            receiver: asset_receiver_address.clone(),
            amount: 0,
        })
        .expect("Failed to add asset transfer");

    let result = composer.send().await.expect("Failed to send transaction");

    match result.txn.transaction {
        algokit_transact::Transaction::AssetTransfer(asset_transfer_fields) => {
            assert_eq!(
                asset_transfer_fields.asset_id,
                asa_create_result.asset_index.unwrap(),
                "Asset ID should match the created ASA"
            );
            // assert_eq!(
            //     asset_transfer_fields.amount, 0,
            //     "Asset transfer amount should be 0 for opt-in"
            // );
            // assert_eq!(
            //     asset_transfer_fields.header.sender, asset_receiver_address,
            //     "Sender and receiver should be the same for opt-in"
            // );
            // assert_eq!(
            //     asset_transfer_fields.header.sender, asset_transfer_fields.receiver,
            //     "Sender and receiver should be the same for opt-in"
            // );
        }
        _ => panic!("Transaction should be an asset transfer transaction"),
    }
}
