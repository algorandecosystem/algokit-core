use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIValue, Arc56Contract};
use algokit_utils::applications::app_client::{AppClient, AppClientParams};
use algokit_utils::clients::app_manager::TealTemplateValue;
use algokit_utils::config::{AppCompiledEventData, EventData, EventType};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use rstest::*;
use std::sync::Arc;

fn get_testing_app_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::testing_app::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

#[rstest]
#[tokio::test]
async fn compile_applies_template_params_and_emits_event(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    algokit_utils::config::Config::configure(Some(true), None);
    let mut events = algokit_utils::config::Config::events().subscribe();

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
        algorand: fixture.algorand_client,
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        source_maps: None,
        transaction_composer_config: None,
    });

    let result = client
        .send()
        .call(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "call_abi".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("test"))],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let abi_return = result.abi_return.expect("Expected ABI return");
    match abi_return.return_value {
        Some(ABIValue::String(s)) => assert_eq!(s, "Hello, test"),
        _ => return Err("Expected string ABI return".into()),
    }

    if let Ok((event_type, data)) =
        tokio::time::timeout(std::time::Duration::from_millis(5000), events.recv()).await?
    {
        assert_eq!(event_type, EventType::AppCompiled);
        match data {
            EventData::AppCompiled(AppCompiledEventData {
                app_name,
                approval_source_map,
                clear_source_map,
            }) => {
                assert_eq!(app_name.as_deref(), Some("TestingApp"));
                assert!(approval_source_map.is_some());
                assert!(clear_source_map.is_some());
            }
            _ => return Err("unexpected event data".into()),
        }
    } else {
        return Err("expected AppCompiled event".into());
    }

    Ok(())
}
