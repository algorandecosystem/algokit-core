// Tests for Default Value Resolution
// - Literal values: Default resolved from base64-encoded constant
// - Method call: Default resolved by calling another ABI method
// - Global state: Default resolved from app global state
// - Local state: Default resolved from app local state for sender

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIValue, Arc56Contract};
use algokit_utils::applications::app_client::{
    AppClient, AppClientMethodCallParams, AppClientParams,
};
use algokit_utils::clients::app_manager::{TealTemplateParams, TealTemplateValue};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use num_bigint::BigUint;
use rstest::*;
use std::sync::Arc;

fn get_testing_app_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::testing_app::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

async fn deploy_testing_app(
    fixture: &crate::common::AlgorandFixture,
    sender: &algokit_transact::Address,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let mut tmpl: TealTemplateParams = Default::default();
    tmpl.insert("VALUE".to_string(), TealTemplateValue::Int(0));
    tmpl.insert("UPDATABLE".to_string(), TealTemplateValue::Int(0));
    tmpl.insert("DELETABLE".to_string(), TealTemplateValue::Int(0));

    deploy_arc56_contract(
        fixture,
        sender,
        &get_testing_app_spec(),
        Some(tmpl),
        None,
        None,
    )
    .await
}

fn new_client(
    app_id: u64,
    fixture: &crate::common::AlgorandFixture,
    sender: &algokit_transact::Address,
) -> AppClient {
    let mut algorand = RootAlgorandClient::default_localnet(None);
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    AppClient::new(AppClientParams {
        app_id,
        app_spec: get_testing_app_spec(),
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        source_maps: None,
        transaction_composer_config: None,
    })
}

#[rstest]
#[tokio::test]
async fn test_default_value_from_literal(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    let app_id = deploy_testing_app(&fixture, &sender).await?;
    let client = new_client(app_id, &fixture, &sender);

    let defined = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("defined value"))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let defined_ret = defined
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match defined_ret {
        ABIValue::String(s) => assert_eq!(s, "defined value"),
        _ => panic!("Expected string return"),
    }

    let defaulted = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value".to_string(),
                args: vec![AppMethodCallArg::DefaultValue],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let default_ret = defaulted
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match default_ret {
        ABIValue::String(s) => assert_eq!(s, "default value"),
        _ => panic!("Expected string return"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_default_value_from_method(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    let app_id = deploy_testing_app(&fixture, &sender).await?;
    let client = new_client(app_id, &fixture, &sender);

    let defined = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_abi".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("defined value"))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let defined_ret = defined
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match defined_ret {
        ABIValue::String(s) => assert_eq!(s, "ABI, defined value"),
        _ => panic!("Expected string return"),
    }

    let defaulted = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_abi".to_string(),
                args: vec![AppMethodCallArg::DefaultValue],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let default_ret = defaulted
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match default_ret {
        ABIValue::String(s) => assert_eq!(s, "ABI, default value"),
        _ => panic!("Expected string return"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_default_value_from_global_state(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    let app_id = deploy_testing_app(&fixture, &sender).await?;
    let client = new_client(app_id, &fixture, &sender);

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "set_global".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from(456u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from(2u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from("asdf")),
                    AppMethodCallArg::ABIValue(ABIValue::Array(vec![
                        ABIValue::from_byte(1),
                        ABIValue::from_byte(2),
                        ABIValue::from_byte(3),
                        ABIValue::from_byte(4),
                    ])),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let defined = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_global_state".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from(123u64))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let defined_ret = defined
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match defined_ret {
        ABIValue::Uint(v) => assert_eq!(v, BigUint::from(123u64)),
        _ => panic!("Expected uint return"),
    }

    let defaulted = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_global_state".to_string(),
                args: vec![AppMethodCallArg::DefaultValue],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let default_ret = defaulted
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match default_ret {
        ABIValue::Uint(v) => assert_eq!(v, BigUint::from(456u64)),
        _ => panic!("Expected uint return"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_default_value_from_local_state(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    let app_id = deploy_testing_app(&fixture, &sender).await?;
    let client = new_client(app_id, &fixture, &sender);

    client
        .send()
        .opt_in(
            AppClientMethodCallParams {
                method: "opt_in".to_string(),
                args: vec![],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "set_local".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from(1u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from(2u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from("banana")),
                    AppMethodCallArg::ABIValue(ABIValue::Array(vec![
                        ABIValue::from_byte(1),
                        ABIValue::from_byte(2),
                        ABIValue::from_byte(3),
                        ABIValue::from_byte(4),
                    ])),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let defined = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_local_state".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("defined value"))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let defined_ret = defined
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match defined_ret {
        ABIValue::String(s) => assert_eq!(s, "Local state, defined value"),
        _ => panic!("Expected string return"),
    }

    let defaulted = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_local_state".to_string(),
                args: vec![AppMethodCallArg::DefaultValue],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;
    let default_ret = defaulted
        .abi_return
        .and_then(|r| r.return_value)
        .expect("Expected ABI return value");
    match default_ret {
        ABIValue::String(s) => assert_eq!(s, "Local state, banana"),
        _ => panic!("Expected string return"),
    }

    Ok(())
}
