// Tests for Client Management Features

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIValue, Arc56Contract};
use algokit_transact::{OnApplicationComplete, StateSchema};
use algokit_utils::applications::app_client::AppClient;
use algokit_utils::clients::app_manager::{AppManager, TealTemplateParams};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppCreateParams, AppMethodCallArg};
use rstest::*;
use std::collections::HashMap;
use std::sync::Arc;

fn get_sandbox_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::sandbox::APPLICATION_ARC56)
        .expect("valid arc56")
}

fn get_hello_world_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::hello_world::APPLICATION_ARC56)
        .expect("valid arc56")
}

#[rstest]
#[tokio::test]
async fn from_network_resolves_id(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let spec = get_hello_world_spec();
    let app_id = deploy_arc56_contract(&fixture, &sender, &spec, None, None, None).await?;

    let mut spec_with_networks = spec.clone();
    spec_with_networks.networks = Some(HashMap::from([(
        "localnet".to_string(),
        algokit_abi::arc56_contract::Network { app_id },
    )]));

    let client = AppClient::from_network(
        spec_with_networks,
        RootAlgorandClient::default_localnet(None),
        None,
        None,
        None,
        None,
        None,
    )
    .await?;

    assert_eq!(client.app_id(), app_id);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn from_creator_and_name_resolves_and_can_call(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let spec = get_sandbox_spec();
    let src = spec.source.as_ref().expect("source expected");
    let approval_teal = src.get_decoded_approval().expect("approval");
    let clear_teal = src.get_decoded_clear().expect("clear");

    let app_manager: &AppManager = fixture.algorand_client.app();
    let compiled_approval = app_manager
        .compile_teal_template(&approval_teal, None::<&TealTemplateParams>, None)
        .await?;
    let compiled_clear = app_manager
        .compile_teal_template(&clear_teal, None::<&TealTemplateParams>, None)
        .await?;

    let app_name = "MY_APP".to_string();
    let deploy_note = format!(
        "{}:j{}",
        "ALGOKIT_DEPLOYER",
        serde_json::json!({
            "name": app_name,
            "version": "1.0",
            "updatable": false,
            "deletable": false
        })
    );

    let create_params = AppCreateParams {
        sender: sender.clone(),
        on_complete: OnApplicationComplete::NoOp,
        approval_program: compiled_approval.compiled_base64_to_bytes.clone(),
        clear_state_program: compiled_clear.compiled_base64_to_bytes.clone(),
        global_state_schema: Some(StateSchema {
            num_uints: spec.state.schema.global_state.ints,
            num_byte_slices: spec.state.schema.global_state.bytes,
        }),
        local_state_schema: Some(StateSchema {
            num_uints: spec.state.schema.local_state.ints,
            num_byte_slices: spec.state.schema.local_state.bytes,
        }),
        note: Some(deploy_note.into_bytes()),
        ..Default::default()
    };

    let create_result = fixture
        .algorand_client
        .send()
        .app_create(create_params, None)
        .await?;

    fixture
        .wait_for_indexer_transaction(&create_result.common_params.tx_id)
        .await?;

    let mut algorand = RootAlgorandClient::default_localnet(None);
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));

    let client = AppClient::from_creator_and_name(
        &sender.to_string(),
        &app_name,
        spec.clone(),
        algorand,
        Some(sender.to_string()),
        Some(Arc::new(fixture.test_account.clone())),
        None,
        None,
        None,
    )
    .await?;

    assert_eq!(client.app_id(), create_result.app_id);
    assert_eq!(client.app_name(), Some(&app_name));

    let res = client
        .send()
        .call(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "hello_world".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("test"))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let abi_ret = res.abi_return.as_ref().expect("abi return");
    match &abi_ret.return_value {
        Some(ABIValue::String(s)) => assert_eq!(s, "Hello, test"),
        _ => panic!("expected string return"),
    }

    Ok(())
}
