use crate::common::init_test_logging;
use algokit_transact::{AssetConfigTransactionBuilder, TransactionHeader};
use algokit_utils::CommonParams;
use algokit_utils::testing::*;
use algokit_utils::transactions::composer::{AssetOptInParams, AssetTransferParams};

#[tokio::test]
async fn test_asset_transfer_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let asa_creator = fixture
        .generate_account(None)
        .await
        .expect("Failed to create ASA creator");

    let context = fixture.context().expect("Failed to get context");

    // FIXME: When composer gets asset creation support, we can use it directly.
    let algod_client = context.algod.clone();
    let sp = algod_client.transaction_params().await.unwrap();
    let asa_create_txn = AssetConfigTransactionBuilder::default()
        .header(TransactionHeader {
            sender: asa_creator.address().unwrap(),
            fee: Some(1_000),
            first_valid: sp.last_round,
            last_valid: sp.last_round + 20,
            genesis_id: Some(sp.genesis_id),
            genesis_hash: Some(sp.genesis_hash.try_into().unwrap()),
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
        })
        .asset_id(0)
        .total(10)
        .decimals(0)
        .default_frozen(false)
        .build()
        .unwrap();
    let signed_asa_create_txn = asa_creator.sign_transaction(&asa_create_txn).unwrap();
    let asa_create_id = algod_client
        .raw_transaction(signed_asa_create_txn)
        .await
        .expect("Failed to send asset creation transaction");

    let composer_asa_create = context.composer.clone();
    let asa_create_result = composer_asa_create
        .wait_for_confirmation(asa_create_id.tx_id.as_str(), 1000)
        .await
        .expect("Failed to wait for ASA creation confirmation");

    let asa_user_address = context.test_account.address().unwrap();
    let mut composer_asa_transfer = context.composer.clone();
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
            receiver: asa_creator.address().unwrap(),
            amount: 0,
        })
        .expect("Failed to add asset transfer");

    let result = composer_asa_transfer
        .send()
        .await
        .expect("Failed to send transaction");

    // FIXME: How to access the second transaction?
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
