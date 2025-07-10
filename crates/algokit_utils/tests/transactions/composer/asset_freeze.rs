use algokit_transact::Transaction;
use algokit_utils::{
    testing::*,
    transactions::{AssetFreezeParams, AssetUnfreezeParams},
};

#[tokio::test]
async fn test_asset_freeze_unfreeze() {
    let mut fixture = algorand_fixture().await.unwrap();
    fixture.new_scope().await.unwrap();

    let context = fixture.context().unwrap();
    let acct1 = context.test_account.address().unwrap();

    let mut composer = context.composer.clone();

    composer
        .add_asset_freeze(AssetFreezeParams {
            common_params: Default::default(),
            asset_id: 1,
            target_account: acct1.clone(),
        })
        .unwrap();

    let built_group = composer.build().await.unwrap().built_group().unwrap();
    assert_eq!(built_group.len(), 1);

    match &built_group[0] {
        Transaction::AssetFreeze(txn) => {
            assert_eq!(txn.asset_id, 1);
            assert_eq!(txn.freeze_target, acct1);
            assert_eq!(txn.frozen, Some(true));
        }
        _ => panic!("Expected AssetFreeze transaction"),
    }

    let mut composer = context.composer.clone();
    composer
        .add_asset_unfreeze(AssetUnfreezeParams {
            common_params: Default::default(),
            asset_id: 1,
            target_account: acct1.clone(),
        })
        .unwrap();

    let built_group = composer.build().await.unwrap().built_group().unwrap();
    assert_eq!(built_group.len(), 1);

    match &built_group[0] {
        Transaction::AssetFreeze(txn) => {
            assert_eq!(txn.asset_id, 1);
            assert_eq!(txn.freeze_target, acct1);
            assert_eq!(txn.frozen, Some(false));
        }
        _ => panic!("Expected AssetFreeze transaction"),
    }
}
