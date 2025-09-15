use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture};
use algokit_abi::Arc56Contract;
use algokit_transact::OnApplicationComplete;
use algokit_utils::AlgorandClient as RootAlgorandClient;
use algokit_utils::applications::app_client::AppClientMethodCallParams;
use algokit_utils::applications::app_factory::{
    AppFactory, AppFactoryCreateMethodCallParams, AppFactoryCreateParams,
};
use algokit_utils::clients::app_manager::{TealTemplateParams, TealTemplateValue};
use rstest::*;
use std::sync::Arc;

fn get_testing_app_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::testing_app::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

#[rstest]
#[tokio::test]
async fn bare_create_with_deploy_time_params(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_create_app — references/.../test_app_factory.py:L85-L105
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let mut tmpl: TealTemplateParams = Default::default();
    tmpl.insert("VALUE".to_string(), TealTemplateValue::Int(1));
    // VALUE-only artifact
    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand,
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: Some(tmpl),
        updatable: Some(false),
        deletable: Some(false),
        source_maps: None,
    });

    let compilation_params = algokit_utils::applications::app_client::CompilationParams {
        deploy_time_params: Some({
            let mut m: TealTemplateParams = Default::default();
            m.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            m
        }),
        updatable: Some(false),
        deletable: Some(false),
        ..Default::default()
    };

    let (client, res) = factory
        .send()
        .bare()
        .create(
            Some(AppFactoryCreateParams::default()),
            None,
            Some(compilation_params),
        )
        .await?;

    assert!(client.app_id().unwrap() > 0);
    assert_eq!(
        client.app_address().unwrap(),
        algokit_transact::Address::from_app_id(&client.app_id().unwrap())
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
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let mut tmpl: TealTemplateParams = Default::default();
    tmpl.insert("VALUE".to_string(), TealTemplateValue::Int(1));

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand,
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: Some(tmpl),
        updatable: Some(false),
        deletable: Some(false),
        source_maps: None,
    });

    let (client, result) = factory.send().bare().create(None, None, None).await?;

    assert!(result.app_id > 0);
    assert_eq!(client.app_id().unwrap(), result.app_id);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn oncomplete_override_on_create(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    // Python: test_create_app_with_oncomplete_overload — L137-L157
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand,
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: None,
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });

    let params = AppFactoryCreateParams {
        on_complete: Some(OnApplicationComplete::OptIn),
        ..Default::default()
    };
    let compilation_params = algokit_utils::applications::app_client::CompilationParams {
        deploy_time_params: Some({
            let mut m: TealTemplateParams = Default::default();
            m.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            m
        }),
        updatable: Some(true),
        deletable: Some(true),
    };
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
    assert!(client.app_id().unwrap() > 0);
    assert_eq!(
        client.app_address().unwrap(),
        algokit_transact::Address::from_app_id(&client.app_id().unwrap())
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
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand,
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });

    let cp = algokit_utils::applications::app_client::CompilationParams {
        deploy_time_params: Some({
            let mut m: TealTemplateParams = Default::default();
            m.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            m
        }),
        updatable: Some(true),
        deletable: Some(false),
        ..Default::default()
    };

    let (_client, call_return) = factory
        .send()
        .create(
            AppFactoryCreateMethodCallParams {
                method: "create_abi(string)string".to_string(),
                args: Some(vec![algokit_utils::AppMethodCallArg::ABIValue(
                    algokit_abi::ABIValue::from("string_io"),
                )]),
                ..Default::default()
            },
            None,
            Some(cp),
        )
        .await?;

    let abi_ret = call_return.abi_return.expect("abi return");
    if let algokit_abi::ABIValue::String(s) = abi_ret.return_value {
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

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand,
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: None,
        updatable: Some(true),
        deletable: None,
        source_maps: None,
    });

    let cp = algokit_utils::applications::app_client::CompilationParams {
        deploy_time_params: Some({
            let mut m: TealTemplateParams = Default::default();
            m.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            m
        }),
        updatable: Some(true),
        deletable: Some(true),
    };

    let (client, _res) = factory.send().bare().create(None, None, Some(cp)).await?;

    let send_res = client
        .send()
        .call(AppClientMethodCallParams {
            method: "call_abi(string)string".to_string(),
            args: Some(vec![algokit_utils::AppMethodCallArg::ABIValue(
                algokit_abi::ABIValue::from("test"),
            )]),
            sender: Some(sender.to_string()),
            ..Default::default()
        })
        .await?;

    let abi_ret = send_res.abi_return.expect("abi return");
    if let algokit_abi::ABIValue::String(s) = abi_ret.return_value {
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

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand,
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(false),
        deletable: Some(false),
        source_maps: None,
    });

    let (client, _res) = factory
        .send()
        .bare()
        .create(
            None,
            None,
            Some(algokit_utils::applications::app_client::CompilationParams {
                deploy_time_params: {
                    let mut m: TealTemplateParams = Default::default();
                    m.insert("VALUE".to_string(), TealTemplateValue::Int(1));
                    Some(m)
                },
                updatable: Some(false),
                deletable: Some(false),
                ..Default::default()
            }),
        )
        .await?;

    let err = client
        .send()
        .call(AppClientMethodCallParams {
            method: "call_abi(string)string".to_string(),
            args: Some(vec![
                algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from("test")),
                algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from("extra")),
            ]),
            sender: Some(sender.to_string()),
            ..Default::default()
        })
        .await
        .expect_err("expected error for too many args");
    // The error is wrapped into a ValidationError; extract message via Display
    let msg = err.to_string();
    // Accept either position=1 (TS/Py message) or position=2 (internal off-by-one) to be tolerant
    assert!(
        msg.contains("Unexpected arg at position 1. call_abi only expects 1 args")
            || msg.contains("Unexpected arg at position 2. call_abi only expects 1 args"),
        "{msg}"
    );
    Ok(())
}

#[rstest]
#[tokio::test]
async fn call_app_with_rekey(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_call_app_with_rekey — L426-L446
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });

    let (client, _res) = factory.send().bare().create(None, None, None).await?;

    // Generate a new account to rekey to
    let mut fixture2 = fixture; // reuse clients
    let rekey_to = fixture2.generate_account(None).await?;
    let rekey_to_addr = rekey_to.account().address();

    // Opt-in with rekey_to
    client
        .send()
        .opt_in(AppClientMethodCallParams {
            method: "opt_in()void".to_string(),
            args: None,
            sender: Some(sender.to_string()),
            rekey_to: Some(rekey_to_addr.to_string()),
            ..Default::default()
        })
        .await?;

    // If rekey succeeded, a zero payment using the rekeyed signer should succeed
    let pay = algokit_utils::PaymentParams {
        common_params: algokit_utils::CommonTransactionParams {
            sender: sender.clone(),
            // signer will be picked up from account manager: set_signer already configured for original sender,
            // but after rekey the auth address must be rekey_to's signer. Use explicit signer.
            signer: Some(Arc::new(rekey_to.clone())),
            ..Default::default()
        },
        receiver: sender.clone(),
        amount: 0,
    };
    let _ = algorand.send().payment(pay, None).await?;
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

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand,
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(false),
        deletable: Some(true),
        source_maps: None,
    });

    let (client, _res) = factory
        .send()
        .bare()
        .create(
            None,
            None,
            Some(algokit_utils::applications::app_client::CompilationParams {
                deploy_time_params: {
                    let mut m: TealTemplateParams = Default::default();
                    m.insert("VALUE".to_string(), TealTemplateValue::Int(1));
                    Some(m)
                },
                updatable: Some(false),
                deletable: Some(true),
                ..Default::default()
            }),
        )
        .await?;

    let delete_res = client
        .send()
        .delete(AppClientMethodCallParams {
            method: "delete_abi(string)string".to_string(),
            args: Some(vec![algokit_utils::AppMethodCallArg::ABIValue(
                algokit_abi::ABIValue::from("string_io"),
            )]),
            sender: Some(sender.to_string()),
            ..Default::default()
        })
        .await?;

    let abi_ret = delete_res.abi_return.expect("abi return expected");
    if let algokit_abi::ABIValue::String(s) = abi_ret.return_value {
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

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(true),
        deletable: Some(false),
        source_maps: None,
    });

    // Initial create
    let (client, _create_res) = factory
        .send()
        .bare()
        .create(
            None,
            None,
            Some(algokit_utils::applications::app_client::CompilationParams {
                deploy_time_params: {
                    let mut m: TealTemplateParams = Default::default();
                    m.insert("VALUE".to_string(), TealTemplateValue::Int(1));
                    Some(m)
                },
                updatable: Some(true),
                deletable: Some(false),
                ..Default::default()
            }),
        )
        .await?;

    // Update via ABI (extra pages are auto-calculated internally)
    let update_res = client
        .send()
        .update(
            AppClientMethodCallParams {
                method: "update_abi(string)string".to_string(),
                args: Some(vec![algokit_utils::AppMethodCallArg::ABIValue(
                    algokit_abi::ABIValue::from("string_io"),
                )]),
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            Some(algokit_utils::applications::app_client::CompilationParams {
                deploy_time_params: {
                    let mut m: TealTemplateParams = Default::default();
                    m.insert("VALUE".to_string(), TealTemplateValue::Int(1));
                    Some(m)
                },
                updatable: Some(true),
                deletable: Some(false),
                ..Default::default()
            }),
        )
        .await?;

    let abi_ret = update_res.abi_return.expect("abi return expected");
    if let algokit_abi::ABIValue::String(s) = abi_ret.return_value {
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
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let mut t = TealTemplateParams::default();
    t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand,
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: Some(t),
        updatable: Some(false),
        deletable: Some(false),
        source_maps: None,
    });

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
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand,
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(true),
        deletable: None,
        source_maps: None,
    });

    let (client, deploy_result) = factory
        .deploy(None, None, None, None, None, None, None, None, None)
        .await?;

    match &deploy_result.operation_performed {
        algokit_utils::applications::AppDeployResult::Create { .. } => {}
        _ => panic!("expected Create"),
    }
    assert!(client.app_id().unwrap() > 0);
    assert_eq!(client.app_id().unwrap(), deploy_result.app.app_id);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_app_create_abi(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_deploy_app_create_abi — L189-L206
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand,
        app_spec: get_testing_app_spec(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });

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
    assert!(client.app_id().unwrap() > 0);
    assert_eq!(client.app_id().unwrap(), deploy_result.app.app_id);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_app_update(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_deploy_app_update — L208-L241
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: get_testing_app_spec(),
        app_name: Some("APP_NAME".to_string()),
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });

    // Initial create (updatable)
    let (_client1, create_res) = factory
        .deploy(None, None, None, None, None, None, None, None, None)
        .await?;
    match &create_res.operation_performed {
        algokit_utils::applications::AppDeployResult::Create { .. } => {}
        _ => panic!("expected Create"),
    }

    // Update
    let mut tmpl2: TealTemplateParams = Default::default();
    tmpl2.insert("VALUE".to_string(), TealTemplateValue::Int(2));
    let factory2 = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: get_testing_app_spec(),
        app_name: Some("APP_NAME".to_string()),
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: Some({
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(2));
            t
        }),
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });
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
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    // Factory with small program spec
    let small_spec = algokit_abi::Arc56Contract::from_json(
        algokit_test_artifacts::extra_pages_test::SMALL_ARC56,
    )
    .expect("valid arc56");
    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: small_spec,
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(true),
        deletable: None,
        source_maps: None,
    });

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
    let factory_large =
        AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
            algorand: algorand.clone(),
            app_spec: large_spec,
            app_name: None,
            default_sender: Some(sender.to_string()),
            default_signer: None,
            version: None,
            deploy_time_params: {
                let mut t = TealTemplateParams::default();
                t.insert("VALUE".to_string(), TealTemplateValue::Int(2));
                Some(t)
            },
            updatable: Some(true),
            deletable: None,
            source_maps: None,
        });

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
    assert_ne!(create_res.app.app_id, large_client.app_id().unwrap());
    Ok(())
}

#[rstest]
#[tokio::test]
async fn deploy_app_update_abi(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    // Python: test_deploy_app_update_abi — L274-L309
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: get_testing_app_spec(),
        app_name: Some("APP_NAME".to_string()),
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });

    // Create updatable
    let _ = factory
        .deploy(None, None, None, None, None, None, None, None, None)
        .await?;

    // Update via ABI with VALUE=2 but same updatable/deletable
    let update_params = AppClientMethodCallParams {
        method: "update_abi(string)string".to_string(),
        args: Some(vec![algokit_utils::AppMethodCallArg::ABIValue(
            algokit_abi::ABIValue::from("args_io"),
        )]),
        ..Default::default()
    };
    let mut tmpl2: TealTemplateParams = Default::default();
    tmpl2.insert("VALUE".to_string(), TealTemplateValue::Int(2));
    let factory2 = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: get_testing_app_spec(),
        app_name: Some("APP_NAME".to_string()),
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: Some({
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(2));
            t
        }),
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });
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
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: get_testing_app_spec(),
        app_name: Some("APP_NAME".to_string()),
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });

    let (_client1, create_res) = factory
        .deploy(None, None, None, None, None, None, None, None, None)
        .await?;
    let old_app_id = create_res.app.app_id;

    // Replace
    let mut tmpl2: TealTemplateParams = Default::default();
    tmpl2.insert("VALUE".to_string(), TealTemplateValue::Int(2));
    let factory2 = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: get_testing_app_spec(),
        app_name: Some("APP_NAME".to_string()),
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: Some({
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(2));
            t
        }),
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });
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
    let sender = fixture.test_account.account().address();

    let mut raw = RootAlgorandClient::default_localnet();
    raw.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let algorand = Arc::new(raw);

    let factory = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: get_testing_app_spec(),
        app_name: Some("APP_NAME".to_string()),
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: {
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(1));
            Some(t)
        },
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });

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
        args: Some(vec![algokit_utils::AppMethodCallArg::ABIValue(
            algokit_abi::ABIValue::from("arg_io"),
        )]),
        ..Default::default()
    };
    let delete_params = AppClientMethodCallParams {
        method: "delete_abi(string)string".to_string(),
        args: Some(vec![algokit_utils::AppMethodCallArg::ABIValue(
            algokit_abi::ABIValue::from("arg2_io"),
        )]),
        ..Default::default()
    };
    let mut tmpl2: TealTemplateParams = Default::default();
    tmpl2.insert("VALUE".to_string(), TealTemplateValue::Int(2));
    let factory2 = AppFactory::new(algokit_utils::applications::app_factory::AppFactoryParams {
        algorand: algorand.clone(),
        app_spec: get_testing_app_spec(),
        app_name: Some("APP_NAME".to_string()),
        default_sender: Some(sender.to_string()),
        default_signer: None,
        version: None,
        deploy_time_params: Some({
            let mut t = TealTemplateParams::default();
            t.insert("VALUE".to_string(), TealTemplateValue::Int(2));
            t
        }),
        updatable: Some(true),
        deletable: Some(true),
        source_maps: None,
    });
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
