use crate::applications::app_client::common::testing_app_fixture;
use crate::common::TestResult;
use algokit_utils::applications::app_client::AppClientMethodCallParams;
use algokit_utils::config::Config;
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
