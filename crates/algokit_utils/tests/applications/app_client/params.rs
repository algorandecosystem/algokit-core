use crate::common::{AlgorandFixtureResult, TestResult, algorand_fixture, deploy_arc56_contract};
use algokit_abi::{ABIValue, Arc56Contract};
use algokit_transact::BoxReference;
use algokit_utils::applications::app_client::{
    AppClient, AppClientBareCallParams, AppClientParams,
};
use algokit_utils::applications::app_client::{AppClientMethodCallParams, FundAppAccountParams};
use algokit_utils::clients::app_manager::{TealTemplateParams, TealTemplateValue};
use algokit_utils::{AlgorandClient as RootAlgorandClient, AppMethodCallArg, PaymentParams};
use rstest::*;
use std::sync::Arc;

fn get_testing_app_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::testing_app::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

fn get_sandbox_spec() -> Arc56Contract {
    let json = algokit_test_artifacts::sandbox::APPLICATION_ARC56;
    Arc56Contract::from_json(json).expect("valid arc56")
}

#[rstest]
#[tokio::test]
async fn params_build_method_call_and_defaults(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &get_testing_app_spec(),
        Some(
            [("VALUE", 0), ("UPDATABLE", 0), ("DELETABLE", 0)]
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
        algorand,
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
                method: "set_global".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from(999u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from(2u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from("seed")),
                    AppMethodCallArg::ABIValue(ABIValue::Array(vec![
                        ABIValue::from_byte(1),
                        ABIValue::from_byte(2),
                        ABIValue::from_byte(3),
                        ABIValue::from_byte(4),
                    ])),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    client
        .send()
        .opt_in(
            AppClientMethodCallParams {
                method: "opt_in".to_string(),
                args: vec![],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;
    client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "set_local".to_string(),
                args: vec![
                    AppMethodCallArg::ABIValue(ABIValue::from(1u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from(2u64)),
                    AppMethodCallArg::ABIValue(ABIValue::from("bananas")),
                    AppMethodCallArg::ABIValue(ABIValue::Array(vec![
                        ABIValue::from_byte(1),
                        ABIValue::from_byte(2),
                        ABIValue::from_byte(3),
                        ABIValue::from_byte(4),
                    ])),
                ],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    let built = client
        .params()
        .call(
            AppClientMethodCallParams {
                method: "default_value_from_local_state".to_string(),
                args: vec![AppMethodCallArg::DefaultValue],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;

    assert_eq!(built.method.name, "default_value_from_local_state");
    assert_eq!(built.args.len(), 1);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn params_build_includes_foreign_references_from_args(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &get_testing_app_spec(),
        Some(
            [("VALUE", 0), ("UPDATABLE", 0), ("DELETABLE", 0)]
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
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        source_maps: None,
        transaction_composer_config: None,
    });

    let mut fixture = fixture;
    let extra = fixture.generate_account(None).await?;
    let extra_addr = extra.account().address().to_string();

    let built = client
        .params()
        .call(
            AppClientMethodCallParams {
                method: "call_abi_foreign_refs".to_string(),
                args: vec![],
                sender: Some(sender.to_string()),
                account_references: Some(vec![extra_addr.clone()]),
                app_references: Some(vec![345]),
                asset_references: Some(vec![567]),
                ..Default::default()
            },
            None,
        )
        .await?;

    assert!(built.account_references.as_ref().unwrap().len() >= 1);
    assert!(built.app_references.as_ref().unwrap().contains(&345));
    assert!(built.asset_references.as_ref().unwrap().contains(&567));

    Ok(())
}

#[rstest]
#[tokio::test]
async fn params_build_bare_and_fund_payment(
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

    let bare = client.params().bare().call(
        AppClientBareCallParams {
            args: None,
            sender: Some(sender.to_string()),
            box_references: Some(vec![BoxReference {
                app_id: 0,
                name: b"1".to_vec(),
            }]),
            ..Default::default()
        },
        None,
    )?;
    assert_eq!(bare.box_references.as_ref().unwrap()[0].name, b"1".to_vec());

    let pay: PaymentParams = client.params().fund_app_account(&FundAppAccountParams {
        amount: 200_000,
        sender: Some(sender.to_string()),
        ..Default::default()
    })?;
    assert_eq!(pay.amount, 200_000);
    assert_eq!(pay.receiver, client.app_address());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn params_construct_txn_with_abi_tx_arg_and_return(
    #[future] algorand_fixture: AlgorandFixtureResult,
) -> TestResult {
    let fixture = algorand_fixture.await?;
    let sender = fixture.test_account.account().address();

    let spec = get_sandbox_spec();
    let app_id = deploy_arc56_contract(&fixture, &sender, &spec, None, None, None).await?;

    let mut algorand = RootAlgorandClient::default_localnet(None);
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let client = AppClient::new(AppClientParams {
        app_id,
        app_spec: spec,
        algorand,
        app_name: None,
        default_sender: Some(sender.to_string()),
        default_signer: None,
        source_maps: None,
        transaction_composer_config: None,
    });

    let payment = PaymentParams {
        sender: sender.clone(),
        signer: None,
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
        amount: 123,
    };

    let result = client
        .send()
        .call(
            AppClientMethodCallParams {
                method: "get_pay_txn_amount".to_string(),
                args: vec![AppMethodCallArg::Payment(payment)],
                sender: Some(sender.to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

    assert_eq!(result.common_params.transactions.len(), 2);
    let abi_ret = result.abi_return.as_ref().expect("abi return expected");
    match &abi_ret.return_value {
        Some(ABIValue::Uint(u)) => assert_eq!(*u, num_bigint::BigUint::from(123u32)),
        _ => return Err("expected uint return".into()),
    }
    Ok(())
}
