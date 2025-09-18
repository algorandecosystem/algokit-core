use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use algokit_abi::Arc56Contract;

use crate::applications::app_client::AppClientMethodCallParams;
use crate::clients::app_manager::TealTemplateValue;
use crate::transactions::{TransactionComposerConfig, TransactionSigner};
use crate::{AlgorandClient, AppClient, AppClientParams, AppSourceMaps};

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

impl AppFactory {
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

    pub fn import_source_maps(&self, source_maps: crate::AppSourceMaps) {
        *self.approval_source_map.lock().unwrap() = source_maps.approval_source_map;
        *self.clear_source_map.lock().unwrap() = source_maps.clear_source_map;
    }

    pub fn export_source_maps(&self) -> Result<crate::AppSourceMaps, AppFactoryError> {
        let approval = self
            .approval_source_map
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| {
                AppFactoryError::ValidationError("Approval source map not loaded".to_string())
            })?;
        let clear = self
            .clear_source_map
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| {
                AppFactoryError::ValidationError("Clear source map not loaded".to_string())
            })?;
        Ok(crate::AppSourceMaps {
            approval_source_map: Some(approval),
            clear_source_map: Some(clear),
        })
    }

    pub fn params_accessor(&self) -> ParamsBuilder<'_> {
        self.params()
    }

    pub fn send_accessor(&self) -> TransactionSender<'_> {
        self.send()
    }

    pub fn create_transaction_accessor(&self) -> TransactionBuilder<'_> {
        self.create_transaction()
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
        .await?;

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

    pub(crate) fn current_source_maps(&self) -> Option<crate::AppSourceMaps> {
        let approval = self.approval_source_map.lock().unwrap().clone();
        let clear = self.clear_source_map.lock().unwrap().clone();

        if approval.is_none() && clear.is_none() {
            None
        } else {
            Some(crate::AppSourceMaps {
                approval_source_map: approval,
                clear_source_map: clear,
            })
        }
    }

    pub(crate) fn logic_error_for(
        &self,
        error_str: &str,
        is_clear_state_program: bool,
    ) -> Option<String> {
        if !(error_str.contains("logic eval error") || error_str.contains("logic error")) {
            return None;
        }

        let tx_err = crate::transactions::TransactionResultError::ParsingError {
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
    #[allow(clippy::too_many_arguments)]
    pub async fn deploy(
        &self,
        on_update: Option<crate::applications::app_deployer::OnUpdate>,
        on_schema_break: Option<crate::applications::app_deployer::OnSchemaBreak>,
        create_params: Option<
            crate::applications::app_factory::types::AppFactoryCreateMethodCallParams,
        >,
        update_params: Option<AppClientMethodCallParams>,
        delete_params: Option<AppClientMethodCallParams>,
        existing_deployments: Option<crate::applications::app_deployer::AppLookup>,
        ignore_cache: Option<bool>,
        app_name: Option<String>,
        send_params: Option<crate::transactions::composer::SendParams>,
    ) -> Result<
        (
            AppClient,
            crate::applications::app_factory::types::AppFactoryDeployResult,
        ),
        AppFactoryError,
    > {
        // Prepare create/update/delete deploy params
        // Auto-detect deploy-time controls if not explicitly provided
        let mut resolved_updatable = self.updatable;
        let mut resolved_deletable = self.deletable;
        if resolved_updatable.is_none() || resolved_deletable.is_none() {
            if let Some(source) = self.app_spec().source.as_ref() {
                if let Ok(approval_teal) = source.get_decoded_approval() {
                    let has_updatable = approval_teal
                        .contains(crate::clients::app_manager::UPDATABLE_TEMPLATE_NAME);
                    let has_deletable = approval_teal
                        .contains(crate::clients::app_manager::DELETABLE_TEMPLATE_NAME);
                    if resolved_updatable.is_none() && has_updatable {
                        resolved_updatable = Some(true);
                    }
                    if resolved_deletable.is_none() && has_deletable {
                        resolved_deletable = Some(true);
                    }
                }
            }
        }
        let resolved_deploy_time_params = self.deploy_time_params.clone();

        let create_deploy_params = match create_params {
            Some(cp) => crate::applications::app_deployer::CreateParams::AppCreateMethodCall(
                self.params().create(cp)?,
            ),
            None => crate::applications::app_deployer::CreateParams::AppCreateCall(
                self.params().bare().create(None)?,
            ),
        };

        let update_deploy_params = match update_params {
            Some(up) => crate::applications::app_deployer::UpdateParams::AppUpdateMethodCall(
                self.params().deploy_update(up)?,
            ),
            None => crate::applications::app_deployer::UpdateParams::AppUpdateCall(
                self.params().bare().deploy_update(None)?,
            ),
        };

        let delete_deploy_params = match delete_params {
            Some(dp) => crate::applications::app_deployer::DeleteParams::AppDeleteMethodCall(
                self.params().deploy_delete(dp)?,
            ),
            None => crate::applications::app_deployer::DeleteParams::AppDeleteCall(
                self.params().bare().deploy_delete(None)?,
            ),
        };

        let metadata = crate::applications::app_deployer::AppDeployMetadata {
            name: app_name.unwrap_or_else(|| self.app_name.clone()),
            version: self.version.clone(),
            updatable: resolved_updatable,
            deletable: resolved_deletable,
        };

        let deploy_params = crate::applications::app_deployer::AppDeployParams {
            metadata,
            deploy_time_params: resolved_deploy_time_params,
            on_schema_break,
            on_update,
            create_params: create_deploy_params,
            update_params: update_deploy_params,
            delete_params: delete_deploy_params,
            existing_deployments,
            ignore_cache,
            send_params: send_params.unwrap_or_default(),
        };

        let mut app_deployer = self.algorand.as_ref().app_deployer();

        let deploy_result = app_deployer
            .deploy(deploy_params)
            .await
            .map_err(|e| AppFactoryError::AppDeployerError(e.to_string()))?;

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

        // Convert deploy result into factory result (simplified)
        let factory_result = crate::applications::app_factory::types::AppFactoryDeployResult {
            app: app_metadata.clone(),
            operation_performed: deploy_result,
            create_result: None,
            update_result: None,
            delete_result: None,
        };

        Ok((app_client, factory_result))
    }
}
