use super::{AppFactory, AppFactoryError};
use crate::applications::app_deployer::{
    AppProgram, DeployAppCreateMethodCallParams, DeployAppCreateParams,
    DeployAppDeleteMethodCallParams, DeployAppDeleteParams, DeployAppUpdateMethodCallParams,
    DeployAppUpdateParams,
};
use algokit_abi::ABIMethod;
use algokit_transact::OnApplicationComplete;
use algokit_transact::StateSchema as TxStateSchema;
use std::str::FromStr;
// use std::str::FromStr;

pub struct ParamsBuilder<'a> {
    pub(crate) factory: &'a AppFactory,
}

pub struct BareParamsBuilder<'a> {
    pub(crate) factory: &'a AppFactory,
}

impl<'a> ParamsBuilder<'a> {
    pub fn bare(&self) -> BareParamsBuilder<'a> {
        BareParamsBuilder {
            factory: self.factory,
        }
    }

    /// Create DeployAppCreateMethodCallParams from factory inputs
    pub fn create(
        &self,
        params: super::types::AppFactoryCreateMethodCallParams,
    ) -> Result<DeployAppCreateMethodCallParams, AppFactoryError> {
        let (approval_teal, clear_teal) = decode_teal_from_spec(self.factory)?;
        let method = to_abi_method(self.factory.app_spec(), &params.method)?;
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(AppFactoryError::ValidationError)?;

        // Merge user args with ARC-56 literal defaults for create-time ABI
        let merged_args = super::utils::merge_create_args_with_defaults(
            self.factory,
            &params.method,
            &params.args,
        )?;

        Ok(DeployAppCreateMethodCallParams {
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
            on_complete: params.on_complete.unwrap_or(OnApplicationComplete::NoOp),
            approval_program: AppProgram::Teal(approval_teal),
            clear_state_program: AppProgram::Teal(clear_teal),
            method,
            args: merged_args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
            global_state_schema: params
                .global_state_schema
                .or_else(|| Some(default_global_schema(self.factory))),
            local_state_schema: params
                .local_state_schema
                .or_else(|| Some(default_local_schema(self.factory))),
            extra_program_pages: params.extra_program_pages,
        })
    }

    /// Create DeployAppUpdateMethodCallParams
    pub fn deploy_update(
        &self,
        params: crate::applications::app_client::AppClientMethodCallParams,
    ) -> Result<DeployAppUpdateMethodCallParams, AppFactoryError> {
        let method = to_abi_method(self.factory.app_spec(), &params.method)?;
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(AppFactoryError::ValidationError)?;

        let merged_args = super::utils::merge_create_args_with_defaults(
            self.factory,
            &params.method,
            &Some(params.args.clone()),
        )?;

        Ok(DeployAppUpdateMethodCallParams {
            sender,
            signer: params.signer,
            rekey_to: params
                .rekey_to
                .as_ref()
                .and_then(|s| algokit_transact::Address::from_str(s).ok()),
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            method,
            args: merged_args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }

    /// Create DeployAppDeleteMethodCallParams
    pub fn deploy_delete(
        &self,
        params: crate::applications::app_client::AppClientMethodCallParams,
    ) -> Result<DeployAppDeleteMethodCallParams, AppFactoryError> {
        let method = to_abi_method(self.factory.app_spec(), &params.method)?;
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(AppFactoryError::ValidationError)?;

        let merged_args = super::utils::merge_create_args_with_defaults(
            self.factory,
            &params.method,
            &Some(params.args.clone()),
        )?;

        Ok(DeployAppDeleteMethodCallParams {
            sender,
            signer: params.signer,
            rekey_to: params
                .rekey_to
                .as_ref()
                .and_then(|s| algokit_transact::Address::from_str(s).ok()),
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            method,
            args: merged_args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }
}

impl BareParamsBuilder<'_> {
    /// Create DeployAppCreateParams from factory inputs
    pub fn create(
        &self,
        params: Option<super::types::AppFactoryCreateParams>,
    ) -> Result<DeployAppCreateParams, AppFactoryError> {
        let params = params.unwrap_or_default();
        let (approval_teal, clear_teal) = decode_teal_from_spec(self.factory)?;
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(AppFactoryError::ValidationError)?;

        Ok(DeployAppCreateParams {
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
            on_complete: params.on_complete.unwrap_or(OnApplicationComplete::NoOp),
            approval_program: AppProgram::Teal(approval_teal),
            clear_state_program: AppProgram::Teal(clear_teal),
            args: params.args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
            global_state_schema: params
                .global_state_schema
                .or_else(|| Some(default_global_schema(self.factory))),
            local_state_schema: params
                .local_state_schema
                .or_else(|| Some(default_local_schema(self.factory))),
            extra_program_pages: params.extra_program_pages,
        })
    }

    /// Create DeployAppUpdateParams
    pub fn deploy_update(
        &self,
        params: Option<crate::applications::app_client::AppClientBareCallParams>,
    ) -> Result<DeployAppUpdateParams, AppFactoryError> {
        let params = params.unwrap_or_default();
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(AppFactoryError::ValidationError)?;

        Ok(DeployAppUpdateParams {
            sender,
            signer: params.signer,
            rekey_to: params
                .rekey_to
                .as_ref()
                .and_then(|s| algokit_transact::Address::from_str(s).ok()),
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            args: params.args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }

    /// Create DeployAppDeleteParams
    pub fn deploy_delete(
        &self,
        params: Option<crate::applications::app_client::AppClientBareCallParams>,
    ) -> Result<DeployAppDeleteParams, AppFactoryError> {
        let params = params.unwrap_or_default();
        let sender = self
            .factory
            .get_sender_address(&params.sender)
            .map_err(AppFactoryError::ValidationError)?;

        Ok(DeployAppDeleteParams {
            sender,
            signer: params.signer,
            rekey_to: params
                .rekey_to
                .as_ref()
                .and_then(|s| algokit_transact::Address::from_str(s).ok()),
            note: params.note,
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            args: params.args,
            account_references: None,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }
}

fn decode_teal_from_spec(factory: &AppFactory) -> Result<(String, String), AppFactoryError> {
    let source = factory.app_spec().source.as_ref().ok_or_else(|| {
        AppFactoryError::CompilationError("Missing source in app spec".to_string())
    })?;
    let approval = source
        .get_decoded_approval()
        .map_err(|e| AppFactoryError::CompilationError(e.to_string()))?;
    let clear = source
        .get_decoded_clear()
        .map_err(|e| AppFactoryError::CompilationError(e.to_string()))?;
    Ok((approval, clear))
}

fn default_global_schema(factory: &AppFactory) -> TxStateSchema {
    let s = &factory.app_spec().state.schema.global_state;
    TxStateSchema {
        num_uints: s.ints,
        num_byte_slices: s.bytes,
    }
}

fn default_local_schema(factory: &AppFactory) -> TxStateSchema {
    let s = &factory.app_spec().state.schema.local_state;
    TxStateSchema {
        num_uints: s.ints,
        num_byte_slices: s.bytes,
    }
}

pub(crate) fn to_abi_method(
    contract: &algokit_abi::Arc56Contract,
    method: &str,
) -> Result<ABIMethod, AppFactoryError> {
    contract
        .find_abi_method(method)
        .map_err(|e| AppFactoryError::MethodNotFound(e.to_string()))
}

// Note: Deploy param structs accept Address already parsed where relevant; factory-level
// params use String types mirroring Python/TS. For now we pass through as-is.
