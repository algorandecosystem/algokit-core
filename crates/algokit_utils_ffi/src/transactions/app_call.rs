use algokit_transact::Address;
use algokit_transact_ffi::transactions::app_call::{
    BoxReference, OnApplicationComplete, StateSchema,
};

use super::common::{CommonParams, UtilsError};

use algokit_utils::transactions::app_call::{
    AppCallParams as RustAppCallParams, AppCreateParams as RustAppCreateParams,
};

#[derive(uniffi::Record)]
pub struct AppCallParams {
    pub common_params: CommonParams,
    /// ID of the app being called.
    pub app_id: u64,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Transaction specific arguments available in the app's
    /// approval program and clear state program.
    #[uniffi(default = None)]
    pub args: Option<Vec<Vec<u8>>>,
    /// List of accounts in addition to the sender that may be accessed
    /// from the app's approval program and clear state program.
    #[uniffi(default = None)]
    pub account_references: Option<Vec<String>>,
    /// List of apps in addition to the current app that may be called
    /// from the app's approval program and clear state program.
    #[uniffi(default = None)]
    pub app_references: Option<Vec<u64>>,
    /// Lists the assets whose parameters may be accessed by this app's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    #[uniffi(default = None)]
    pub asset_references: Option<Vec<u64>>,
    /// The boxes that should be made available for the runtime of the program.
    #[uniffi(default = None)]
    pub box_references: Option<Vec<BoxReference>>,
}

impl TryFrom<AppCallParams> for RustAppCallParams {
    type Error = UtilsError;

    fn try_from(value: AppCallParams) -> Result<Self, Self::Error> {
        Ok(RustAppCallParams {
            common_params: value.common_params.try_into()?,
            app_id: value.app_id,
            on_complete: value.on_complete.into(),
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| {
                    accounts
                        .into_iter()
                        .map(|a| a.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(|e: <algokit_transact::Address as std::str::FromStr>::Err| {
                    UtilsError::UtilsError {
                        message: e.to_string(),
                    }
                })?,
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

impl From<RustAppCallParams> for AppCallParams {
    fn from(value: RustAppCallParams) -> Self {
        AppCallParams {
            common_params: value.common_params.into(),
            app_id: value.app_id,
            on_complete: value.on_complete.into(),
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        }
    }
}

#[derive(uniffi::Record)]
pub struct AppCreateParams {
    pub common_params: CommonParams,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Logic executed for every app call transaction, except when
    /// on-completion is set to "clear".
    ///
    /// Approval programs may reject the transaction.
    pub approval_program: Vec<u8>,
    /// Logic executed for app call transactions with on-completion set to "clear".
    ///
    /// Clear state programs cannot reject the transaction.
    pub clear_state_program: Vec<u8>,
    /// Holds the maximum number of global state values.
    ///
    /// This cannot be changed after creation.
    #[uniffi(default = None)]
    pub global_state_schema: Option<StateSchema>,
    /// Holds the maximum number of local state values.
    ///
    /// This cannot be changed after creation.
    #[uniffi(default = None)]
    pub local_state_schema: Option<StateSchema>,
    /// Number of additional pages allocated to the app's approval
    /// and clear state programs.
    ///
    /// Each extra program page is 2048 bytes. The sum of approval program
    /// and clear state program may not exceed 2048*(1+extra_program_pages) bytes.
    /// Currently, the maximum value is 3.
    /// This cannot be changed after creation.
    #[uniffi(default = None)]
    pub extra_program_pages: Option<u64>,
    /// Transaction specific arguments available in the app's
    /// approval program and clear state program.
    #[uniffi(default = None)]
    pub args: Option<Vec<Vec<u8>>>,
    /// List of accounts in addition to the sender that may be accessed
    /// from the app's approval program and clear state program.
    #[uniffi(default = None)]
    pub account_references: Option<Vec<String>>,
    /// List of apps in addition to the current app that may be called
    /// from the app's approval program and clear state program.
    #[uniffi(default = None)]
    pub app_references: Option<Vec<u64>>,
    /// Lists the assets whose parameters may be accessed by this app's
    /// approval program and clear state program.
    ///
    /// The access is read-only.
    #[uniffi(default = None)]
    pub asset_references: Option<Vec<u64>>,
    /// The boxes that should be made available for the runtime of the program.
    #[uniffi(default = None)]
    pub box_references: Option<Vec<BoxReference>>,
}

impl TryFrom<AppCreateParams> for RustAppCreateParams {
    type Error = UtilsError;

    fn try_from(value: AppCreateParams) -> Result<Self, Self::Error> {
        Ok(RustAppCreateParams {
            common_params: value.common_params.try_into()?,
            on_complete: value.on_complete.into(),
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            global_state_schema: value.global_state_schema.map(|s| s.into()),
            local_state_schema: value.local_state_schema.map(|s| s.into()),
            extra_program_pages: value.extra_program_pages,
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| {
                    accounts
                        .into_iter()
                        .map(|a| a.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(
                    |e: <Address as std::str::FromStr>::Err| UtilsError::UtilsError {
                        message: e.to_string(),
                    },
                )?,
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        })
    }
}

impl From<RustAppCreateParams> for AppCreateParams {
    fn from(value: RustAppCreateParams) -> Self {
        AppCreateParams {
            common_params: value.common_params.into(),
            on_complete: value.on_complete.into(),
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            global_state_schema: value.global_state_schema.map(|s| s.into()),
            local_state_schema: value.local_state_schema.map(|s| s.into()),
            extra_program_pages: value.extra_program_pages,
            args: value.args,
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
        }
    }
}
