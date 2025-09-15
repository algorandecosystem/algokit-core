use super::AppFactory;
use crate::applications::app_client::CompilationParams;
use crate::applications::app_client::{AppClient, AppClientParams};
use crate::transactions::{SendAppCreateResult, SendParams, TransactionSenderError};

pub struct TransactionSender<'a> {
    pub(crate) factory: &'a AppFactory,
}

pub struct BareTransactionSender<'a> {
    pub(crate) factory: &'a AppFactory,
}

impl<'a> TransactionSender<'a> {
    pub fn bare(&self) -> BareTransactionSender<'a> {
        BareTransactionSender {
            factory: self.factory,
        }
    }

    /// Send an app creation via method call and return (AppClient, SendAppCreateResult)
    pub async fn create(
        &self,
        params: super::types::AppFactoryCreateMethodCallParams,
        send_params: Option<SendParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<(AppClient, SendAppCreateResult), TransactionSenderError> {
        // Compile using centralized helper (with override params)
        let compiled = self
            .factory
            .compile_programs_with(compilation_params)
            .await
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        // Merge user args with ARC-56 literal defaults
        let merged_args = super::utils::merge_create_args_with_defaults(
            self.factory,
            &params.method,
            &params.args,
        )
        .map_err(|e| TransactionSenderError::ValidationError {
            message: e.to_string(),
        })?;

        // Resolve ABI method
        let method = super::params_builder::to_abi_method(self.factory.app_spec(), &params.method)
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        // Resolve sender
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;

        // Default schemas from spec when not provided
        let global_schema = params.global_state_schema.or_else(|| {
            let s = &self.factory.app_spec().state.schema.global_state;
            Some(algokit_transact::StateSchema {
                num_uints: s.ints,
                num_byte_slices: s.bytes,
            })
        });
        let local_schema = params.local_state_schema.or_else(|| {
            let s = &self.factory.app_spec().state.schema.local_state;
            Some(algokit_transact::StateSchema {
                num_uints: s.ints,
                num_byte_slices: s.bytes,
            })
        });

        let create_params = crate::transactions::AppCreateMethodCallParams {
            common_params: crate::transactions::common::CommonTransactionParams {
                sender,
                ..Default::default()
            },
            on_complete: params
                .on_complete
                .unwrap_or(algokit_transact::OnApplicationComplete::NoOp),
            approval_program: compiled.approval.compiled_base64_to_bytes,
            clear_state_program: compiled.clear.compiled_base64_to_bytes,
            method,
            args: merged_args,
            account_references: params.account_references,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
            global_state_schema: global_schema,
            local_state_schema: local_schema,
            extra_program_pages: params.extra_program_pages,
        };

        let result = self
            .factory
            .algorand()
            .send()
            .app_create_method_call(create_params, send_params)
            .await
            .map_err(|e| {
                super::utils::transform_transaction_error_for_factory(self.factory, e, false)
            })?;

        let app_client = AppClient::new(AppClientParams {
            app_id: Some(result.app_id),
            app_spec: self.factory.app_spec().clone(),
            algorand: self.factory.algorand().clone(),
            app_name: Some(self.factory.app_name().to_string()),
            default_sender: self.factory.default_sender.clone(),
            source_maps: None,
        });

        Ok((app_client, result))
    }

    /// Send an app update via method call
    pub async fn update(
        &self,
        params: super::types::AppFactoryUpdateMethodCallParams,
        send_params: Option<SendParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<crate::transactions::SendAppUpdateResult, TransactionSenderError> {
        let compiled = self
            .factory
            .compile_programs_with(compilation_params)
            .await
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        let method = super::params_builder::to_abi_method(self.factory.app_spec(), &params.method)
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;

        let update_params = crate::transactions::AppUpdateMethodCallParams {
            common_params: crate::transactions::common::CommonTransactionParams {
                sender,
                ..Default::default()
            },
            app_id: params.app_id,
            approval_program: compiled.approval.compiled_base64_to_bytes,
            clear_state_program: compiled.clear.compiled_base64_to_bytes,
            method,
            args: params.args.unwrap_or_default(),
            account_references: params.account_references,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        };

        self.factory
            .algorand()
            .send()
            .app_update_method_call(update_params, send_params)
            .await
            .map_err(|e| {
                super::utils::transform_transaction_error_for_factory(self.factory, e, false)
            })
    }

    /// Send an app delete via method call
    pub async fn delete(
        &self,
        params: super::types::AppFactoryDeleteMethodCallParams,
        send_params: Option<SendParams>,
    ) -> Result<crate::transactions::SendAppCallResult, TransactionSenderError> {
        let method = super::params_builder::to_abi_method(self.factory.app_spec(), &params.method)
            .map_err(|e| TransactionSenderError::ValidationError {
                message: e.to_string(),
            })?;

        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;

        let delete_params = crate::transactions::AppDeleteMethodCallParams {
            common_params: crate::transactions::common::CommonTransactionParams {
                sender,
                ..Default::default()
            },
            app_id: params.app_id,
            method,
            args: params.args.unwrap_or_default(),
            account_references: params.account_references,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        };

        self.factory
            .algorand()
            .send()
            .app_delete_method_call(delete_params, send_params)
            .await
            .map_err(|e| {
                super::utils::transform_transaction_error_for_factory(self.factory, e, true)
            })
    }
}

impl BareTransactionSender<'_> {
    /// Send a bare app creation and return (AppClient, SendAppCreateResult)
    pub async fn create(
        &self,
        params: Option<super::types::AppFactoryCreateParams>,
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

        let global_schema = params.global_state_schema.or_else(|| {
            let s = &self.factory.app_spec().state.schema.global_state;
            Some(algokit_transact::StateSchema {
                num_uints: s.ints,
                num_byte_slices: s.bytes,
            })
        });
        let local_schema = params.local_state_schema.or_else(|| {
            let s = &self.factory.app_spec().state.schema.local_state;
            Some(algokit_transact::StateSchema {
                num_uints: s.ints,
                num_byte_slices: s.bytes,
            })
        });

        let create_params = crate::transactions::AppCreateParams {
            common_params: crate::transactions::common::CommonTransactionParams {
                sender,
                ..Default::default()
            },
            on_complete: params
                .on_complete
                .unwrap_or(algokit_transact::OnApplicationComplete::NoOp),
            approval_program: compiled.approval.compiled_base64_to_bytes,
            clear_state_program: compiled.clear.compiled_base64_to_bytes,
            args: params.args,
            account_references: params.account_references,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
            global_state_schema: global_schema,
            local_state_schema: local_schema,
            extra_program_pages: params.extra_program_pages,
        };

        let result = self
            .factory
            .algorand()
            .send()
            .app_create(create_params, send_params)
            .await
            .map_err(|e| {
                super::utils::transform_transaction_error_for_factory(self.factory, e, false)
            })?;

        let app_client = AppClient::new(AppClientParams {
            app_id: Some(result.app_id),
            app_spec: self.factory.app_spec().clone(),
            algorand: self.factory.algorand().clone(),
            app_name: Some(self.factory.app_name().to_string()),
            default_sender: self.factory.default_sender.clone(),
            source_maps: None,
        });

        Ok((app_client, result))
    }

    /// Send an app update (bare)
    pub async fn update(
        &self,
        params: super::types::AppFactoryUpdateParams,
        send_params: Option<SendParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<crate::transactions::SendAppUpdateResult, TransactionSenderError> {
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

        let update_params = crate::transactions::AppUpdateParams {
            common_params: crate::transactions::common::CommonTransactionParams {
                sender,
                ..Default::default()
            },
            app_id: params.app_id,
            approval_program: compiled.approval.compiled_base64_to_bytes,
            clear_state_program: compiled.clear.compiled_base64_to_bytes,
            args: params.args,
            account_references: params.account_references,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        };

        self.factory
            .algorand()
            .send()
            .app_update(update_params, send_params)
            .await
            .map_err(|e| {
                super::utils::transform_transaction_error_for_factory(self.factory, e, false)
            })
    }

    /// Send an app delete (bare)
    pub async fn delete(
        &self,
        params: super::types::AppFactoryDeleteParams,
        send_params: Option<SendParams>,
    ) -> Result<crate::transactions::SendTransactionResult, TransactionSenderError> {
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;

        let delete_params = crate::transactions::AppDeleteParams {
            common_params: crate::transactions::common::CommonTransactionParams {
                sender,
                ..Default::default()
            },
            app_id: params.app_id,
            args: params.args,
            account_references: params.account_references,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        };

        self.factory
            .algorand()
            .send()
            .app_delete(delete_params, send_params)
            .await
            .map_err(|e| {
                super::utils::transform_transaction_error_for_factory(self.factory, e, true)
            })
    }
}
