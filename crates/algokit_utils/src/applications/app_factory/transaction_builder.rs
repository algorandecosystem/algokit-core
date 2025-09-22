use super::AppFactory;
use super::utils::build_create_method_call_params;
use crate::applications::app_client::CompilationParams;
use crate::applications::app_factory::utils::resolve_signer;
use crate::applications::app_factory::{AppFactoryCreateMethodCallParams, AppFactoryCreateParams};
use crate::transactions::{AppCreateParams, composer::ComposerError};
use algokit_transact::Transaction;

pub struct TransactionBuilder<'app_factory> {
    pub(crate) factory: &'app_factory AppFactory,
}

pub struct BareTransactionBuilder<'app_factory> {
    pub(crate) factory: &'app_factory AppFactory,
}

impl<'app_factory> TransactionBuilder<'app_factory> {
    pub fn bare(&self) -> BareTransactionBuilder<'app_factory> {
        BareTransactionBuilder {
            factory: self.factory,
        }
    }

    pub async fn create(
        &self,
        params: AppFactoryCreateMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<Vec<Transaction>, ComposerError> {
        // Prepare compiled programs, method and sender in one step
        let (compiled, method, sender) = self
            .factory
            .prepare_compiled_method(&params.method, compilation_params, &params.sender)
            .await
            .map_err(|e| ComposerError::TransactionError {
                message: e.to_string(),
            })?;

        let create_params = build_create_method_call_params(
            self.factory,
            sender,
            &params,
            method,
            params.args.clone().unwrap_or_default(),
            compiled.approval.compiled_base64_to_bytes,
            compiled.clear.compiled_base64_to_bytes,
        );

        self.factory
            .algorand()
            .create()
            .app_create_method_call(create_params)
            .await
    }
}

impl BareTransactionBuilder<'_> {
    pub async fn create(
        &self,
        params: Option<AppFactoryCreateParams>,
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

        let create_params = AppCreateParams {
            sender,
            signer: resolve_signer(self.factory, &params.sender, params.signer),
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
}
