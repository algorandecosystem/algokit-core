use algokit_http_client::DefaultHttpClient;
use algokit_transact::{OnApplicationComplete, StateSchema};
use algokit_utils::{ApplicationCreateParams, ClientManager, CommonParams, testing::*};
use indexer_client::IndexerClient;
use std::sync::Arc;

use crate::common::{init_test_logging, wait_for_indexer_application};

const HELLO_WORLD_APPROVAL_PROGRAM: [u8; 18] = [
    10, 128, 7, 72, 101, 108, 108, 111, 44, 32, 54, 26, 0, 80, 176, 129, 1, 67,
];

const HELLO_WORLD_CLEAR_STATE_PROGRAM: [u8; 4] = [10, 129, 1, 67];

async fn create_test_app(
    context: &AlgorandTestContext,
    sender: algokit_transact::Address,
) -> Option<u64> {
    let app_create_params = ApplicationCreateParams {
        common_params: CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        on_complete: OnApplicationComplete::NoOp,
        approval_program: HELLO_WORLD_APPROVAL_PROGRAM.to_vec(),
        clear_state_program: HELLO_WORLD_CLEAR_STATE_PROGRAM.to_vec(),
        global_state_schema: Some(StateSchema {
            num_uints: 1,
            num_byte_slices: 1,
        }),
        local_state_schema: Some(StateSchema {
            num_uints: 1,
            num_byte_slices: 1,
        }),
        extra_program_pages: None,
        args: Some(vec![b"Create".to_vec()]),
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
    };

    let mut composer = context.composer.clone();
    composer
        .add_application_create(app_create_params)
        .expect("Failed to add application create");
    let result = composer
        .send(None)
        .await
        .expect("Failed to send application create");
    result.confirmations[0].application_index
}

#[tokio::test]
async fn test_search_applications() {
    init_test_logging();

    let mut fixture = algorand_fixture().await.expect("Failed to create fixture");
    fixture
        .new_scope()
        .await
        .expect("Failed to create new scope");
    let context = fixture.context().expect("Failed to get context");
    let sender_addr = context
        .test_account
        .account()
        .expect("Failed to get sender account")
        .address();

    let app_id = create_test_app(context, sender_addr.clone())
        .await
        .expect("Failed to create test app");

    let config = ClientManager::get_config_from_environment_or_localnet();
    let base_url = if let Some(port) = config.indexer_config.port {
        format!("{}:{}", config.indexer_config.server, port)
    } else {
        config.indexer_config.server.clone()
    };
    let http_client = Arc::new(DefaultHttpClient::new(&base_url));
    let indexer_client = IndexerClient::new(http_client);

    wait_for_indexer_application(&indexer_client, app_id, 15)
        .await
        .expect("Application should be indexed");

    let result = indexer_client
        .search_for_applications(Some(app_id), None, None, None, None)
        .await;

    assert!(
        result.is_ok(),
        "Search applications should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.applications.is_empty(),
        "Should find the created application"
    );
    assert_eq!(
        response.applications[0].id, app_id,
        "Application ID should match"
    );

    if let Some(token) = &response.next_token {
        assert!(!token.is_empty(), "Next token should not be empty");
    }
}

#[tokio::test]
async fn test_search_applications_error_handling() {
    init_test_logging();

    let http_client = Arc::new(DefaultHttpClient::new("http://invalid-host:8980"));
    let indexer_client = IndexerClient::new(http_client);

    let result = indexer_client
        .search_for_applications(None, None, None, None, None)
        .await;

    assert!(result.is_err(), "Invalid indexer should fail");
}
