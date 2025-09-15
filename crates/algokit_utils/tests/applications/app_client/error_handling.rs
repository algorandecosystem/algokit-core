use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::Arc56Contract;
use algokit_utils::AlgorandClient;
use algokit_utils::applications::app_client::{
    AppClient, AppClientMethodCallParams, AppClientParams,
};
use algokit_utils::clients::app_manager::TealTemplateValue;
use algokit_utils::config::Config;
use rstest::*;
use std::sync::Arc;

fn get_testing_app_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::testing_app::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

#[rstest]
#[tokio::test]
async fn test_exposing_logic_error_with_and_without_sourcemaps(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    let spec = get_testing_app_spec();

    let tmpl = [("VALUE", 1), ("UPDATABLE", 0), ("DELETABLE", 0)]
        .into_iter()
        .map(|(k, v)| (k.to_string(), TealTemplateValue::Int(v)))
        .collect();

    let app_id = deploy_arc56_contract(&fixture, &sender, &spec, Some(tmpl), None, None).await?;

    let mut algorand = AlgorandClient::default_localnet(None);
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));

    let mut client = AppClient::new(AppClientParams {
        app_id,
        app_spec: spec,
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        source_maps: None,
        transaction_composer_config: None,
    });

    let err_without_maps = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "error".to_string(),
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await
        .expect_err("expected logic error");
    assert!(err_without_maps.to_string().contains("assert failed"));

    Config::configure(Some(true), None);
    let source_maps = client.export_source_maps();
    if let Some(maps) = source_maps {
        client.import_source_maps(maps);
    }

    let err_with_maps = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "error".to_string(),
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await
        .expect_err("expected logic error");
    assert!(err_with_maps.to_string().contains("assert failed"));

    Ok(())
}
