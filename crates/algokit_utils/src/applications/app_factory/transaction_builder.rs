use super::AppFactory;
use crate::applications::app_client::CompilationParams;
use crate::transactions::composer::ComposerError;
use algokit_transact::Transaction;

pub struct TransactionBuilder<'a> {
    pub(crate) factory: &'a AppFactory,
}

pub struct BareTransactionBuilder<'a> {
    pub(crate) factory: &'a AppFactory,
}

impl<'a> TransactionBuilder<'a> {
    pub fn bare(&self) -> BareTransactionBuilder<'a> {
        BareTransactionBuilder {
            factory: self.factory,
        }
    }

    pub async fn create(
        &self,
        params: super::types::AppFactoryCreateMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<Vec<Transaction>, ComposerError> {
        // Compile using centralized helper
        let compiled = self
            .factory
            .compile_programs_with(compilation_params)
            .await
            .map_err(|e| ComposerError::TransactionError {
                message: e.to_string(),
            })?;

        // Resolve ABI method
        let method = super::params_builder::to_abi_method(self.factory.app_spec(), &params.method)
            .map_err(|e| ComposerError::TransactionError {
                message: e.to_string(),
            })?;

        // Resolve sender
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| ComposerError::TransactionError { message: e })?;

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
            sender,
            signer: params.signer,
            rekey_to: params.rekey_to,
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            on_complete: params
                .on_complete
                .unwrap_or(algokit_transact::OnApplicationComplete::NoOp),
            approval_program: compiled.approval.compiled_base64_to_bytes,
            clear_state_program: compiled.clear.compiled_base64_to_bytes,
            method,
            args: params.args.unwrap_or_default(),
            account_references: params.account_references,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
            global_state_schema: global_schema,
            local_state_schema: local_schema,
            extra_program_pages: params.extra_program_pages,
        };

        self.factory
            .algorand()
            .create()
            .app_create_method_call(create_params)
            .await
    }

    pub async fn update(
        &self,
        params: super::types::AppFactoryUpdateMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<Vec<Transaction>, ComposerError> {
        let compiled = self
            .factory
            .compile_programs_with(compilation_params)
            .await
            .map_err(|e| ComposerError::TransactionError {
                message: e.to_string(),
            })?;

        let method = super::params_builder::to_abi_method(self.factory.app_spec(), &params.method)
            .map_err(|e| ComposerError::TransactionError {
                message: e.to_string(),
            })?;

        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| ComposerError::TransactionError { message: e })?;

        let update_params = crate::transactions::AppUpdateMethodCallParams {
            sender,
            signer: params.signer,
            rekey_to: params.rekey_to,
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
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
            .create()
            .app_update_method_call(update_params)
            .await
    }
}

impl BareTransactionBuilder<'_> {
    pub async fn create(
        &self,
        params: Option<super::types::AppFactoryCreateParams>,
        compilation_params: Option<CompilationParams>,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        let params = params.unwrap_or_default();

        // Compile using centralized helper
        let compiled = self
            .factory
            .compile_programs_with(compilation_params)
            .await
            .map_err(|e| ComposerError::TransactionError {
                message: e.to_string(),
            })?;

        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| ComposerError::TransactionError { message: e })?;

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
            sender,
            signer: params.signer,
            rekey_to: params.rekey_to,
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
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

        self.factory
            .algorand()
            .create()
            .app_create(create_params)
            .await
    }

    pub async fn update(
        &self,
        params: super::types::AppFactoryUpdateParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        let compiled = self
            .factory
            .compile_programs_with(compilation_params)
            .await
            .map_err(|e| ComposerError::TransactionError {
                message: e.to_string(),
            })?;

        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| ComposerError::TransactionError { message: e })?;

        let update_params = crate::transactions::AppUpdateParams {
            sender,
            signer: params.signer,
            rekey_to: params.rekey_to,
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
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
            .create()
            .app_update(update_params)
            .await
    }

    pub async fn delete(
        &self,
        params: super::types::AppFactoryDeleteParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(|e| ComposerError::TransactionError { message: e })?;

        let delete_params = crate::transactions::AppDeleteParams {
            sender,
            signer: params.signer,
            rekey_to: params.rekey_to,
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            app_id: params.app_id,
            args: params.args,
            account_references: params.account_references,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        };

        self.factory
            .algorand()
            .create()
            .app_delete(delete_params)
            .await
    }
}
