use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use algokit_abi::arc56_contract::CallOnApplicationComplete;
use algokit_abi::{ABIReturn, ABIValue, Arc56Contract};

use crate::applications::app_client::{AppClientMethodCallParams, CompilationParams};
use crate::applications::app_deployer::{AppLookup, OnSchemaBreak, OnUpdate};
use crate::applications::app_factory;
use crate::clients::app_manager::{
    DELETABLE_TEMPLATE_NAME, TealTemplateValue, UPDATABLE_TEMPLATE_NAME,
};
use crate::transactions::{
    TransactionComposerConfig, TransactionResultError, TransactionSigner,
    composer::{SendParams as ComposerSendParams, SendTransactionComposerResults},
    sender_results::{
        SendAppCallResult, SendAppCreateResult, SendAppUpdateResult, SendTransactionResult,
    },
};
use crate::{AlgorandClient, AppClient, AppClientParams, AppSourceMaps};
use app_factory::types as aftypes;

mod compilation;
mod error;
mod params_builder;
mod sender;
mod transaction_builder;
mod types;
mod utils;

pub use error::AppFactoryError;
pub use params_builder::ParamsBuilder;
pub use sender::TransactionSender;
pub use transaction_builder::TransactionBuilder;
pub use types::*;

/// Factory for creating and deploying Algorand applications from an ARC-56 spec.
pub struct AppFactory {
    app_spec: Arc56Contract,
    algorand: Arc<AlgorandClient>,
    app_name: String,
    version: String,
    default_sender: Option<String>,
    default_signer: Option<Arc<dyn TransactionSigner>>,
    approval_source_map: Mutex<Option<serde_json::Value>>,
    clear_source_map: Mutex<Option<serde_json::Value>>,
    pub(crate) deploy_time_params: Option<HashMap<String, TealTemplateValue>>,
    pub(crate) updatable: Option<bool>,
    pub(crate) deletable: Option<bool>,
    pub(crate) transaction_composer_config: Option<TransactionComposerConfig>,
}

#[derive(Default)]
pub struct DeployArgs {
    pub on_update: Option<OnUpdate>,
    pub on_schema_break: Option<OnSchemaBreak>,
    pub create_params: Option<aftypes::AppFactoryCreateMethodCallParams>,
    pub update_params: Option<AppClientMethodCallParams>,
    pub delete_params: Option<AppClientMethodCallParams>,
    pub existing_deployments: Option<AppLookup>,
    pub ignore_cache: Option<bool>,
    pub app_name: Option<String>,
    pub send_params: Option<ComposerSendParams>,
}

impl AppFactory {
    async fn deploy_create_result(
        &self,
        composer_result: &SendTransactionComposerResults,
    ) -> Result<AppFactoryCreateMethodCallResult, AppFactoryError> {
        let compiled = self.compile_programs_with(None).await?;
        let base = self.to_send_transaction_result(composer_result)?;
        let last_abi_return = base.abi_returns.as_ref().and_then(|v| v.last()).cloned();
        let created = SendAppCreateResult::new(
            base,
            last_abi_return.clone(),
            Some(compiled.approval.compiled_base64_to_bytes.clone()),
            Some(compiled.clear.compiled_base64_to_bytes.clone()),
            compiled.approval.source_map.clone(),
            compiled.clear.source_map.clone(),
        )
        .map_err(|e| AppFactoryError::ValidationError {
            message: e.to_string(),
        })?;
        let arc56_return = self.parse_method_return_value(&last_abi_return)?;
        Ok(AppFactoryMethodCallResult::new(created, arc56_return))
    }

    async fn deploy_update_result(
        &self,
        composer_result: &SendTransactionComposerResults,
    ) -> Result<AppFactoryUpdateMethodCallResult, AppFactoryError> {
        let compiled = self.compile_programs_with(None).await?;
        let base = self.to_send_transaction_result(composer_result)?;
        let last_abi_return = base.abi_returns.as_ref().and_then(|v| v.last()).cloned();
        let updated = SendAppUpdateResult::new(
            base,
            last_abi_return.clone(),
            Some(compiled.approval.compiled_base64_to_bytes.clone()),
            Some(compiled.clear.compiled_base64_to_bytes.clone()),
            compiled.approval.source_map.clone(),
            compiled.clear.source_map.clone(),
        );
        let arc56_return = self.parse_method_return_value(&last_abi_return)?;
        Ok(AppFactoryMethodCallResult::new(updated, arc56_return))
    }

    async fn deploy_replace_results(
        &self,
        composer_result: &SendTransactionComposerResults,
    ) -> Result<
        (
            Option<AppFactoryCreateMethodCallResult>,
            Option<AppFactoryDeleteMethodCallResult>,
        ),
        AppFactoryError,
    > {
        if composer_result.confirmations.is_empty()
            || composer_result.confirmations.len() != composer_result.transaction_ids.len()
        {
            return Ok((None, None));
        }
        let compiled = self.compile_programs_with(None).await?;
        // Create index 0
        let create_tx = composer_result.confirmations[0].txn.transaction.clone();
        let create_base = SendTransactionResult::new(
            composer_result.group.map(hex::encode).unwrap_or_default(),
            vec![composer_result.transaction_ids[0].clone()],
            vec![create_tx],
            vec![composer_result.confirmations[0].clone()],
            if !composer_result.abi_returns.is_empty() {
                Some(vec![composer_result.abi_returns[0].clone()])
            } else {
                None
            },
        )
        .map_err(|e| AppFactoryError::ValidationError {
            message: e.to_string(),
        })?;
        let create_abi = create_base
            .abi_returns
            .as_ref()
            .and_then(|v| v.last())
            .cloned();
        let created = SendAppCreateResult::new(
            create_base,
            create_abi.clone(),
            Some(compiled.approval.compiled_base64_to_bytes.clone()),
            Some(compiled.clear.compiled_base64_to_bytes.clone()),
            compiled.approval.source_map.clone(),
            compiled.clear.source_map.clone(),
        )
        .map_err(|e| AppFactoryError::ValidationError {
            message: e.to_string(),
        })?;
        let create_arc56 = self.parse_method_return_value(&create_abi)?;
        let create_result = Some(AppFactoryMethodCallResult::new(created, create_arc56));
        // Optional delete uses the final transaction in the replacement group
        let delete_result = if composer_result.confirmations.len() > 1 {
            let delete_index = composer_result.confirmations.len() - 1;
            let delete_tx = composer_result.confirmations[delete_index]
                .txn
                .transaction
                .clone();
            let delete_base = SendTransactionResult::new(
                composer_result.group.map(hex::encode).unwrap_or_default(),
                vec![composer_result.transaction_ids[delete_index].clone()],
                vec![delete_tx],
                vec![composer_result.confirmations[delete_index].clone()],
                if composer_result.abi_returns.len() > delete_index {
                    Some(vec![composer_result.abi_returns[delete_index].clone()])
                } else {
                    None
                },
            )
            .map_err(|e| AppFactoryError::ValidationError {
                message: e.to_string(),
            })?;
            let delete_abi = delete_base
                .abi_returns
                .as_ref()
                .and_then(|v| v.last())
                .cloned();
            let deleted = SendAppCallResult::new(delete_base, delete_abi.clone());
            let delete_arc56 = self.parse_method_return_value(&delete_abi)?;
            Some(AppFactoryMethodCallResult::new(deleted, delete_arc56))
        } else {
            None
        };
        Ok((create_result, delete_result))
    }
    /// Convert SendTransactionComposerResults into a rich SendTransactionResult by
    /// reconstructing transactions from confirmations.
    fn to_send_transaction_result(
        &self,
        composer_results: &SendTransactionComposerResults,
    ) -> Result<SendTransactionResult, AppFactoryError> {
        let group_id = composer_results.group.map(hex::encode).unwrap_or_default();

        // Reconstruct transactions from confirmations (txn.signed.transaction)
        let transactions: Vec<algokit_transact::Transaction> = composer_results
            .confirmations
            .iter()
            .map(|c| c.txn.transaction.clone())
            .collect();

        SendTransactionResult::new(
            group_id,
            composer_results.transaction_ids.clone(),
            transactions,
            composer_results.confirmations.clone(),
            if composer_results.abi_returns.is_empty() {
                None
            } else {
                Some(composer_results.abi_returns.clone())
            },
        )
        .map_err(|e| AppFactoryError::ValidationError {
            message: e.to_string(),
        })
    }
    pub fn new(params: AppFactoryParams) -> Self {
        let AppFactoryParams {
            algorand,
            app_spec,
            app_name,
            default_sender,
            default_signer,
            version,
            deploy_time_params,
            updatable,
            deletable,
            source_maps,
            transaction_composer_config,
        } = params;

        let (initial_approval_source_map, initial_clear_source_map) = match source_maps {
            Some(maps) => (maps.approval_source_map, maps.clear_source_map),
            None => (None, None),
        };

        Self {
            app_spec,
            algorand,
            app_name: app_name.unwrap_or_else(|| "<unnamed>".to_string()),
            version: version.unwrap_or_else(|| "1.0".to_string()),
            default_sender,
            default_signer,
            approval_source_map: Mutex::new(initial_approval_source_map),
            clear_source_map: Mutex::new(initial_clear_source_map),
            deploy_time_params,
            updatable,
            deletable,
            transaction_composer_config,
        }
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }
    pub fn app_spec(&self) -> &Arc56Contract {
        &self.app_spec
    }

    pub fn algorand(&self) -> Arc<AlgorandClient> {
        self.algorand.clone()
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn params(&self) -> ParamsBuilder<'_> {
        ParamsBuilder { factory: self }
    }
    pub fn create_transaction(&self) -> TransactionBuilder<'_> {
        TransactionBuilder { factory: self }
    }
    pub fn send(&self) -> TransactionSender<'_> {
        TransactionSender { factory: self }
    }

    pub fn import_source_maps(&self, source_maps: AppSourceMaps) {
        *self.approval_source_map.lock().unwrap() = source_maps.approval_source_map;
        *self.clear_source_map.lock().unwrap() = source_maps.clear_source_map;
    }

    pub fn export_source_maps(&self) -> Result<AppSourceMaps, AppFactoryError> {
        let approval = self
            .approval_source_map
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| AppFactoryError::ValidationError {
                message: "Approval source map not loaded".to_string(),
            })?;
        let clear = self
            .clear_source_map
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| AppFactoryError::ValidationError {
                message: "Clear source map not loaded".to_string(),
            })?;
        Ok(AppSourceMaps {
            approval_source_map: Some(approval),
            clear_source_map: Some(clear),
        })
    }

    pub async fn compile(
        &self,
        compilation_params: Option<CompilationParams>,
    ) -> Result<AppFactoryCompilationResult, AppFactoryError> {
        let compiled = self.compile_programs_with(compilation_params).await?;
        Ok(AppFactoryCompilationResult {
            approval_program: compiled.approval.compiled_base64_to_bytes.clone(),
            clear_state_program: compiled.clear.compiled_base64_to_bytes.clone(),
            compiled_approval: compiled.approval,
            compiled_clear: compiled.clear,
        })
    }

    pub fn get_app_client_by_id(
        &self,
        app_id: u64,
        app_name: Option<String>,
        default_sender: Option<String>,
        default_signer: Option<Arc<dyn TransactionSigner>>,
        source_maps: Option<AppSourceMaps>,
    ) -> AppClient {
        let resolved_source_maps = source_maps.or_else(|| self.current_source_maps());
        AppClient::new(AppClientParams {
            app_id,
            app_spec: self.app_spec.clone(),
            algorand: self.algorand.clone(),
            app_name: Some(app_name.unwrap_or_else(|| self.app_name.clone())),
            default_sender: Some(
                default_sender.unwrap_or_else(|| self.default_sender.clone().unwrap_or_default()),
            ),
            default_signer: default_signer.or_else(|| self.default_signer.clone()),
            source_maps: resolved_source_maps,
            transaction_composer_config: self.transaction_composer_config.clone(),
        })
    }

    pub async fn get_app_client_by_creator_and_name(
        &self,
        creator_address: &str,
        app_name: Option<String>,
        default_sender: Option<String>,
        default_signer: Option<Arc<dyn TransactionSigner>>,
        ignore_cache: Option<bool>,
    ) -> Result<AppClient, AppFactoryError> {
        let resolved_app_name = app_name.unwrap_or_else(|| self.app_name.clone());
        let resolved_sender = default_sender.or_else(|| self.default_sender.clone());
        let resolved_signer = default_signer.or_else(|| self.default_signer.clone());

        let client = AppClient::from_creator_and_name(
            creator_address,
            &resolved_app_name,
            self.app_spec.clone(),
            self.algorand.clone(),
            resolved_sender,
            resolved_signer,
            self.current_source_maps(),
            ignore_cache,
            self.transaction_composer_config.clone(),
        )
        .await
        .map_err(|e| AppFactoryError::AppClientError { source: e })?;

        Ok(client)
    }
}

impl AppFactory {
    pub(crate) fn get_sender_address(
        &self,
        sender: &Option<String>,
    ) -> Result<algokit_transact::Address, String> {
        let sender_str = sender
            .as_ref()
            .or(self.default_sender.as_ref())
            .ok_or_else(|| {
                format!(
                    "No sender provided and no default sender configured for app {}",
                    self.app_name
                )
            })?;
        algokit_transact::Address::from_str(sender_str)
            .map_err(|e| format!("Invalid sender address: {}", e))
    }

    pub(crate) fn update_source_maps(
        &self,
        approval: Option<serde_json::Value>,
        clear: Option<serde_json::Value>,
    ) {
        *self.approval_source_map.lock().unwrap() = approval;
        *self.clear_source_map.lock().unwrap() = clear;
    }

    pub(crate) fn current_source_maps(&self) -> Option<AppSourceMaps> {
        let approval = self.approval_source_map.lock().unwrap().clone();
        let clear = self.clear_source_map.lock().unwrap().clone();

        if approval.is_none() && clear.is_none() {
            None
        } else {
            Some(AppSourceMaps {
                approval_source_map: approval,
                clear_source_map: clear,
            })
        }
    }

    pub(crate) fn parse_method_return_value(
        &self,
        abi_return: &Option<ABIReturn>,
    ) -> Result<Option<ABIValue>, AppFactoryError> {
        match abi_return {
            None => Ok(None),
            Some(ret) => {
                if let Some(err) = &ret.decode_error {
                    return Err(AppFactoryError::ValidationError {
                        message: err.to_string(),
                    });
                }
                Ok(ret.return_value.clone())
            }
        }
    }

    pub(crate) fn detect_deploy_time_control_flag(
        &self,
        template_name: &str,
        on_complete: CallOnApplicationComplete,
    ) -> Option<bool> {
        let source = self.app_spec().source.as_ref()?;
        let approval = source.get_decoded_approval().ok()?;
        if !approval.contains(template_name) {
            return None;
        }

        let bare_allows = self
            .app_spec()
            .bare_actions
            .call
            .iter()
            .any(|action| *action == on_complete);
        let method_allows = self.app_spec().methods.iter().any(|method| {
            method
                .actions
                .call
                .iter()
                .any(|action| *action == on_complete)
        });

        Some(bare_allows || method_allows)
    }

    pub(crate) fn logic_error_for(
        &self,
        error_str: &str,
        is_clear_state_program: bool,
    ) -> Option<String> {
        if !(error_str.contains("logic eval error") || error_str.contains("logic error")) {
            return None;
        }

        let tx_err = TransactionResultError::ParsingError {
            message: error_str.to_string(),
        };

        let client = AppClient::new(AppClientParams {
            app_id: 0,
            app_spec: self.app_spec.clone(),
            algorand: self.algorand.clone(),
            app_name: Some(self.app_name.clone()),
            default_sender: self.default_sender.clone(),
            default_signer: self.default_signer.clone(),
            source_maps: self.current_source_maps(),
            transaction_composer_config: self.transaction_composer_config.clone(),
        });

        Some(
            client
                .expose_logic_error(&tx_err, is_clear_state_program)
                .message,
        )
    }

    /// Idempotently deploy (create/update/delete) an application using AppDeployer
    pub async fn deploy(
        &self,
        args: DeployArgs,
    ) -> Result<(AppClient, aftypes::AppFactoryDeployResult), AppFactoryError> {
        // Prepare create/update/delete deploy params
        // Auto-detect deploy-time controls if not explicitly provided
        let mut resolved_updatable = self.updatable;
        let mut resolved_deletable = self.deletable;
        if resolved_updatable.is_none() {
            resolved_updatable = self.detect_deploy_time_control_flag(
                UPDATABLE_TEMPLATE_NAME,
                CallOnApplicationComplete::UpdateApplication,
            );
        }

        if resolved_deletable.is_none() {
            resolved_deletable = self.detect_deploy_time_control_flag(
                DELETABLE_TEMPLATE_NAME,
                CallOnApplicationComplete::DeleteApplication,
            );
        }
        let resolved_deploy_time_params = self.deploy_time_params.clone();

        let create_deploy_params = match args.create_params {
            Some(cp) => crate::applications::app_deployer::CreateParams::AppCreateMethodCall(
                self.params().create(cp)?,
            ),
            None => crate::applications::app_deployer::CreateParams::AppCreateCall(
                self.params().bare().create(None)?,
            ),
        };

        let update_deploy_params = match args.update_params {
            Some(up) => crate::applications::app_deployer::UpdateParams::AppUpdateMethodCall(
                self.params().deploy_update(up)?,
            ),
            None => crate::applications::app_deployer::UpdateParams::AppUpdateCall(
                self.params().bare().deploy_update(None)?,
            ),
        };

        let delete_deploy_params = match args.delete_params {
            Some(dp) => crate::applications::app_deployer::DeleteParams::AppDeleteMethodCall(
                self.params().deploy_delete(dp)?,
            ),
            None => crate::applications::app_deployer::DeleteParams::AppDeleteCall(
                self.params().bare().deploy_delete(None)?,
            ),
        };

        let metadata = crate::applications::app_deployer::AppDeployMetadata {
            name: args.app_name.unwrap_or_else(|| self.app_name.clone()),
            version: self.version.clone(),
            updatable: resolved_updatable,
            deletable: resolved_deletable,
        };

        let deploy_params = crate::applications::app_deployer::AppDeployParams {
            metadata,
            deploy_time_params: resolved_deploy_time_params,
            on_schema_break: args.on_schema_break,
            on_update: args.on_update,
            create_params: create_deploy_params,
            update_params: update_deploy_params,
            delete_params: delete_deploy_params,
            existing_deployments: args.existing_deployments,
            ignore_cache: args.ignore_cache,
            send_params: args.send_params.unwrap_or_default(),
        };

        let mut app_deployer = self.algorand.as_ref().app_deployer();

        let deploy_result = app_deployer
            .deploy(deploy_params)
            .await
            .map_err(|e| AppFactoryError::AppDeployerError { source: e })?;

        // Build AppClient for the resulting app
        let app_metadata = match &deploy_result {
            crate::applications::app_deployer::AppDeployResult::Create { app, .. }
            | crate::applications::app_deployer::AppDeployResult::Update { app, .. }
            | crate::applications::app_deployer::AppDeployResult::Replace { app, .. }
            | crate::applications::app_deployer::AppDeployResult::Nothing { app } => app,
        };

        // Create AppClient with shared signers from the factory's AlgorandClient
        let app_client = AppClient::new(AppClientParams {
            app_id: app_metadata.app_id,
            app_spec: self.app_spec.clone(),
            algorand: self.algorand.clone(),
            app_name: Some(self.app_name.clone()),
            default_sender: self.default_sender.clone(),
            default_signer: self.default_signer.clone(),
            source_maps: self.current_source_maps(),
            transaction_composer_config: self.transaction_composer_config.clone(),
        });

        // Convert deploy result into factory result with enriched typed results
        let mut create_result: Option<aftypes::AppFactoryCreateMethodCallResult> = None;
        let mut update_result: Option<aftypes::AppFactoryUpdateMethodCallResult> = None;
        let mut delete_result: Option<aftypes::AppFactoryDeleteMethodCallResult> = None;

        match &deploy_result {
            crate::applications::app_deployer::AppDeployResult::Create { result, .. } => {
                create_result = Some(self.deploy_create_result(result).await?);
            }
            crate::applications::app_deployer::AppDeployResult::Update { result, .. } => {
                update_result = Some(self.deploy_update_result(result).await?);
            }
            crate::applications::app_deployer::AppDeployResult::Replace { result, .. } => {
                let (c, d) = self.deploy_replace_results(result).await?;
                create_result = c;
                delete_result = d;
            }
            crate::applications::app_deployer::AppDeployResult::Nothing { .. } => {}
        }

        let factory_result = aftypes::AppFactoryDeployResult {
            app: app_metadata.clone(),
            operation_performed: deploy_result,
            create_result,
            update_result,
            delete_result,
        };

        Ok((app_client, factory_result))
    }
}
