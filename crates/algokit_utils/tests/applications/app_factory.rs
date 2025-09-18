use crate::common::TestAccount;
use crate::common::{
    AlgorandFixture, AlgorandFixtureResult, TestResult, algorand_fixture, testing_app_spec,
};
use algokit_abi::Arc56Contract;
use algokit_transact::Address;
use algokit_transact::OnApplicationComplete;
use algokit_utils::applications::app_client::{AppClientMethodCallParams, CompilationParams};
use algokit_utils::applications::app_factory::AppFactoryParams;
use algokit_utils::applications::app_factory::{
    AppFactory, AppFactoryCreateMethodCallParams, AppFactoryCreateParams,
};
use algokit_utils::clients::app_manager::{TealTemplateParams, TealTemplateValue};
use algokit_utils::transactions::TransactionComposerConfig;
use algokit_utils::{AlgorandClient, AppMethodCallArg};
use rstest::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct AppFactoryOptions {
    pub app_name: Option<String>,
    pub updatable: Option<bool>,
    pub deletable: Option<bool>,
    pub deploy_time_params: Option<HashMap<String, TealTemplateValue>>,
    pub transaction_composer_config: Option<TransactionComposerConfig>,
}

fn abi_str_arg(s: &str) -> AppMethodCallArg {
    AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(s))
}

fn into_factory_inputs(fixture: AlgorandFixture) -> (Arc<AlgorandClient>, TestAccount) {
    let AlgorandFixture {
        algorand_client,
        test_account,
        ..
    } = fixture;
    (Arc::new(algorand_client), test_account)
}

/// Construct an `AppFactory` for a provided ARC-56 spec with common defaults.
pub async fn build_app_factory_with_spec(
    algorand_client: Arc<AlgorandClient>,
    test_account: TestAccount,
    app_spec: Arc56Contract,
    opts: AppFactoryOptions,
) -> AppFactory {
    let sender: Address = test_account.account().address();

    AppFactory::new(AppFactoryParams {
        algorand: algorand_client,
        app_spec,
        app_name: opts.app_name,
        default_sender: Some(sender.to_string()),
        default_signer: Some(Arc::new(test_account.clone())),
        version: None,
        deploy_time_params: opts.deploy_time_params,
        updatable: opts.updatable,
        deletable: opts.deletable,
        source_maps: None,
        transaction_composer_config: opts.transaction_composer_config,
    })
}

async fn build_testing_app_factory(
    algorand_client: Arc<AlgorandClient>,
    test_account: TestAccount,
    opts: AppFactoryOptions,
) -> AppFactory {
    return build_app_factory_with_spec(algorand_client, test_account, testing_app_spec(), opts)
        .await;
}

fn compilation_params(value: u64, updatable: bool, deletable: bool) -> CompilationParams {
    let mut t = TealTemplateParams::default();
    t.insert("VALUE".to_string(), TealTemplateValue::Int(value));
    CompilationParams {
        deploy_time_params: Some(t),
        updatable: Some(updatable),
        deletable: Some(deletable),
        ..Default::default()
    }
}

#[rstest]
#[tokio::test]
async fn bare_create_with_deploy_time_params(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_create_app — references/.../test_app_factory.py:L85-L105
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(false),
            deletable: Some(false),
            ..Default::default()
        },
    )
    .await;

    let compilation_params = compilation_params(1, false, false);

    let (client, res) = factory
        .send()
        .bare()
        .create(
            Some(AppFactoryCreateParams::default()),
            None,
            Some(compilation_params),
        )
        .await?;

    assert!(client.app_id() > 0);
    assert_eq!(
        client.app_address(),
        algokit_transact::Address::from_app_id(&client.app_id())
    );
    assert!(res.app_id > 0);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn constructor_compilation_params_precedence(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_create_app_with_constructor_deploy_time_params — L107-L135
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(false),
            deletable: Some(false),
            ..Default::default()
        },
    )
    .await;

    let (client, result) = factory.send().bare().create(None, None, None).await?;

    assert!(result.app_id > 0);
    assert_eq!(client.app_id(), result.app_id);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn oncomplete_override_on_create(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_create_app_with_oncomplete_overload — L137-L157
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;

    let params = AppFactoryCreateParams {
        on_complete: Some(OnApplicationComplete::OptIn),
        ..Default::default()
    };
    let compilation_params = compilation_params(1, true, true);
    let (client, result) = factory
        .send()
        .bare()
        .create(Some(params), None, Some(compilation_params))
        .await?;

    match &result.common_params.transaction {
        algokit_transact::Transaction::AppCall(fields) => {
            assert_eq!(
                fields.on_complete,
                algokit_transact::OnApplicationComplete::OptIn
            );
        }
        _ => panic!("expected app call"),
    }
    assert!(client.app_id() > 0);
    assert_eq!(
        client.app_address(),
        algokit_transact::Address::from_app_id(&client.app_id())
    );
    assert!(result.common_params.confirmations.first().is_some());
    Ok(())
}

#[rstest]
#[tokio::test]
async fn abi_based_create_returns_value(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_create_app_with_abi — L448-L465
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;

    let cp = compilation_params(1, true, false);

    let (_client, call_return) = factory
        .send()
        .create(
            AppFactoryCreateMethodCallParams {
                method: "create_abi(string)string".to_string(),
                args: Some(vec![abi_str_arg("string_io")]),
                ..Default::default()
            },
            None,
            Some(cp),
        )
        .await?;

    let abi_ret = call_return.abi_return.expect("abi return");
    if let Some(algokit_abi::ABIValue::String(s)) = abi_ret.return_value {
        assert_eq!(s, "string_io");
    } else {
        panic!("expected string");
    }
    Ok(())
}

#[rstest]
#[tokio::test]
async fn create_then_call_via_app_client(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_create_then_call_app — L396-L409
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            updatable: Some(true),
            ..Default::default()
        },
    )
    .await;

    let cp = compilation_params(1, true, true);

    let (client, _res) = factory.send().bare().create(None, None, Some(cp)).await?;

    let send_res = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi(string)string".to_string(),
                args: vec![abi_str_arg("test")],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let abi_ret = send_res.abi_return.expect("abi return");
    if let Some(algokit_abi::ABIValue::String(s)) = abi_ret.return_value {
        assert_eq!(s, "Hello, test");
    } else {
        panic!("expected string");
    }
    Ok(())
}

#[rstest]
#[tokio::test]
async fn call_app_with_too_many_args(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_call_app_with_too_many_args — L411-L424
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(false),
            deletable: Some(false),
            ..Default::default()
        },
    )
    .await;

    let (client, _res) = factory
        .send()
        .bare()
        .create(None, None, Some(compilation_params(1, false, false)))
        .await?;

    let err = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi(string)string".to_string(),
                args: vec![abi_str_arg("test"), abi_str_arg("extra")],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await
        .expect_err("expected error for too many args");
    // The error is wrapped into a ValidationError; extract message via Display
    let msg = err.to_string();
    // Accept the actual error message format from Rust implementation
    assert!(
        msg.contains("The number of provided arguments is 2 while the method expects 1 arguments")
            || msg.contains("Unexpected arg at position 1. call_abi only expects 1 args")
            || msg.contains("Unexpected arg at position 2. call_abi only expects 1 args"),
        "Expected error message about too many arguments, got: {msg}"
    );
    Ok(())
}

#[rstest]
#[tokio::test]
async fn call_app_with_rekey(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_call_app_with_rekey — L426-L446
    let mut fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    // Generate a new account to rekey to before consuming the fixture
    let rekey_to = fixture.generate_account(None).await?;
    let rekey_to_addr = rekey_to.account().address();
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        Arc::clone(&algorand_client),
        test_account,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;

    let (client, _res) = factory.send().bare().create(None, None, None).await?;

    // Opt-in with rekey_to
    client
        .send()
        .opt_in(
            AppClientMethodCallParams {
                method: "opt_in()void".to_string(),
                args: vec![],
                sender: Some(sender.to_string()),
                rekey_to: Some(rekey_to_addr.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    // If rekey succeeded, a zero payment using the rekeyed signer should succeed
    let pay = algokit_utils::PaymentParams {
        sender: sender.clone(),
        // signer will be picked up from account manager: set_signer already configured for original sender,
        // but after rekey the auth address must be rekey_to's signer. Use explicit signer.
        signer: Some(Arc::new(rekey_to.clone())),
        rekey_to: None,
        note: None,
        lease: None,
        static_fee: None,
        extra_fee: None,
        max_fee: None,
        validity_window: None,
        first_valid_round: None,
        last_valid_round: None,
        receiver: sender.clone(),
        amount: 0,
    };
    let _ = algorand_client.send().payment(pay, None).await?;
    Ok(())
}

#[rstest]
#[tokio::test]
async fn delete_app_with_abi_direct(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_delete_app_with_abi — L493-L512
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(false),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;

    let (client, _res) = factory
        .send()
        .bare()
        .create(None, None, Some(compilation_params(1, false, true)))
        .await?;

    let delete_res = client
        .send()
        .delete(
            AppClientMethodCallParams {
                method: "delete_abi(string)string".to_string(),
                args: vec![abi_str_arg("string_io")],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    let abi_ret = delete_res.abi_return.expect("abi return expected");
    if let Some(algokit_abi::ABIValue::String(s)) = abi_ret.return_value {
        assert_eq!(s, "string_io");
    } else {
        panic!("expected string return");
    }
    Ok(())
}

#[rstest]
#[tokio::test]
async fn update_app_with_abi_direct(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(true),
            deletable: Some(false),
            ..Default::default()
        },
    )
    .await;

    // Initial create
    let (client, _create_res) = factory
        .send()
        .bare()
        .create(None, None, Some(compilation_params(1, true, false)))
        .await?;

    // Update via ABI (extra pages are auto-calculated internally)
    let update_res = client
        .send()
        .update(
            AppClientMethodCallParams {
                method: "update_abi(string)string".to_string(),
                args: vec![abi_str_arg("string_io")],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            Some(compilation_params(1, true, false)),
            None,
        )
        .await?;

    let abi_ret = update_res.abi_return.expect("abi return expected");
    if let Some(algokit_abi::ABIValue::String(s)) = abi_ret.return_value {
        assert_eq!(s, "string_io");
    } else {
        panic!("expected string return");
    }
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_when_immutable_and_permanent(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_deploy_when_immutable_and_permanent — L159-L170
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(false),
            deletable: Some(false),
            ..Default::default()
        },
    )
    .await;

    let _ = factory
        .deploy(
            Some(algokit_utils::applications::OnUpdate::Fail),
            Some(algokit_utils::applications::OnSchemaBreak::Fail),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_app_create(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_deploy_app_create — L173-L187
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(true),
            ..Default::default()
        },
    )
    .await;

    let (client, deploy_result) = factory
        .deploy(None, None, None, None, None, None, None, None, None)
        .await?;

    match &deploy_result.operation_performed {
        algokit_utils::applications::AppDeployResult::Create { .. } => {}
        _ => panic!("expected Create"),
    }
    assert!(client.app_id() > 0);
    assert_eq!(client.app_id(), deploy_result.app.app_id);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_app_create_abi(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_deploy_app_create_abi — L189-L206
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        algorand_client,
        test_account,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;

    let create_params = AppFactoryCreateMethodCallParams {
        method: "create_abi(string)string".to_string(),
        args: Some(vec![algokit_utils::AppMethodCallArg::ABIValue(
            algokit_abi::ABIValue::from("arg_io"),
        )]),
        ..Default::default()
    };

    let (client, deploy_result) = factory
        .deploy(
            None,
            None,
            Some(create_params),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;

    match &deploy_result.operation_performed {
        algokit_utils::applications::AppDeployResult::Create { .. } => {}
        _ => panic!("expected Create"),
    }
    assert!(client.app_id() > 0);
    assert_eq!(client.app_id(), deploy_result.app.app_id);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_app_update(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_deploy_app_update — L208-L241
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        Arc::clone(&algorand_client),
        test_account.clone(),
        AppFactoryOptions {
            app_name: Some("APP_NAME".to_string()),
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;

    // Initial create (updatable)
    let (_client1, create_res) = factory
        .deploy(None, None, None, None, None, None, None, None, None)
        .await?;
    match &create_res.operation_performed {
        algokit_utils::applications::AppDeployResult::Create { .. } => {}
        _ => panic!("expected Create"),
    }

    // Update
    let factory2 = build_testing_app_factory(
        Arc::clone(&algorand_client),
        test_account,
        AppFactoryOptions {
            app_name: Some("APP_NAME".to_string()),
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(2),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;

    let (_client2, update_res) = factory2
        .deploy(
            Some(algokit_utils::applications::OnUpdate::Update),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;

    match &update_res.operation_performed {
        algokit_utils::applications::AppDeployResult::Update { .. } => {}
        _ => panic!("expected Update"),
    }
    assert_eq!(create_res.app.app_id, update_res.app.app_id);
    assert_eq!(create_res.app.app_address, update_res.app.app_address);
    assert!(update_res.app.updated_round >= create_res.app.created_round);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_app_update_detects_extra_pages_as_breaking_change(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_deploy_app_update_detects_extra_pages_as_breaking_change — L243-L272
    let fixture = algorand_fixture.await?;
    // Factory with small program spec
    let small_spec = algokit_abi::Arc56Contract::from_json(
        algokit_test_artifacts::extra_pages_test::SMALL_ARC56,
    )
    .expect("valid arc56");
    let (algorand_client, test_account) = into_factory_inputs(fixture);
    let factory = build_app_factory_with_spec(
        Arc::clone(&algorand_client),
        test_account.clone(),
        small_spec,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(true),
            ..Default::default()
        },
    )
    .await;

    // Create using small
    let (_small_client, create_res) = factory
        .deploy(None, None, None, None, None, None, None, None, None)
        .await?;
    match &create_res.operation_performed {
        algokit_utils::applications::AppDeployResult::Create { .. } => {}
        _ => panic!("expected Create for small"),
    }

    // Switch to large spec and attempt update with Append schema break
    let large_spec = algokit_abi::Arc56Contract::from_json(
        algokit_test_artifacts::extra_pages_test::LARGE_ARC56,
    )
    .expect("valid arc56");
    let factory_large = build_app_factory_with_spec(
        algorand_client,
        test_account,
        large_spec,
        AppFactoryOptions {
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(2),
            )])),
            updatable: Some(true),
            ..Default::default()
        },
    )
    .await;

    let (large_client, update_res) = factory_large
        .deploy(
            Some(algokit_utils::applications::OnUpdate::Update),
            Some(algokit_utils::applications::OnSchemaBreak::Append),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;

    match &update_res.operation_performed {
        algokit_utils::applications::AppDeployResult::Create { .. } => {}
        _ => panic!("expected Create on schema break append"),
    }

    // App id should differ between small and large
    assert_ne!(create_res.app.app_id, large_client.app_id());
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_app_update_abi(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_deploy_app_update_abi — L274-L309
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        Arc::clone(&algorand_client),
        test_account.clone(),
        AppFactoryOptions {
            app_name: Some("APP_NAME".to_string()),
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;

    // Create updatable
    let _ = factory
        .deploy(None, None, None, None, None, None, None, None, None)
        .await?;

    // Update via ABI with VALUE=2 but same updatable/deletable
    let update_params = AppClientMethodCallParams {
        method: "update_abi(string)string".to_string(),
        args: vec![algokit_utils::AppMethodCallArg::ABIValue(
            algokit_abi::ABIValue::from("args_io"),
        )],
        ..Default::default()
    };
    let factory2 = build_testing_app_factory(
        Arc::clone(&algorand_client),
        test_account,
        AppFactoryOptions {
            app_name: Some("APP_NAME".to_string()),
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(2),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;
    let (_client2, update_res) = factory2
        .deploy(
            Some(algokit_utils::applications::OnUpdate::Update),
            None,
            None,
            Some(update_params),
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    match &update_res.operation_performed {
        algokit_utils::applications::AppDeployResult::Update { .. } => {}
        _ => panic!("expected Update"),
    }
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_app_replace(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_deploy_app_replace — L312-L350
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        Arc::clone(&algorand_client),
        test_account.clone(),
        AppFactoryOptions {
            app_name: Some("APP_NAME".to_string()),
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;

    let (_client1, create_res) = factory
        .deploy(None, None, None, None, None, None, None, None, None)
        .await?;
    let old_app_id = create_res.app.app_id;

    // Replace
    let factory2 = build_testing_app_factory(
        Arc::clone(&algorand_client),
        test_account,
        AppFactoryOptions {
            app_name: Some("APP_NAME".to_string()),
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(2),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;
    let (_client2, replace_res) = factory2
        .deploy(
            Some(algokit_utils::applications::OnUpdate::Replace),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    match &replace_res.operation_performed {
        algokit_utils::applications::AppDeployResult::Replace { .. } => {}
        _ => panic!("expected Replace"),
    }
    assert!(replace_res.app.app_id > old_app_id);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_app_replace_abi(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_deploy_app_replace_abi — L352-L394
    let fixture = algorand_fixture.await?;
    let (algorand_client, test_account) = into_factory_inputs(fixture);

    let factory = build_testing_app_factory(
        Arc::clone(&algorand_client),
        test_account.clone(),
        AppFactoryOptions {
            app_name: Some("APP_NAME".to_string()),
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(1),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;

    // Initial create
    let (_client1, create_res) = factory
        .deploy(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("APP_NAME".to_string()),
            None,
        )
        .await?;

    let old_app_id = create_res.app.app_id;

    // Replace via ABI create/delete
    let create_params = AppFactoryCreateMethodCallParams {
        method: "create_abi(string)string".to_string(),
        args: Some(vec![abi_str_arg("arg_io")]),
        ..Default::default()
    };
    let delete_params = AppClientMethodCallParams {
        method: "delete_abi(string)string".to_string(),
        args: vec![abi_str_arg("arg2_io")],
        ..Default::default()
    };
    let factory2 = build_testing_app_factory(
        Arc::clone(&algorand_client),
        test_account,
        AppFactoryOptions {
            app_name: Some("APP_NAME".to_string()),
            deploy_time_params: Some(HashMap::from([(
                "VALUE".to_string(),
                TealTemplateValue::Int(2),
            )])),
            updatable: Some(true),
            deletable: Some(true),
            ..Default::default()
        },
    )
    .await;
    let (_client2, replace_res) = factory2
        .deploy(
            Some(algokit_utils::applications::OnUpdate::Replace),
            None,
            Some(create_params),
            Some(delete_params),
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    match &replace_res.operation_performed {
        algokit_utils::applications::AppDeployResult::Replace { .. } => {}
        _ => panic!("expected Replace"),
    }
    assert!(replace_res.app.app_id > old_app_id);
    Ok(())
}
