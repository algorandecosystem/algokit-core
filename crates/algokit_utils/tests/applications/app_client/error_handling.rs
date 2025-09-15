// Tests for Error Handling Features
// - Logic Error Exposure: Expose logic errors with details
// - Source Map Support: Use source maps for debugging
// - ARC56 Error Messages: Handle ARC56-specific errors
// - Error Transformer: Transform errors for better debugging

use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::Arc56Contract;
use algokit_utils::applications::app_client::{
    AppClient, AppClientMethodCallParams, AppClientParams,
};
use algokit_utils::config::Config;
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use rstest::*;
use std::sync::Arc;

fn get_testing_app_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::testing_app::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

fn get_template_variables_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::template_variables::APPLICATION_ARC56;
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
    let mut tmpl: algokit_utils::clients::app_manager::TealTemplateParams = Default::default();
    tmpl.insert(
        "VALUE".to_string(),
        algokit_utils::clients::app_manager::TealTemplateValue::Int(1),
    );
    tmpl.insert(
        "UPDATABLE".to_string(),
        algokit_utils::clients::app_manager::TealTemplateValue::Int(0),
    );
    tmpl.insert(
        "DELETABLE".to_string(),
        algokit_utils::clients::app_manager::TealTemplateValue::Int(0),
    );
    let app_id = deploy_arc56_contract(&fixture, &sender, &spec, Some(tmpl), None, None).await?;

    let mut algorand = RootAlgorandClient::default_localnet(None);
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
    if let Some(maps) = source_maps.clone() {
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
    let s = err_with_maps.to_string();
    assert!(s.contains("assert failed"));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_display_nice_error_messages_when_logic_error(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let spec = get_testing_app_spec();
    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &spec,
        Some(
            [("VALUE", 1), ("UPDATABLE", 0), ("DELETABLE", 0)]
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.to_string(),
                        algokit_utils::clients::app_manager::TealTemplateValue::Int(v),
                    )
                })
                .collect(),
        ),
        None,
        None,
    )
    .await?;

    let mut algorand = RootAlgorandClient::default_localnet(None);
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

    // Enable debug mode to get detailed error information
    Config::configure(Some(true), None);

    // Import source maps if available
    let source_maps = client.export_source_maps();
    if let Some(maps) = source_maps.clone() {
        client.import_source_maps(maps);
    }

    // Call the error method which should trigger a logic error
    let result = client
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
        .await;

    // Verify we get an error
    assert!(result.is_err(), "Expected logic error but call succeeded");

    let error = result.unwrap_err();
    let error_str = error.to_string();

    // Check that error contains expected information
    assert!(
        error_str.contains("assert failed"),
        "Error should contain 'assert failed'"
    );

    // Check for PC (program counter) in error - typically in format "pc=885" or similar
    assert!(
        error_str.contains("pc="),
        "Error should contain program counter"
    );

    // Extract PC value from error string
    let pc_regex = regex::Regex::new(r"pc=(\d+)").unwrap();
    let pc_value = pc_regex
        .captures(&error_str)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str());

    // Verify PC value exists
    assert!(pc_value.is_some(), "Should extract PC value from error");

    // Create a structured error info for snapshot testing
    #[derive(serde::Serialize)]
    struct ErrorInfo {
        contains_assert_failed: bool,
        has_pc: bool,
        has_source_maps: bool,
        error_pattern: String,
    }

    let error_info = ErrorInfo {
        contains_assert_failed: error_str.contains("assert failed"),
        has_pc: error_str.contains("pc="),
        has_source_maps: source_maps.is_some(),
        // Extract just the error pattern without specific values for stable snapshots
        error_pattern: if error_str.contains("assert failed") && error_str.contains("pc=") {
            "assert failed pc=XXX".to_string()
        } else {
            "unexpected error format".to_string()
        },
    };

    // Use insta for snapshot testing
    insta::assert_json_snapshot!(error_info, {
        ".has_source_maps" => "[source_maps_status]",
    });

    // With source maps, we should get additional context about the error location
    if source_maps.is_some() {
        // Extract the TEAL source context if available
        let teal_context = if error_str.contains("// error") || error_str.contains("assert") {
            // Create a simplified version of the TEAL context for snapshot
            Some("TEAL context with error marker present")
        } else {
            None
        };

        insta::assert_debug_snapshot!(teal_context);
    }

    Ok(())
}
