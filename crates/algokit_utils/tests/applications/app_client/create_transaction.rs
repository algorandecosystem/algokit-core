// Tests for Transaction Creation Features

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIValue, Arc56Contract};
use algokit_transact::BoxReference;
use algokit_utils::applications::app_client::{
    AppClient, AppClientMethodCallParams, AppClientParams,
};
use algokit_utils::clients::app_manager::TealTemplateValue;
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use rstest::*;
use std::sync::Arc;

fn get_testing_app_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::testing_app::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

#[rstest]
#[tokio::test]
async fn create_txn_with_box_references(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &get_testing_app_spec(),
        Some(
            [("VALUE", 1), ("UPDATABLE", 0), ("DELETABLE", 0)]
                .into_iter()
                .map(|(k, v)| (k.to_string(), TealTemplateValue::Int(v)))
                .collect(),
        ),
        None,
        None,
    )
    .await?;

    let mut algorand = RootAlgorandClient::default_localnet(None);
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let client = AppClient::new(AppClientParams {
        app_id,
        app_spec: get_testing_app_spec(),
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        source_maps: None,
        transaction_composer_config: None,
    });

    let tx = client
        .create_transaction()
        .call(
            AppClientMethodCallParams {
                method: "call_abi".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("test"))],
                sender: Some(sender.to_string()),
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: b"1".to_vec(),
                }]),
                ..Default::default()
            },
            None,
        )
        .await?;

    if let algokit_transact::Transaction::AppCall(fields) = tx {
        let boxes = fields.box_references.expect("boxes");
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].app_id, 0);
        assert_eq!(boxes[0].name, b"1".to_vec());
    } else {
        panic!("expected app call txn")
    }

    Ok(())
}
