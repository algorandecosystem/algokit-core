use std::collections::HashMap;
use std::sync::Arc;

use algokit_abi::Arc56Contract;

use crate::AlgorandClient;
use crate::clients::app_manager::TealTemplateValue;
use crate::transactions::common::TransactionSigner;

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
    algorand: std::sync::Arc<AlgorandClient>,
    app_name: String,
    version: String,
    default_sender: Option<String>,
    #[allow(dead_code)]
    default_signer: Option<Arc<dyn TransactionSigner>>, // reserved for future use
    approval_source_map: Option<serde_json::Value>,
    clear_source_map: Option<serde_json::Value>,
    pub(crate) deploy_time_params: Option<HashMap<String, TealTemplateValue>>,
    pub(crate) updatable: Option<bool>,
    pub(crate) deletable: Option<bool>,
}

impl AppFactory {
    pub fn new(params: AppFactoryParams) -> Self {
        Self {
            app_spec: params.app_spec,
            algorand: params.algorand,
            app_name: params.app_name.unwrap_or_else(|| "<unnamed>".to_string()),
            version: params.version.unwrap_or_else(|| "1.0".to_string()),
            default_sender: params.default_sender,
            default_signer: params.default_signer,
            approval_source_map: None,
            clear_source_map: None,
            deploy_time_params: params.deploy_time_params,
            updatable: params.updatable,
            deletable: params.deletable,
        }
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }
    pub fn app_spec(&self) -> &Arc56Contract {
        &self.app_spec
    }
    pub fn algorand(&self) -> &std::sync::Arc<AlgorandClient> {
        &self.algorand
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

    pub fn import_source_maps(&mut self, source_maps: crate::AppSourceMaps) {
        self.approval_source_map = source_maps.approval_source_map;
        self.clear_source_map = source_maps.clear_source_map;
    }

    pub fn export_source_maps(&self) -> Result<crate::AppSourceMaps, AppFactoryError> {
        let approval = self.approval_source_map.clone().ok_or_else(|| {
            AppFactoryError::ValidationError("Approval source map not loaded".to_string())
        })?;
        let clear = self.clear_source_map.clone().ok_or_else(|| {
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
    ) -> crate::applications::app_client::AppClient {
        crate::applications::app_client::AppClient::new(
            crate::applications::app_client::AppClientParams {
                app_id: Some(app_id),
                app_spec: self.app_spec.clone(),
                algorand: self.algorand.clone(),
                app_name: Some(app_name.unwrap_or_else(|| self.app_name.clone())),
                default_sender: Some(
                    default_sender
                        .unwrap_or_else(|| self.default_sender.clone().unwrap_or_default()),
                ),
                source_maps: None,
            },
        )
    }

    pub async fn get_app_client_by_creator_and_name(
        &self,
        creator_address: &str,
        app_name: Option<String>,
        default_sender: Option<String>,
        ignore_cache: Option<bool>,
    ) -> Result<crate::applications::app_client::AppClient, AppFactoryError> {
        let name = app_name.unwrap_or_else(|| self.app_name.clone());
        crate::applications::app_client::AppClient::from_creator_and_name(
            creator_address,
            &name,
            self.app_spec.clone(),
            self.algorand.clone(),
            default_sender.or(self.default_sender.clone()),
            None,
            ignore_cache,
        )
        .await
        .map_err(|e| AppFactoryError::AppClientError(e.to_string()))
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
        use std::str::FromStr;
        algokit_transact::Address::from_str(sender_str)
            .map_err(|e| format!("Invalid sender address: {}", e))
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
        update_params: Option<crate::applications::app_client::AppClientMethodCallParams>,
        delete_params: Option<crate::applications::app_client::AppClientMethodCallParams>,
        existing_deployments: Option<crate::applications::app_deployer::AppLookup>,
        ignore_cache: Option<bool>,
        app_name: Option<String>,
        send_params: Option<crate::transactions::composer::SendParams>,
    ) -> Result<
        (
            crate::applications::app_client::AppClient,
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

        let mut app_deployer = self.algorand.app_deployer();

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

        let app_client = crate::applications::app_client::AppClient::new(
            crate::applications::app_client::AppClientParams {
                app_id: Some(app_metadata.app_id),
                app_spec: self.app_spec.clone(),
                algorand: self.algorand.clone(),
                app_name: Some(self.app_name.clone()),
                default_sender: self.default_sender.clone(),
                source_maps: None,
            },
        );

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
