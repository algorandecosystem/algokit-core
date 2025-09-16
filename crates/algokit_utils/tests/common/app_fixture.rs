use crate::common::{
    AlgorandFixture, AlgorandFixtureResult, algorand_fixture, deploy_arc56_contract,
};
use algokit_abi::Arc56Contract;
use algokit_transact::Address;
use algokit_utils::AlgorandClient;
use algokit_utils::applications::app_client::{AppClient, AppClientParams};
use algokit_utils::clients::app_manager::{
    DeploymentMetadata, TealTemplateParams, TealTemplateValue,
};
use algokit_utils::transactions::TransactionComposerConfig;
use rstest::fixture;
use std::sync::Arc;

pub struct AppFixture {
    pub algorand_fixture: AlgorandFixture,
    pub sender_address: Address,
    pub app_id: u64,
    pub app_spec: Arc56Contract,
    pub client: AppClient,
}

pub type AppFixtureResult = Result<AppFixture, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Default)]
pub struct AppFixtureOptions {
    pub template_params: Option<TealTemplateParams>,
    pub deploy_metadata: Option<DeploymentMetadata>,
    pub args: Option<Vec<Vec<u8>>>,
    pub transaction_composer_config: Option<TransactionComposerConfig>,
    pub default_sender_override: Option<String>,
}

pub async fn build_app_fixture(
    fixture: AlgorandFixture,
    spec: Arc56Contract,
    opts: AppFixtureOptions,
) -> AppFixtureResult {
    let sender = fixture.test_account.account().address();

    let app_id = deploy_arc56_contract(
        &fixture,
        &sender,
        &spec,
        opts.template_params.clone(),
        opts.deploy_metadata.clone(),
        opts.args.clone(),
    )
    .await?;

    let mut algorand = AlgorandClient::default_localnet(None);
    algorand.set_signer(sender.clone(), Arc::new(fixture.test_account.clone()));
    let client = AppClient::new(AppClientParams {
        app_id,
        app_spec: spec.clone(),
        algorand,
        app_name: None,
        default_sender: Some(
            opts.default_sender_override
                .unwrap_or_else(|| sender.to_string()),
        ),
        default_signer: None,
        source_maps: None,
        transaction_composer_config: opts.transaction_composer_config,
    });

    Ok(AppFixture {
        algorand_fixture: fixture,
        sender_address: sender,
        app_id,
        app_spec: spec,
        client,
    })
}

// Intentionally no generic rstest fixture here; prefer build_app_fixture and spec-specific wrappers.

pub fn default_teal_params(value: u64, updatable: bool, deletable: bool) -> TealTemplateParams {
    let mut t = TealTemplateParams::default();
    t.insert("VALUE".to_string(), TealTemplateValue::Int(value));
    t.insert(
        "UPDATABLE".to_string(),
        TealTemplateValue::Int(if updatable { 1 } else { 0 }),
    );
    t.insert(
        "DELETABLE".to_string(),
        TealTemplateValue::Int(if deletable { 1 } else { 0 }),
    );
    t
}
