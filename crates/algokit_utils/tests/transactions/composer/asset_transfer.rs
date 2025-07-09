use crate::common::init_test_logging;
use algokit_utils::testing::*;
use algokit_utils::transactions::composer::{AssetOptInParams, AssetTransferParams};
use algokit_utils::{AssetCreateParams, CommonParams};

#[tokio::test]
async fn test_asset_transfer_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context_asa_creator = fixture.context().expect("Failed to get context");
    let asa_creator_address = context_asa_creator.test_account.address().unwrap();
    let mut composer_asa_creator = context_asa_creator.composer.clone();

    composer_asa_creator
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
    let asa_create_result = composer_asa_creator
        .send()
        .await
        .expect("Failed to send asset create transaction");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context_asa_transfer = fixture.context().expect("Failed to get context");

    let asa_user_address = context_asa_transfer.test_account.address().unwrap();
    let mut composer_asa_transfer = context_asa_transfer.composer.clone();
    composer_asa_transfer
        .add_asset_opt_in(AssetOptInParams {
            common_params: CommonParams {
                sender: asa_user_address.clone(),
                ..Default::default()
            },
            asset_id: asa_create_result.asset_index.unwrap(),
        })
        .expect("Failed to add asset opt-in");

    composer_asa_transfer
        .add_asset_transfer(AssetTransferParams {
            common_params: CommonParams {
                sender: asa_user_address.clone(),
                ..Default::default()
            },
            asset_id: asa_create_result.asset_index.unwrap(),
            receiver: asa_creator_address,
            amount: 0,
        })
        .expect("Failed to add asset transfer");

    let result = composer_asa_transfer
        .send()
        .await
        .expect("Failed to send transaction");

    match result.txn.transaction {
        algokit_transact::Transaction::AssetTransfer(asset_transfer_fields) => {
            assert_eq!(
                asset_transfer_fields.asset_id,
                asa_create_result.asset_index.unwrap(),
                "Asset ID should match the created ASA"
            );
            assert_eq!(
                asset_transfer_fields.amount, 0,
                "Asset transfer amount should be 0 for opt-in"
            );
            assert_eq!(
                asset_transfer_fields.header.sender, asa_user_address,
                "Sender and receiver should be the same for opt-in"
            );
            assert_eq!(
                asset_transfer_fields.header.sender, asset_transfer_fields.receiver,
                "Sender and receiver should be the same for opt-in"
            );
        }
        _ => panic!("Transaction should be an asset transfer transaction"),
    }
}
