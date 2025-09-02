use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::Arc56Contract;
use algokit_transact::BoxReference;
use algokit_utils::applications::app_client::{
    AppClient, AppClientBareCallParams, AppClientJsonParams, AppClientParams,
};
use algokit_utils::clients::app_manager::{
    DeploymentMetadata, TealTemplateParams, TealTemplateValue,
};
use algokit_utils::{AlgorandClient as RootAlgorandClient, transactions::composer::SimulateParams};
use rstest::*;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

fn get_testing_app_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::testing_app::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

#[test]
fn app_client_from_network_works() {
    let algorand = RootAlgorandClient::default_localnet();
    // JSON constructor
    let json = algokit_test_artifacts::state_management_demo::APPLICATION_ARC56;
    let client = AppClient::from_json(AppClientJsonParams {
        app_id: None,
        app_spec_json: json,
        algorand,
        app_name: None,
        default_sender: None,
        source_maps: None,
    })
    .expect("app client from json");
    assert!(!client.app_spec().methods.is_empty());
}

fn get_sandbox_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::sandbox::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

#[rstest]
#[tokio::test]
async fn retrieve_state() -> TestResult {
    use algokit_utils::applications::app_client::AppClientMethodCallParams as MCP;

    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();

    // Deploy testing_app
    let mut tmpl: TealTemplateParams = Default::default();
    tmpl.insert("VALUE".to_string(), TealTemplateValue::Int(0));
    tmpl.insert("UPDATABLE".to_string(), TealTemplateValue::Int(0));
    tmpl.insert("DELETABLE".to_string(), TealTemplateValue::Int(0));
    let app_id =
        deploy_arc56_contract(&fixture, &sender, &get_testing_app_spec(), Some(tmpl), None).await?;

    let mut algorand = RootAlgorandClient::default_localnet();
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let client = AppClient::new(AppClientParams {
        app_id: Some(app_id),
        app_spec: get_testing_app_spec(),
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        source_maps: None,
    });

    // Global state: set and verify
    client
        .send()
        .call(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "set_global(uint64,uint64,string,byte[4])void".to_string(),
                args: Some(vec![
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(1u64)),
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(2u64)),
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from("asdf")),
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::Array(vec![
                        algokit_abi::ABIValue::from_byte(1),
                        algokit_abi::ABIValue::from_byte(2),
                        algokit_abi::ABIValue::from_byte(3),
                        algokit_abi::ABIValue::from_byte(4),
                    ])),
                ]),
                sender: Some(sender.to_string()),
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: None,
                on_complete: None,
            },
        )
        .await?;

    let global_state = client.state().global_state().get_all().await?;
    assert!(global_state.contains_key("int1"));
    assert!(global_state.contains_key("int2"));
    assert!(global_state.contains_key("bytes1"));
    assert!(global_state.contains_key("bytes2"));
    assert_eq!(
        global_state.get("int1").unwrap(),
        &algokit_abi::ABIValue::from(1u64)
    );
    assert_eq!(
        global_state.get("int2").unwrap(),
        &algokit_abi::ABIValue::from(2u64)
    );
    assert_eq!(
        global_state.get("bytes1").unwrap(),
        &algokit_abi::ABIValue::from("asdf")
    );

    // Local: opt-in and set; verify
    client
        .send()
        .opt_in(MCP {
            method: "opt_in()void".to_string(),
            args: None,
            sender: Some(sender.to_string()),
            rekey_to: None,
            note: None,
            lease: None,
            static_fee: None,
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: None,
            on_complete: None,
        })
        .await?;

    client
        .send()
        .call(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "set_local(uint64,uint64,string,byte[4])void".to_string(),
                args: Some(vec![
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(1u64)),
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(2u64)),
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from("asdf")),
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::Array(vec![
                        algokit_abi::ABIValue::from_byte(1),
                        algokit_abi::ABIValue::from_byte(2),
                        algokit_abi::ABIValue::from_byte(3),
                        algokit_abi::ABIValue::from_byte(4),
                    ])),
                ]),
                sender: Some(sender.to_string()),
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: None,
                on_complete: None,
            },
        )
        .await?;

    let local_state = client
        .state()
        .local_state(&sender.to_string())
        .get_all()
        .await?;
    assert_eq!(
        local_state.get("local_int1").unwrap(),
        &algokit_abi::ABIValue::from(1u64)
    );
    assert_eq!(
        local_state.get("local_int2").unwrap(),
        &algokit_abi::ABIValue::from(2u64)
    );
    assert_eq!(
        local_state.get("local_bytes1").unwrap(),
        &algokit_abi::ABIValue::from("asdf")
    );

    // Boxes
    let box_name1: Vec<u8> = vec![0, 0, 0, 1];
    let box_name2: Vec<u8> = vec![0, 0, 0, 2];

    // Fund app account to enable box writes
    client
        .fund_app_account(
            algokit_utils::applications::app_client::FundAppAccountParams {
                amount: 1_000_000,
                sender: Some(sender.to_string()),
                ..Default::default()
            },
        )
        .await?;

    client
        .send()
        .call(MCP {
            method: "set_box(byte[4],string)void".to_string(),
            args: Some(vec![
                algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::Array(
                    box_name1
                        .clone()
                        .into_iter()
                        .map(algokit_abi::ABIValue::from_byte)
                        .collect(),
                )),
                algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from("value1")),
            ]),
            sender: Some(sender.to_string()),
            rekey_to: None,
            note: None,
            lease: None,
            static_fee: None,
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: Some(vec![BoxReference {
                app_id: 0,
                name: box_name1.clone(),
            }]),
            on_complete: None,
        })
        .await?;

    client
        .send()
        .call(MCP {
            method: "set_box(byte[4],string)void".to_string(),
            args: Some(vec![
                algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::Array(
                    box_name2
                        .clone()
                        .into_iter()
                        .map(algokit_abi::ABIValue::from_byte)
                        .collect(),
                )),
                algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from("value2")),
            ]),
            sender: Some(sender.to_string()),
            rekey_to: None,
            note: None,
            lease: None,
            static_fee: None,
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: Some(vec![BoxReference {
                app_id: 0,
                name: box_name2.clone(),
            }]),
            on_complete: None,
        })
        .await?;

    let box_names = client.get_box_names().await?;
    let names: Vec<Vec<u8>> = box_names.into_iter().map(|n| n.name_raw).collect();
    assert!(names.contains(&box_name1));
    assert!(names.contains(&box_name2));

    let box1_value = client.get_box_value(&box_name1).await?;
    assert_eq!(box1_value, b"value1");

    Ok(())
}

#[rstest]
#[tokio::test]
async fn logic_error_exposure_with_source_maps() -> TestResult {
    use algokit_utils::applications::app_client::AppClientMethodCallParams as MCP;
    use algokit_utils::applications::app_client::AppSourceMaps;
    use algokit_utils::transactions::sender_results::TransactionResultError;

    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();

    // Deploy testing_app with template params
    let mut tmpl: TealTemplateParams = Default::default();
    tmpl.insert("VALUE".to_string(), TealTemplateValue::Int(0));
    tmpl.insert("UPDATABLE".to_string(), TealTemplateValue::Int(0));
    tmpl.insert("DELETABLE".to_string(), TealTemplateValue::Int(0));
    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &get_testing_app_spec(),
        Some(tmpl.clone()),
        None,
    )
    .await?;

    let mut algorand = RootAlgorandClient::default_localnet();
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let mut client = AppClient::new(AppClientParams {
        app_id: Some(app_id),
        app_spec: get_testing_app_spec(),
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        source_maps: None,
    });

    // Compile TEAL to get source maps and import
    let src = client.app_spec().source.as_ref().expect("source expected");
    let approval_teal = src.get_decoded_approval().unwrap();
    let clear_teal = src.get_decoded_clear().unwrap();
    let app_manager = fixture.algorand_client.app();
    let compiled_approval = app_manager
        .compile_teal_template(&approval_teal, Some(&tmpl), None)
        .await?;
    let compiled_clear = app_manager
        .compile_teal_template(&clear_teal, Some(&tmpl), None)
        .await?;
    client.import_source_maps(AppSourceMaps {
        approval_source_map: compiled_approval.source_map,
        clear_source_map: compiled_clear.source_map,
    });

    // Trigger logic error
    let err = client
        .send()
        .call(MCP {
            method: "error()void".to_string(),
            args: None,
            sender: Some(sender.to_string()),
            rekey_to: None,
            note: None,
            lease: None,
            static_fee: None,
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: None,
            on_complete: None,
        })
        .await
        .expect_err("expected logic error");

    let logic = client.expose_logic_error(
        &TransactionResultError::ParsingError {
            message: err.to_string(),
        },
        false,
    );
    assert!(logic.pc.is_some());
    assert!(logic.logic_error_str.contains("assert failed"));
    if let Some(id) = &logic.transaction_id {
        assert!(id.len() >= 52);
    }
    Ok(())
}

#[rstest]
#[tokio::test]
async fn box_methods_with_manually_encoded_abi_args() -> TestResult {
    use algokit_utils::applications::app_client::AppClientMethodCallParams as MCP;

    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();

    // Deploy testing_app_puya
    let json = algokit_test_artifacts::testing_app_puya::APPLICATION_ARC56;
    let spec = Arc56Contract::from_json(json).expect("valid arc56");
    let app_id = deploy_arc56_contract(&fixture, &sender, &spec, None, None).await?;

    let mut algorand = RootAlgorandClient::default_localnet();
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let client = AppClient::new(AppClientParams {
        app_id: Some(app_id),
        app_spec: spec,
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        source_maps: None,
    });

    // Fund app account
    client
        .fund_app_account(
            algokit_utils::applications::app_client::FundAppAccountParams {
                amount: 1_000_000,
                sender: Some(sender.to_string()),
                ..Default::default()
            },
        )
        .await?;

    // Prepare box name and encoded value
    let box_prefix = b"box_bytes".to_vec();
    let name_type = algokit_abi::ABIType::from_str("string").unwrap();
    let box_name = "name1";
    let box_name_encoded = name_type
        .encode(&algokit_abi::ABIValue::from(box_name))
        .unwrap();
    let box_identifier = {
        let mut v = box_prefix.clone();
        v.extend_from_slice(&box_name_encoded);
        v
    };

    // byte[] value
    let value_type = algokit_abi::ABIType::from_str("byte[]").unwrap();
    let encoded = value_type
        .encode(&algokit_abi::ABIValue::from(vec![
            algokit_abi::ABIValue::from_byte(116),
            algokit_abi::ABIValue::from_byte(101),
            algokit_abi::ABIValue::from_byte(115),
            algokit_abi::ABIValue::from_byte(116),
            algokit_abi::ABIValue::from_byte(95),
            algokit_abi::ABIValue::from_byte(98),
            algokit_abi::ABIValue::from_byte(121),
            algokit_abi::ABIValue::from_byte(116),
            algokit_abi::ABIValue::from_byte(101),
            algokit_abi::ABIValue::from_byte(115),
        ]))
        .unwrap();

    client
        .send()
        .call(MCP {
            method: "set_box_bytes(string,byte[])void".to_string(),
            args: Some(vec![
                algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(box_name)),
                algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::Array(
                    encoded
                        .into_iter()
                        .map(algokit_abi::ABIValue::from_byte)
                        .collect(),
                )),
            ]),
            sender: Some(sender.to_string()),
            rekey_to: None,
            note: None,
            lease: None,
            static_fee: None,
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: Some(vec![BoxReference {
                app_id: 0,
                name: box_identifier.clone(),
            }]),
            on_complete: None,
        })
        .await?;

    let retrieved = client
        .get_box_value_from_abi_type(&box_identifier, &value_type)
        .await?;
    assert_eq!(
        retrieved,
        algokit_abi::ABIValue::Array(vec![
            algokit_abi::ABIValue::from_byte(116),
            algokit_abi::ABIValue::from_byte(101),
            algokit_abi::ABIValue::from_byte(115),
            algokit_abi::ABIValue::from_byte(116),
            algokit_abi::ABIValue::from_byte(95),
            algokit_abi::ABIValue::from_byte(98),
            algokit_abi::ABIValue::from_byte(121),
            algokit_abi::ABIValue::from_byte(116),
            algokit_abi::ABIValue::from_byte(101),
            algokit_abi::ABIValue::from_byte(115),
        ])
    );

    Ok(())
}
async fn construct_transaction_with_abi_encoding_including_foreign_references_not_in_signature()
-> TestResult {
    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();
    // Deploy testing_app which has call_abi_foreign_refs()string
    let mut tmpl: TealTemplateParams = Default::default();
    tmpl.insert("VALUE".to_string(), TealTemplateValue::Int(0));
    tmpl.insert("UPDATABLE".to_string(), TealTemplateValue::Int(0));
    tmpl.insert("DELETABLE".to_string(), TealTemplateValue::Int(0));
    let app_id =
        deploy_arc56_contract(&fixture, &sender, &get_testing_app_spec(), Some(tmpl), None).await?;

    let mut algorand = RootAlgorandClient::default_localnet();
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));

    let client = AppClient::new(AppClientParams {
        app_id: Some(app_id),
        app_spec: get_testing_app_spec(),
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        source_maps: None,
    });

    // Create a secondary account for account_references
    let mut new_algorand = RootAlgorandClient::default_localnet();
    let mut new_fixture = fixture; // reuse underlying clients for funding convenience
    let new_account = new_fixture.generate_account(None).await?;
    let new_addr = new_account.account().address();

    let send_res = client
        .send()
        .call(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "call_abi_foreign_refs()string".to_string(),
                args: None,
                sender: Some(sender.to_string()),
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
                account_references: Some(vec![new_addr.to_string()]),
                app_references: Some(vec![345]),
                asset_references: Some(vec![567]),
                box_references: None,
                on_complete: None,
            },
        )
        .await?;

    let abi_ret = send_res.abi_return.as_ref().expect("abi return expected");
    if let algokit_abi::ABIValue::String(s) = &abi_ret.return_value {
        assert!(s.contains("App: 345"));
        assert!(s.contains("Asset: 567"));
    } else {
        panic!("expected string return");
    }
    Ok(())
}

#[rstest]
#[tokio::test]
async fn abi_with_default_arg_from_local_state() -> TestResult {
    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();
    let mut tmpl: TealTemplateParams = Default::default();
    tmpl.insert("VALUE".to_string(), TealTemplateValue::Int(0));
    tmpl.insert("UPDATABLE".to_string(), TealTemplateValue::Int(0));
    tmpl.insert("DELETABLE".to_string(), TealTemplateValue::Int(0));
    let app_id =
        deploy_arc56_contract(&fixture, &sender, &get_testing_app_spec(), Some(tmpl), None).await?;

    let mut algorand = RootAlgorandClient::default_localnet();
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));

    let client = AppClient::new(AppClientParams {
        app_id: Some(app_id),
        app_spec: get_testing_app_spec(),
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        source_maps: None,
    });

    // Opt-in and set local state
    client
        .send()
        .opt_in(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "opt_in()void".to_string(),
                args: None,
                sender: Some(sender.to_string()),
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: None,
                on_complete: None,
            },
        )
        .await?;

    client
        .send()
        .call(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "set_local(uint64,uint64,string,byte[4])void".to_string(),
                args: Some(vec![
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(1u64)),
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(2u64)),
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(
                        "banana",
                    )),
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::Array(vec![
                        algokit_abi::ABIValue::from_byte(1),
                        algokit_abi::ABIValue::from_byte(2),
                        algokit_abi::ABIValue::from_byte(3),
                        algokit_abi::ABIValue::from_byte(4),
                    ])),
                ]),
                sender: Some(sender.to_string()),
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: None,
                on_complete: None,
            },
        )
        .await?;

    // Debug: fetch local state and print expected key
    let local_state = fixture
        .algorand_client
        .app()
        .get_local_state(app_id, &sender.to_string())
        .await?;
    if let Some(val) = local_state.get("local_bytes1".as_bytes()) {
        println!(
            "local_bytes1 -> value_raw: {:?}, value: {:?}",
            val.value_raw, val.value
        );
    } else {
        println!("local_bytes1 not found in local state");
    }

    // Call method without providing arg; expect default from local state
    let res = client
        .send()
        .call(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "default_value_from_local_state(string)string".to_string(),
                args: None, // missing arg to trigger default resolver
                sender: Some(sender.to_string()),
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: None,
                on_complete: None,
            },
        )
        .await?;

    let abi_ret = res.abi_return.as_ref().expect("abi return expected");
    if let algokit_abi::ABIValue::String(s) = &abi_ret.return_value {
        println!("method returned: {}", s);
        assert!(s.contains("Local state"));
        assert!(s.contains("banana"));
    } else {
        panic!("expected string return");
    }
    Ok(())
}
#[rstest]
#[tokio::test]
async fn bare_call_with_box_reference_builds_and_sends() -> TestResult {
    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();

    let app_id = deploy_arc56_contract(&fixture, &sender, &get_sandbox_spec(), None, None).await?;

    let mut algorand = RootAlgorandClient::default_localnet();
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));

    let client = AppClient::new(AppClientParams {
        app_id: Some(app_id),
        app_spec: get_sandbox_spec(),
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        source_maps: None,
    });

    let params = AppClientBareCallParams {
        args: None,
        sender: Some(sender.to_string()),
        rekey_to: None,
        note: None,
        lease: None,
        static_fee: None,
        extra_fee: None,
        max_fee: None,
        validity_window: None,
        first_valid_round: None,
        last_valid_round: None,
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: Some(vec![BoxReference {
            app_id: 0,
            name: b"1".to_vec(),
        }]),
        on_complete: None,
    };

    // Use method call (sandbox does not allow bare NoOp)
    let result = client
        .send()
        .call(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "hello_world(string)string".to_string(),
                args: Some(vec![algokit_utils::AppMethodCallArg::ABIValue(
                    algokit_abi::ABIValue::from("test"),
                )]),
                sender: Some(sender.to_string()),
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: b"1".to_vec(),
                }]),
                on_complete: None,
            },
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

#[rstest]
#[tokio::test]
async fn construct_transaction_with_boxes() -> TestResult {
    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();
    let app_id = deploy_arc56_contract(&fixture, &sender, &get_sandbox_spec(), None, None).await?;

    let mut algorand = RootAlgorandClient::default_localnet();
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));

    let client = AppClient::new(AppClientParams {
        app_id: Some(app_id),
        app_spec: get_sandbox_spec(),
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        source_maps: None,
    });

    // Build transaction with a box reference
    let built = client
        .create_transaction()
        .call(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "hello_world(string)string".to_string(),
                args: Some(vec![algokit_utils::AppMethodCallArg::ABIValue(
                    algokit_abi::ABIValue::from("test"),
                )]),
                sender: Some(sender.to_string()),
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: b"1".to_vec(),
                }]),
                on_complete: None,
            },
        )
        .await?;
    match built {
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

#[rstest]
#[tokio::test]
async fn group_simulate_matches_send() -> TestResult {
    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();
    let app_id = deploy_arc56_contract(&fixture, &sender, &get_sandbox_spec(), None, None).await?;

    let client = AppClient::new(AppClientParams {
        app_id: Some(app_id),
        app_spec: get_testing_app_spec(),
        algorand: RootAlgorandClient::default_localnet(),
        app_name: None,
        default_sender: Some(sender.to_string()),
        source_maps: None,
    });

    // Compose group: set_global + payment + call_abi
    let mut composer = fixture.algorand_client.new_group();

    // 1) add(uint64,uint64)uint64
    let method_set_global = algokit_abi::ABIMethod::from_str("add(uint64,uint64)uint64").unwrap();
    let set_params = algokit_utils::AppCallMethodCallParams {
        common_params: algokit_utils::CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        app_id,
        method: method_set_global,
        args: vec![
            algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(1u64)),
            algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from(2u64)),
        ],
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: algokit_transact::OnApplicationComplete::NoOp,
    };
    composer.add_app_call_method_call(set_params)?;

    // 2) payment
    let payment = algokit_utils::PaymentParams {
        common_params: algokit_utils::CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        receiver: sender.clone(),
        amount: 10_000,
    };
    composer.add_payment(payment)?;

    // 3) hello_world(string)string
    let method_call_abi = algokit_abi::ABIMethod::from_str("hello_world(string)string").unwrap();
    let call_params = algokit_utils::AppCallMethodCallParams {
        common_params: algokit_utils::CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        app_id,
        method: method_call_abi,
        args: vec![algokit_utils::AppMethodCallArg::ABIValue(
            algokit_abi::ABIValue::from("test"),
        )],
        account_references: None,
        app_references: None,
        asset_references: None,
        box_references: None,
        on_complete: algokit_transact::OnApplicationComplete::NoOp,
    };
    composer.add_app_call_method_call(call_params)?;

    let simulate = composer
        .simulate(Some(SimulateParams {
            skip_signatures: true,
            ..Default::default()
        }))
        .await?;
    let send = composer.send(None).await?;

    assert_eq!(simulate.transactions.len(), send.transaction_ids.len());
    let last_idx = send.abi_returns.len().saturating_sub(1);
    if !simulate.returns.is_empty() && send.abi_returns.get(last_idx).is_some() {
        let sim_ret = simulate.returns.last().unwrap();
        if let Some(Ok(Some(send_ret))) = send.abi_returns.get(last_idx).map(|r| r.as_ref()) {
            assert_eq!(sim_ret.return_value, send_ret.return_value);
        }
    }
    Ok(())
}

#[rstest]
#[tokio::test]
async fn construct_transaction_with_abi_encoding_including_transaction() -> TestResult {
    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();
    // Use sandbox which has get_pay_txn_amount(pay)uint64
    let spec = get_sandbox_spec();
    let app_id = deploy_arc56_contract(&fixture, &sender, &spec, None, None).await?;

    let mut algorand = RootAlgorandClient::default_localnet();
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let client = AppClient::new(AppClientParams {
        app_id: Some(app_id),
        app_spec: spec,
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        source_maps: None,
    });

    // Prepare a payment as an ABI transaction argument
    let payment = algokit_utils::PaymentParams {
        common_params: algokit_utils::CommonParams {
            sender: sender.clone(),
            ..Default::default()
        },
        receiver: sender.clone(),
        amount: 12345,
    };

    let send_res = client
        .send()
        .call(
            algokit_utils::applications::app_client::AppClientMethodCallParams {
                method: "get_pay_txn_amount(pay)uint64".to_string(),
                args: Some(vec![algokit_utils::AppMethodCallArg::Payment(payment)]),
                sender: Some(sender.to_string()),
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: None,
                on_complete: None,
            },
        )
        .await?;

    // Expect a group of 2 transactions: payment + app call
    assert_eq!(send_res.common_params.transactions.len(), 2);
    // ABI return should be present and decode to expected value
    let abi_ret = send_res.abi_return.as_ref().expect("abi return expected");
    let ret_val = match &abi_ret.return_value {
        algokit_abi::ABIValue::Uint(u) => u.clone(),
        _ => panic!("expected uint64 return"),
    };
    assert_eq!(ret_val, num_bigint::BigUint::from(12345u32));
    Ok(())
}

#[rstest]
#[tokio::test]
async fn box_methods_with_arc4_returns_parametrized() -> TestResult {
    use algokit_utils::applications::app_client::AppClientMethodCallParams as MCP;

    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();

    // Deploy testing_app_puya
    let spec =
        Arc56Contract::from_json(algokit_test_artifacts::testing_app_puya::APPLICATION_ARC56)
            .expect("valid arc56");
    let app_id = deploy_arc56_contract(&fixture, &sender, &spec, None, None).await?;

    let mut algorand = RootAlgorandClient::default_localnet();
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let client = AppClient::new(AppClientParams {
        app_id: Some(app_id),
        app_spec: spec,
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        source_maps: None,
    });

    // Fund app account to allow box writes
    client
        .fund_app_account(
            algokit_utils::applications::app_client::FundAppAccountParams {
                amount: 1_000_000,
                sender: Some(sender.to_string()),
                ..Default::default()
            },
        )
        .await?;

    // Parametrized ARC-4 return cases
    let mut big = num_bigint::BigUint::from(1u64);
    big <<= 256u32;
    let cases: Vec<(Vec<u8>, &str, &str, algokit_abi::ABIValue)> = vec![
        (
            b"box_str".to_vec(),
            "set_box_str(string,string)void",
            "string",
            algokit_abi::ABIValue::from("string"),
        ),
        (
            b"box_int".to_vec(),
            "set_box_int(string,uint32)void",
            "uint32",
            algokit_abi::ABIValue::from(123u32),
        ),
        (
            b"box_int512".to_vec(),
            "set_box_int512(string,uint512)void",
            "uint512",
            algokit_abi::ABIValue::from(big),
        ),
        (
            b"box_static".to_vec(),
            "set_box_static(string,byte[4])void",
            "byte[4]",
            algokit_abi::ABIValue::Array(vec![
                algokit_abi::ABIValue::from_byte(1),
                algokit_abi::ABIValue::from_byte(2),
                algokit_abi::ABIValue::from_byte(3),
                algokit_abi::ABIValue::from_byte(4),
            ]),
        ),
        (
            b"".to_vec(),
            "set_struct",
            "(string,uint64)",
            algokit_abi::ABIValue::Array(vec![
                algokit_abi::ABIValue::from("box1"),
                algokit_abi::ABIValue::from(123u64),
            ]),
        ),
    ];

    for (box_prefix, method_sig, value_type_str, arg_val) in cases {
        // Encode the box name using ABIType "string"
        let name_type = algokit_abi::ABIType::from_str("string").unwrap();
        let name_encoded = name_type
            .encode(&algokit_abi::ABIValue::from("box1"))
            .unwrap();
        let mut box_reference = box_prefix.clone();
        box_reference.extend_from_slice(&name_encoded);

        // Send method call
        client
            .send()
            .call(MCP {
                method: method_sig.to_string(),
                args: Some(vec![
                    algokit_utils::AppMethodCallArg::ABIValue(algokit_abi::ABIValue::from("box1")),
                    algokit_utils::AppMethodCallArg::ABIValue(arg_val.clone()),
                ]),
                sender: Some(sender.to_string()),
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: Some(vec![BoxReference {
                    app_id: 0,
                    name: box_reference.clone(),
                }]),
                on_complete: None,
            })
            .await?;

        // Verify raw equals ABI-encoded expected
        let expected_raw = algokit_abi::ABIType::from_str(value_type_str)
            .unwrap()
            .encode(&arg_val)
            .unwrap();
        let actual_raw = client.get_box_value(&box_reference).await?;
        assert_eq!(actual_raw, expected_raw);

        // Decode via ABI type and verify
        let decoded = client
            .get_box_value_from_abi_type(
                &box_reference,
                &algokit_abi::ABIType::from_str(value_type_str).unwrap(),
            )
            .await?;
        assert_eq!(decoded, arg_val);

        // For struct case, also verify bulk fetch decode path matches
        if method_sig == "set_struct" {
            let values = client
                .get_box_values_from_abi_type(
                    &[box_reference.clone()],
                    &algokit_abi::ABIType::from_str(value_type_str).unwrap(),
                )
                .await?;
            assert_eq!(values.len(), 1);
            assert_eq!(values[0], decoded);
        }
    }

    Ok(())
}

#[rstest]
#[tokio::test]
async fn app_client_from_network_resolves_id() -> TestResult {
    // Deploy hello_world and write networks mapping into spec, then call from_network
    let fixture = crate::common::algorand_fixture().await?;
    let sender = fixture.test_account.account().address();

    let spec = Arc56Contract::from_json(algokit_test_artifacts::hello_world::APPLICATION_ARC56)
        .expect("valid arc56");
    let app_id = deploy_arc56_contract(&fixture, &sender, &spec, None, None).await?;

    let mut spec_with_networks = spec.clone();
    spec_with_networks.networks = Some(std::collections::HashMap::from([(
        "localnet".to_string(),
        algokit_abi::arc56_contract::Network { app_id },
    )]));

    let client = AppClient::from_network(
        spec_with_networks,
        RootAlgorandClient::default_localnet(),
        None,
        None,
        None,
    )
    .await
    .expect("from_network");
    assert_eq!(client.app_id(), Some(app_id));
    Ok(())
}
