use crate::applications::app_client::common::testing_app_fixture;
use crate::common::TestResult;
use algokit_utils::applications::app_client::{
    AppClientError, AppClientMethodCallParams, AppSourceMaps,
};
use algokit_utils::config::Config;
use regex::Regex;
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_exposing_logic_error_with_and_without_sourcemaps(
    #[future] testing_app_fixture: crate::common::AppFixtureResult,
) -> TestResult {
    let f = testing_app_fixture.await?;
    let sender = f.sender_address;
    let mut client = f.client;

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
    let msg_wo = err_without_maps.to_string();
    assert!(msg_wo.contains("Runtime error when executing"));
    assert!(msg_wo.contains("assert failed"));
    let re_tx = Regex::new(r"transaction [A-Z2-7]{52}").unwrap();
    let re_app = Regex::new(r"appId: [0-9]+").unwrap();
    let msg_wo_redacted = re_app
        .replace_all(
            &re_tx.replace_all(&msg_wo, "transaction <TXID>"),
            "appId: <APPID>",
        )
        .to_string();
    insta::assert_snapshot!(msg_wo_redacted, @r###"Runtime error when executing TestingApp (appId: <APPID>) in transaction <TXID>: assert failed pc=885"###);
    if let AppClientError::LogicError { logic, .. } = &err_without_maps {
        assert!(logic.message.contains("Runtime error when executing"));
    }

    Config::configure(Some(true), None);
    let approval_src = client
        .app_spec()
        .source
        .as_ref()
        .unwrap()
        .get_decoded_approval()
        .unwrap()
        .into_bytes();
    let clear_src = client
        .app_spec()
        .source
        .as_ref()
        .unwrap()
        .get_decoded_clear()
        .unwrap()
        .into_bytes();
    let approval_map = client
        .algorand()
        .app()
        .compile_teal(&String::from_utf8_lossy(&approval_src))
        .await
        .ok()
        .and_then(|c| c.source_map);
    let clear_map = client
        .algorand()
        .app()
        .compile_teal(&String::from_utf8_lossy(&clear_src))
        .await
        .ok()
        .and_then(|c| c.source_map);
    client.import_source_maps(AppSourceMaps {
        approval_source_map: approval_map,
        clear_source_map: clear_map,
    });

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
    let msg_w = err_with_maps.to_string();
    assert!(msg_w.contains("Runtime error when executing"));
    assert!(msg_w.contains("assert failed"));
    let msg_w_redacted = re_app
        .replace_all(
            &re_tx.replace_all(&msg_w, "transaction <TXID>"),
            "appId: <APPID>",
        )
        .to_string();
    insta::assert_snapshot!(msg_w_redacted, @r###"Runtime error when executing TestingApp (appId: <APPID>) in transaction <TXID>: assert failed pc=885"###);
    if let AppClientError::LogicError { logic, .. } = &err_with_maps {
        if let Some(lines) = &logic.lines {
            let listing = lines.join("\n");
            assert!(!listing.is_empty());
            assert!(listing.contains("assert"));
        }
    }

    Ok(())
}
