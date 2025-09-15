use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIValue, Arc56Contract};
use algokit_transact::{BoxReference, SignedTransaction, Transaction};
use algokit_utils::applications::app_client::{
    AppClient, AppClientMethodCallParams, AppClientParams,
};
use algokit_utils::clients::app_manager::TealTemplateValue;
use algokit_utils::transactions::app_call::AppCallMethodCallParams;
use algokit_utils::transactions::composer::SimulateParams;
use algokit_utils::transactions::{PaymentParams, TransactionSigner, TransactionWithSigner};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg};
use async_trait::async_trait;
use rand::Rng;
use rstest::*;
use std::sync::{Arc, Mutex};

fn get_testing_app_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::testing_app::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

fn get_sandbox_spec() -> Arc56Contract {
    Arc56Contract::from_json(algokit_test_artifacts::sandbox::APPLICATION_ARC56)
        .expect("valid arc56")
}

#[rstest]
#[tokio::test]
async fn test_create_then_call_app(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

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
    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &get_testing_app_spec(),
        Some(tmpl),
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
        _ => panic!("Expected string ABI return"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_construct_transaction_with_abi_encoding_including_transaction(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

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

    let funded_account = fixture.generate_account(None).await?;
    let funded_addr = funded_account.account().address();

    let mut rng = rand::thread_rng();
    let amount: u64 = rng.gen_range(1..=10000);

    let payment_txn = fixture
        .algorand_client
        .create()
        .payment(PaymentParams {
            sender: funded_addr.clone(),
            receiver: funded_addr.clone(),
            amount,
            ..Default::default()
        })
        .await?;

    let client = AppClient::new(AppClientParams {
        app_id,
        app_spec: get_testing_app_spec(),
        algorand: fixture.algorand_client,
        app_name: None,
        default_sender: Some(funded_addr.to_string()),
        default_signer: None,
        source_maps: None,
        transaction_composer_config: None,
    });

    let result = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi_txn".to_string(),
                args: vec![
                    AppMethodCallArg::Transaction(payment_txn),
                    AppMethodCallArg::ABIValue(ABIValue::from("test")),
                ],
                sender: Some(funded_addr.to_string()),
                signer: Some(Arc::new(funded_account.clone())),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    assert_eq!(result.common_params.transactions.len(), 2);

    let abi_return = result.abi_return.as_ref().expect("Expected ABI return");
    let expected_return = format!("Sent {}. {}", amount, "test");
    match &abi_return.return_value {
        Some(ABIValue::String(s)) => assert_eq!(s, &expected_return),
        _ => panic!("Expected string ABI return"),
    }

    let method = get_testing_app_spec()
        .find_abi_method("call_abi_txn")
        .expect("ABI method");
    let decoded = algokit_utils::clients::app_manager::AppManager::get_abi_return(
        &abi_return.raw_return_value,
        &method,
    )
    .expect("Decoded ABI return");
    match decoded.return_value {
        Some(ABIValue::String(s)) => assert_eq!(s, expected_return),
        _ => panic!("Expected string ABI return from AppManager decoding"),
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_call_app_with_too_many_args(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

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
    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &get_testing_app_spec(),
        Some(tmpl),
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

    let err = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from("one")),
                    AppMethodCallArg::ABIValue(ABIValue::from("two")),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await
        .expect_err("Expected validation error due to too many args");

    assert!(
        err.to_string()
            .contains("The number of provided arguments is"),
        "Unexpected error message: {}",
        err
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_call_app_with_rekey(#[future] algorand_fixture: AlgorandFixtureResult) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let mut fixture = fixture;
    let rekey_to_account = fixture.generate_account(None).await?;
    let rekey_to_addr = rekey_to_account.account().address();

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

    client
        .send()
        .opt_in(
            AppClientMethodCallParams {
                method: "opt_in".to_string(),
                args: vec![],
                sender: Some(sender.to_string()),
                rekey_to: Some(rekey_to_addr.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    let _payment_result = client
        .algorand()
        .send()
        .payment(
            PaymentParams {
                sender: sender.clone(),
                signer: Some(Arc::new(rekey_to_account.clone())),
                receiver: sender.clone(),
                amount: 0,
                ..Default::default()
            },
            None,
        )
        .await?;

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_group_simulate_matches_send(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

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

    let set_global_method = get_testing_app_spec()
        .find_abi_method("set_global")
        .unwrap();
    let call_abi_method = get_testing_app_spec().find_abi_method("call_abi").unwrap();

    let app_call1_params = AppCallMethodCallParams {
        sender: sender.clone(),
        app_id,
        method: set_global_method,
        args: vec![
            AppMethodCallArg::ABIValue(ABIValue::from(1u64)),
            AppMethodCallArg::ABIValue(ABIValue::from(2u64)),
            AppMethodCallArg::ABIValue(ABIValue::from("asdf")),
            AppMethodCallArg::ABIValue(ABIValue::Array(vec![
                ABIValue::from_byte(1),
                ABIValue::from_byte(2),
                ABIValue::from_byte(3),
                ABIValue::from_byte(4),
            ])),
        ],
        on_complete: algokit_transact::OnApplicationComplete::NoOp,
        ..Default::default()
    };

    let payment_params = PaymentParams {
        sender: sender.clone(),
        receiver: sender.clone(),
        amount: 10_000,
        ..Default::default()
    };

    let app_call2_params = AppCallMethodCallParams {
        sender: sender.clone(),
        app_id,
        method: call_abi_method,
        args: vec![AppMethodCallArg::ABIValue(ABIValue::from("test"))],
        on_complete: algokit_transact::OnApplicationComplete::NoOp,
        ..Default::default()
    };

    let mut simulate_composer = algorand.new_group(None);
    simulate_composer.add_app_call_method_call(app_call1_params.clone())?;
    simulate_composer.add_payment(payment_params.clone())?;
    simulate_composer.add_app_call_method_call(app_call2_params.clone())?;
    let simulate_result = simulate_composer
        .simulate(Some(SimulateParams {
            skip_signatures: true,
            ..Default::default()
        }))
        .await?;

    let mut send_composer = algorand.new_group(None);
    send_composer.add_app_call_method_call(app_call1_params)?;
    send_composer.add_payment(payment_params)?;
    send_composer.add_app_call_method_call(app_call2_params)?;
    let send_result = send_composer.send(None).await?;

    assert_eq!(
        simulate_result.transaction_ids.len(),
        send_result.transaction_ids.len()
    );
    assert_eq!(
        simulate_result.abi_returns.len(),
        send_result.abi_returns.len()
    );
    assert_eq!(
        simulate_result.abi_returns[0].return_value,
        send_result.abi_returns[0].return_value
    );
    assert_eq!(
        simulate_result.abi_returns[1].return_value,
        send_result.abi_returns[1].return_value
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_sign_all_transactions_in_group_with_abi_call_with_transaction_arg(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

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

    let funded_account = fixture.generate_account(None).await?;
    let funded_addr = funded_account.account().address();

    let mut rng = rand::thread_rng();
    let amount = rng.gen_range(1..=10000);

    let payment_txn = fixture
        .algorand_client
        .create()
        .payment(PaymentParams {
            sender: funded_addr.clone(),
            receiver: funded_addr.clone(),
            amount,
            ..Default::default()
        })
        .await?;

    let called_indexes = Arc::new(Mutex::new(Vec::new()));

    struct IndexCapturingSigner {
        original_signer: Arc<dyn TransactionSigner>,
        called_indexes: Arc<Mutex<Vec<usize>>>,
    }

    #[async_trait]
    impl TransactionSigner for IndexCapturingSigner {
        async fn sign_transactions(
            &self,
            transactions: &[Transaction],
            indices: &[usize],
        ) -> Result<Vec<SignedTransaction>, String> {
            {
                let mut indexes = self.called_indexes.lock().unwrap();
                indexes.extend_from_slice(indices);
            }
            self.original_signer
                .sign_transactions(transactions, indices)
                .await
        }
    }

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

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi_txn".to_string(),
                args: vec![
                    AppMethodCallArg::Transaction(payment_txn),
                    AppMethodCallArg::ABIValue(ABIValue::from("test")),
                ],
                sender: Some(funded_addr.to_string()),
                signer: Some(Arc::new(IndexCapturingSigner {
                    original_signer: Arc::new(funded_account.clone()),
                    called_indexes: called_indexes.clone(),
                })),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let indexes = called_indexes.lock().unwrap().clone();

    assert_eq!(indexes, vec![0, 1], "Expected indexes 0 and 1 to be signed");

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_sign_transaction_in_group_with_different_signer_if_provided(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let mut fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

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
    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &get_testing_app_spec(),
        Some(tmpl),
        None,
        None,
    )
    .await?;

    let new_account = fixture.generate_account(None).await?;
    let new_addr = new_account.account().address();

    let payment_txn = fixture
        .algorand_client
        .create()
        .payment(PaymentParams {
            sender: new_addr.clone(),
            receiver: new_addr.clone(),
            amount: 2_000,
            ..Default::default()
        })
        .await?;

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

    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "call_abi_txn".to_string(),
                args: vec![
                    AppMethodCallArg::TransactionWithSigner(TransactionWithSigner {
                        transaction: payment_txn,
                        signer: Arc::new(new_account.clone()),
                    }),
                    AppMethodCallArg::ABIValue(ABIValue::from("test")),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    Ok(())
}

#[rstest]
#[tokio::test]
async fn bare_call_with_box_reference_builds_and_sends(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let app_id =
        deploy_arc56_contract(&fixture, &sender, &get_sandbox_spec(), None, None, None).await?;

    let mut algorand = RootAlgorandClient::default_localnet(None);
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));

    let client = AppClient::new(AppClientParams {
        app_id,
        app_spec: get_sandbox_spec(),
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        source_maps: None,
        transaction_composer_config: None,
    });

    let result = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "hello_world".to_string(),
                args: vec![AppMethodCallArg::ABIValue(ABIValue::from("test"))],
                sender: Some(sender.to_string()),
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: b"1".to_vec(),
                }]),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    match &result.common_params.transaction {
        algokit_transact::Transaction::AppCall(fields) => {
            assert_eq!(fields.app_id, app_id);
            assert_eq!(
                fields.box_references.as_ref().unwrap(),
                &vec![BoxReference {
                    app_id: 0,
                    name: b"1".to_vec()
                }]
            );
        }
        _ => panic!("expected app call"),
    }

    Ok(())
}
