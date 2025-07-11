use crate::common::init_test_logging;
use algokit_transact::{
    AssetConfigTransactionBuilder, AssetTransferTransactionBuilder, FeeParams,
    TransactionHeaderBuilder,
};
use algokit_utils::CommonParams;
use algokit_utils::testing::*;
use algokit_utils::transactions::composer::{AssetTransferParams, SendParams};

#[tokio::test]
async fn test_asset_transfer_transaction() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");

    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");

    let context = fixture.context().expect("Failed to get context");
    let mut composer = context.composer.clone();
    let algod = context.algod.clone();
    let asset_creator = context.test_account.clone();
    let asset_creator_address = asset_creator.account().unwrap().address();
    let asset_user = fixture
        .generate_account(None)
        .await
        .expect("Failed to generate account");
    let asset_user_address = asset_user.account().unwrap().address();

    let asset_create_sp = algod
        .transaction_params()
        .await
        .expect("Failed to get suggested params for asset create transaction");
    let mut asset_create_transaction = AssetConfigTransactionBuilder::default()
        .header(
            TransactionHeaderBuilder::default()
                .sender(asset_creator_address.clone())
                .first_valid(asset_create_sp.last_round)
                .last_valid(asset_create_sp.last_round + 10)
                .genesis_hash(
                    asset_create_sp
                        .genesis_hash
                        .try_into()
                        .expect("Failed to convert genesis hash for asset create transaction"),
                )
                .genesis_id(asset_create_sp.genesis_id)
                .build()
                .expect("Failed to build header for asset create transaction"),
        )
        .asset_id(0)
        .total(10)
        .decimals(0)
        .default_frozen(false)
        .build()
        .expect("Failed to build asset create transaction");
    asset_create_transaction = asset_create_transaction
        .assign_fee(FeeParams {
            fee_per_byte: asset_create_sp.fee,
            min_fee: asset_create_sp.min_fee,
            extra_fee: None,
            max_fee: None,
        })
        .expect("Failed to assign fee for asset create transaction");
    let signed_asset_create_transaction = asset_creator
        .sign_transaction(&asset_create_transaction)
        .expect("Failed to sign asset create transaction");
    let asset_create_transaction_id = algod
        .raw_transaction(signed_asset_create_transaction)
        .await
        .expect("Failed to send asset create transaction")
        .tx_id;
    let asset_create_result = composer
        .wait_for_confirmation(asset_create_transaction_id.as_str(), 10)
        .await
        .expect("Failed to wait for asset create transaction confirmation");

    let asset_opt_in_sp = algod
        .transaction_params()
        .await
        .expect("Failed to get suggested params for asset opt-in transaction");
    let mut asset_opt_in_transaction = AssetTransferTransactionBuilder::default()
        .header(
            TransactionHeaderBuilder::default()
                .sender(asset_user_address.clone())
                .fee(asset_opt_in_sp.fee)
                .first_valid(asset_opt_in_sp.last_round)
                .last_valid(asset_opt_in_sp.last_round + 10)
                .genesis_hash(
                    asset_opt_in_sp
                        .genesis_hash
                        .try_into()
                        .expect("Failed to convert genesis hash for asset opt-in transaction"),
                )
                .genesis_id(asset_opt_in_sp.genesis_id)
                .build()
                .expect("Failed to build header for asset opt-in transaction"),
        )
        .asset_id(asset_create_result.asset_index.unwrap())
        .receiver(asset_user_address.clone())
        .amount(0)
        .build()
        .expect("Failed to build asset opt-in transaction");
    asset_opt_in_transaction = asset_opt_in_transaction
        .assign_fee(FeeParams {
            fee_per_byte: asset_opt_in_sp.fee,
            min_fee: asset_opt_in_sp.min_fee,
            extra_fee: None,
            max_fee: None,
        })
        .expect("Failed to assign fee for asset opt-in transaction");
    let signed_asset_opt_in_transaction = asset_user
        .sign_transaction(&asset_opt_in_transaction)
        .expect("Failed to sign asset opt-in transaction");
    let asset_opt_in_transaction_id = algod
        .raw_transaction(signed_asset_opt_in_transaction)
        .await
        .expect("Failed to send asset opt-in transaction")
        .tx_id;
    composer
        .wait_for_confirmation(asset_opt_in_transaction_id.as_str(), 10)
        .await
        .expect("Failed to wait for asset opt-in transaction confirmation");

    composer
        .add_asset_transfer(AssetTransferParams {
            common_params: CommonParams {
                sender: asset_creator_address.clone(),
                ..Default::default()
            },
            asset_id: asset_create_result.asset_index.unwrap(),
            receiver: asset_user_address.clone(),
            amount: 1,
        })
        .expect("Failed to add asset create");
    let asset_transfer_result = composer
        .send(Some(SendParams {
            max_rounds_to_wait_for_confirmation: Some(10),
        }))
        .await
        .expect("Failed to send asset create transaction");

    match &asset_transfer_result
        .confirmations
        .first()
        .unwrap()
        .txn
        .transaction
    {
        algokit_transact::Transaction::AssetTransfer(asset_transfer_fields) => {
            assert_eq!(
                asset_transfer_fields.asset_id,
                asset_create_result.asset_index.unwrap(),
                "Asset ID should match the created asset"
            );
            assert_eq!(
                asset_transfer_fields.amount, 1,
                "Asset transfer amount should be 1"
            );
            assert_eq!(
                asset_transfer_fields.header.sender,
                asset_creator_address.clone(),
                "Sender should be the asset creator"
            );
            assert_eq!(
                asset_transfer_fields.receiver,
                asset_user_address.clone(),
                "Receiver should be the asset user"
            );
        }
        _ => panic!("Transaction should be an asset transfer transaction"),
    }
}
