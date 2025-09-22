//! Send helpers for `AppFactory`, mirroring the TypeScript/Python surface by exposing
//! create-only flows while attaching compiled program metadata to results.

use super::AppFactory;
use super::utils::{
    build_bare_create_params, build_create_method_call_params, merge_args_with_defaults,
    transform_transaction_error_for_factory,
};
use crate::applications::app_client::CompilationParams;
use crate::applications::app_client::{AppClient, AppClientParams};
use crate::applications::app_factory::{
    AppFactoryCreateMethodCallParams, AppFactoryCreateMethodCallResult, AppFactoryCreateParams,
    AppFactoryMethodCallResult,
};
use crate::transactions::{SendAppCreateResult, SendParams, TransactionSenderError};

/// Sends factory-backed create transactions and returns both the client and send results.
pub struct TransactionSender<'app_factory> {
    pub(crate) factory: &'app_factory AppFactory,
}

/// Bare transaction helpers for AppFactory create flows.
pub struct BareTransactionSender<'app_factory> {
    pub(crate) factory: &'app_factory AppFactory,
}

impl<'app_factory> TransactionSender<'app_factory> {
    pub fn bare(&self) -> BareTransactionSender<'app_factory> {
        BareTransactionSender {
            factory: self.factory,
        }
    }

    /// Send an app creation via method call and return (AppClient, SendAppCreateResult)
    pub async fn create(
        &self,
        params: AppFactoryCreateMethodCallParams,
        send_params: Option<SendParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<(AppClient, AppFactoryCreateMethodCallResult), TransactionSenderError> {
        // Merge user args with ARC-56 literal defaults
        let merged_args = merge_args_with_defaults(self.factory, &params.method, &params.args)
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        // Prepare compiled programs, method and sender in one step
        let (compiled, method, sender) = self
            .factory
            .prepare_compiled_method(&params.method, compilation_params, &params.sender)
            .await
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        // Avoid moving compiled bytes we still need later
        let approval_bytes = compiled.approval.compiled_base64_to_bytes.clone();
        let clear_bytes = compiled.clear.compiled_base64_to_bytes.clone();

        let create_params = build_create_method_call_params(
            self.factory,
            sender,
            &params,
            method,
            merged_args,
            approval_bytes.clone(),
            clear_bytes.clone(),
        );

        let mut result = self
            .factory
            .algorand()
            .send()
            .app_create_method_call(create_params, send_params)
            .await
            .map_err(|e| transform_transaction_error_for_factory(self.factory, e, false))?;

        result.compiled_approval = Some(approval_bytes);
        result.compiled_clear = Some(clear_bytes);
        result.approval_source_map = compiled.approval.source_map.clone();
        result.clear_source_map = compiled.clear.source_map.clone();

        let app_client = AppClient::new(AppClientParams {
            app_id: result.app_id,
            app_spec: self.factory.app_spec().clone(),
            algorand: self.factory.algorand().clone(),
            app_name: Some(self.factory.app_name().to_string()),
            default_sender: self.factory.default_sender.clone(),
            default_signer: self.factory.default_signer.clone(),
            source_maps: self.factory.current_source_maps(),
            transaction_composer_config: self.factory.transaction_composer_config.clone(),
        });

        // Extract ABI return value as ABIValue (if present and decodable)
        let arc56_return = self
            .factory
            .parse_method_return_value(&result.abi_return)
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        Ok((
            app_client,
            AppFactoryMethodCallResult::new(result, arc56_return),
        ))
    }
}

impl BareTransactionSender<'_> {
    /// Send a bare app creation and return (AppClient, SendAppCreateResult)
    pub async fn create(
        &self,
        params: Option<AppFactoryCreateParams>,
        send_params: Option<SendParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<(AppClient, SendAppCreateResult), TransactionSenderError> {
        let params = params.unwrap_or_default();

        // Compile using centralized helper (with override params)
        let compiled = self
            .factory
            .compile_programs_with(compilation_params)
            .await
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;

        let create_params = build_bare_create_params(
            self.factory,
            sender,
            &params,
            compiled.approval.compiled_base64_to_bytes.clone(),
            compiled.clear.compiled_base64_to_bytes.clone(),
        );

        let mut result = self
            .factory
            .algorand()
            .send()
            .app_create(create_params, send_params)
            .await
            .map_err(|e| transform_transaction_error_for_factory(self.factory, e, false))?;

        result.compiled_approval = Some(compiled.approval.compiled_base64_to_bytes.clone());
        result.compiled_clear = Some(compiled.clear.compiled_base64_to_bytes.clone());
        result.approval_source_map = compiled.approval.source_map.clone();
        result.clear_source_map = compiled.clear.source_map.clone();

        let app_client = AppClient::new(AppClientParams {
            app_id: result.app_id,
            app_spec: self.factory.app_spec().clone(),
            algorand: self.factory.algorand().clone(),
            app_name: Some(self.factory.app_name().to_string()),
            default_sender: self.factory.default_sender.clone(),
            default_signer: self.factory.default_signer.clone(),
            source_maps: self.factory.current_source_maps(),
            transaction_composer_config: self.factory.transaction_composer_config.clone(),
        });

        Ok((app_client, result))
    }
}
