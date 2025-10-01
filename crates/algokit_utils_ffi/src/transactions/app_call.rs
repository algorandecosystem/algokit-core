use crate::abi::{abi_type::ABIType, abi_value::ABIValue};
use crate::create_transaction_params;
use algokit_transact::Address;
use algokit_transact_ffi::transactions::app_call::{
    BoxReference, OnApplicationComplete, StateSchema,
};
use std::sync::Arc;

use super::common::{
    FfiTransactionSignerFromRust, RustTransactionSignerFromFfi, TransactionSigner, UtilsError,
};

use algokit_utils::transactions::app_call::{
    AppCallParams as RustAppCallParams, AppCreateParams as RustAppCreateParams,
};

use algokit_abi::ABIMethodArg as RustABIMethodArg;
use algokit_abi::ABIMethodArgType as RustABIMethodArgType;

#[derive(uniffi::Enum, Debug)]
pub enum ABIReferenceType {
    /// Reference to an account in the Accounts reference array
    Account,
    /// Reference to an application in the Applications reference array
    Application,
    /// Reference to an asset in the Assets reference array
    Asset,
}

#[derive(uniffi::Enum, Debug)]
pub enum ABITransactionType {
    /// Any transaction type
    Txn,
    /// Payment (algo transfer)
    Payment,
    /// Key registration (configure consensus participation)
    KeyRegistration,
    /// Asset configuration (create, configure, or destroy ASAs)
    AssetConfig,
    /// Asset transfer (ASA transfer)
    AssetTransfer,
    /// Asset freeze (freeze or unfreeze ASAs)
    AssetFreeze,
    /// App call (create, update, delete and call an app)
    AppCall,
}

#[derive(uniffi::Enum, Debug)]
pub enum ABIMethodArgType {
    /// A value that is directly encoded in the app arguments.
    Value(Arc<ABIType>),
    Transaction(ABITransactionType),
    Reference(ABIReferenceType),
}

impl From<ABIMethodArgType> for RustABIMethodArgType {
    fn from(value: ABIMethodArgType) -> Self {
        match value {
            ABIMethodArgType::Value(abi_type) => {
                RustABIMethodArgType::Value(abi_type.abi_type.clone())
            }
            ABIMethodArgType::Transaction(txn_type) => {
                let rust_txn_type = match txn_type {
                    ABITransactionType::Txn => algokit_abi::ABITransactionType::Txn,
                    ABITransactionType::Payment => algokit_abi::ABITransactionType::Payment,
                    ABITransactionType::KeyRegistration => {
                        algokit_abi::ABITransactionType::KeyRegistration
                    }
                    ABITransactionType::AssetConfig => algokit_abi::ABITransactionType::AssetConfig,
                    ABITransactionType::AssetTransfer => {
                        algokit_abi::ABITransactionType::AssetTransfer
                    }
                    ABITransactionType::AssetFreeze => algokit_abi::ABITransactionType::AssetFreeze,
                    ABITransactionType::AppCall => algokit_abi::ABITransactionType::AppCall,
                };
                RustABIMethodArgType::Transaction(rust_txn_type)
            }
            ABIMethodArgType::Reference(ref_type) => {
                let rust_ref_type = match ref_type {
                    ABIReferenceType::Account => algokit_abi::ABIReferenceType::Account,
                    ABIReferenceType::Application => algokit_abi::ABIReferenceType::Application,
                    ABIReferenceType::Asset => algokit_abi::ABIReferenceType::Asset,
                };
                RustABIMethodArgType::Reference(rust_ref_type)
            }
        }
    }
}

impl From<RustABIMethodArgType> for ABIMethodArgType {
    fn from(value: RustABIMethodArgType) -> Self {
        match value {
            RustABIMethodArgType::Value(abi_type) => {
                ABIMethodArgType::Value(Arc::new(ABIType { abi_type }))
            }
            RustABIMethodArgType::Transaction(txn_type) => {
                let ffi_txn_type = match txn_type {
                    algokit_abi::ABITransactionType::Txn => ABITransactionType::Txn,
                    algokit_abi::ABITransactionType::Payment => ABITransactionType::Payment,
                    algokit_abi::ABITransactionType::KeyRegistration => {
                        ABITransactionType::KeyRegistration
                    }
                    algokit_abi::ABITransactionType::AssetConfig => ABITransactionType::AssetConfig,
                    algokit_abi::ABITransactionType::AssetTransfer => {
                        ABITransactionType::AssetTransfer
                    }
                    algokit_abi::ABITransactionType::AssetFreeze => ABITransactionType::AssetFreeze,
                    algokit_abi::ABITransactionType::AppCall => ABITransactionType::AppCall,
                };
                ABIMethodArgType::Transaction(ffi_txn_type)
            }
            RustABIMethodArgType::Reference(ref_type) => {
                let ffi_ref_type = match ref_type {
                    algokit_abi::ABIReferenceType::Account => ABIReferenceType::Account,
                    algokit_abi::ABIReferenceType::Application => ABIReferenceType::Application,
                    algokit_abi::ABIReferenceType::Asset => ABIReferenceType::Asset,
                };
                ABIMethodArgType::Reference(ffi_ref_type)
            }
        }
    }
}

#[derive(uniffi::Record, Debug)]
pub struct ABIMethodArg {
    /// The type of the argument.
    pub arg_type: ABIMethodArgType,
    /// An optional name for the argument.
    pub name: Option<String>,
    /// An optional description of the argument.
    pub description: Option<String>,
}

impl From<ABIMethodArg> for RustABIMethodArg {
    fn from(value: ABIMethodArg) -> Self {
        RustABIMethodArg {
            arg_type: value.arg_type.into(),
            name: value.name,
            description: value.description,
            default_value: None, // FFI doesn't support default values yet
        }
    }
}

impl From<RustABIMethodArg> for ABIMethodArg {
    fn from(value: RustABIMethodArg) -> Self {
        ABIMethodArg {
            arg_type: value.arg_type.into(),
            name: value.name,
            description: value.description,
        }
    }
}

use algokit_abi::ABIMethod as RustABIMethod;

#[derive(uniffi::Record, Debug)]
pub struct ABIMethod {
    /// The name of the method.
    pub name: String,
    /// A list of the method's arguments.
    pub args: Vec<ABIMethodArg>,
    /// The return type of the method, or `None` if the method does not return a value.
    pub returns: Option<Arc<ABIType>>,
    /// An optional description of the method.
    pub description: Option<String>,
}

impl From<ABIMethod> for RustABIMethod {
    fn from(value: ABIMethod) -> Self {
        RustABIMethod {
            name: value.name,
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
            returns: value.returns.map(|r| r.abi_type.clone()),
            description: value.description,
        }
    }
}

impl From<RustABIMethod> for ABIMethod {
    fn from(value: RustABIMethod) -> Self {
        ABIMethod {
            name: value.name,
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
            returns: value.returns.map(|r| Arc::new(ABIType { abi_type: r })),
            description: value.description,
        }
    }
}

#[derive(uniffi::Enum, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum AppMethodCallArg {
    ABIValue(Arc<ABIValue>),
    AppCreateCall(AppCreateParams),
    AppUpdateCall(AppUpdateParams),
    AppDeleteCall(AppDeleteParams),
    AppCallMethodCall(AppCallMethodCallParams),
    AppCreateMethodCall(AppCreateMethodCallParams),
    AppUpdateMethodCall(AppUpdateMethodCallParams),
    AppDeleteMethodCall(AppDeleteMethodCallParams),
}

impl From<AppMethodCallArg> for algokit_utils::transactions::app_call::AppMethodCallArg {
    fn from(value: AppMethodCallArg) -> Self {
        match value {
            AppMethodCallArg::ABIValue(abi_value) => {
                algokit_utils::transactions::app_call::AppMethodCallArg::ABIValue(
                    abi_value.rust_value.clone(),
                )
            }
            AppMethodCallArg::AppCreateCall(app_create_params) => {
                algokit_utils::transactions::app_call::AppMethodCallArg::AppCreateCall(
                    app_create_params.try_into().unwrap(),
                )
            }
            AppMethodCallArg::AppUpdateCall(app_update_params) => {
                algokit_utils::transactions::app_call::AppMethodCallArg::AppUpdateCall(
                    app_update_params.try_into().unwrap(),
                )
            }
            AppMethodCallArg::AppDeleteCall(app_delete_params) => {
                algokit_utils::transactions::app_call::AppMethodCallArg::AppDeleteCall(
                    app_delete_params.try_into().unwrap(),
                )
            }
            AppMethodCallArg::AppCallMethodCall(app_call_method_params) => {
                algokit_utils::transactions::app_call::AppMethodCallArg::AppCallMethodCall(
                    app_call_method_params.try_into().unwrap(),
                )
            }
            AppMethodCallArg::AppCreateMethodCall(app_create_method_params) => {
                algokit_utils::transactions::app_call::AppMethodCallArg::AppCreateMethodCall(
                    app_create_method_params.try_into().unwrap(),
                )
            }
            AppMethodCallArg::AppUpdateMethodCall(app_update_method_params) => {
                algokit_utils::transactions::app_call::AppMethodCallArg::AppUpdateMethodCall(
                    app_update_method_params.try_into().unwrap(),
                )
            }
            AppMethodCallArg::AppDeleteMethodCall(app_delete_method_params) => {
                algokit_utils::transactions::app_call::AppMethodCallArg::AppDeleteMethodCall(
                    app_delete_method_params.try_into().unwrap(),
                )
            }
        }
    }
}

impl From<algokit_utils::transactions::app_call::AppMethodCallArg> for AppMethodCallArg {
    fn from(value: algokit_utils::transactions::app_call::AppMethodCallArg) -> Self {
        match value {
            algokit_utils::transactions::app_call::AppMethodCallArg::ABIValue(rust_value) => {
                AppMethodCallArg::ABIValue(Arc::new(ABIValue { rust_value }))
            }
            algokit_utils::transactions::app_call::AppMethodCallArg::AppCreateCall(
                app_create_params,
            ) => AppMethodCallArg::AppCreateCall(app_create_params.into()),
            algokit_utils::transactions::app_call::AppMethodCallArg::AppUpdateCall(
                app_update_params,
            ) => AppMethodCallArg::AppUpdateCall(app_update_params.into()),
            algokit_utils::transactions::app_call::AppMethodCallArg::AppDeleteCall(
                app_delete_params,
            ) => AppMethodCallArg::AppDeleteCall(app_delete_params.into()),
            algokit_utils::transactions::app_call::AppMethodCallArg::AppCallMethodCall(
                app_call_method_params,
            ) => AppMethodCallArg::AppCallMethodCall(app_call_method_params.into()),
            algokit_utils::transactions::app_call::AppMethodCallArg::AppCreateMethodCall(
                app_create_method_params,
            ) => AppMethodCallArg::AppCreateMethodCall(app_create_method_params.into()),
            algokit_utils::transactions::app_call::AppMethodCallArg::AppUpdateMethodCall(
                app_update_method_params,
            ) => AppMethodCallArg::AppUpdateMethodCall(app_update_method_params.into()),
            algokit_utils::transactions::app_call::AppMethodCallArg::AppDeleteMethodCall(
                app_delete_method_params,
            ) => AppMethodCallArg::AppDeleteMethodCall(app_delete_method_params.into()),
            _ => {
                // For variants we don't support in FFI, we need to handle them somehow
                // For now, let's just panic with a descriptive error
                panic!("Unsupported AppMethodCallArg variant for FFI conversion")
            }
        }
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppCallMethodCallParams {
        /// ID of the app being called.
        pub app_id: u64,
        /// The ABI method to call.
        pub method: ABIMethod,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Vec<AppMethodCallArg>,
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
        /// Defines what additional actions occur with the transaction.
        pub on_complete: OnApplicationComplete,
    }
}

impl TryFrom<AppCallMethodCallParams>
    for algokit_utils::transactions::app_call::AppCallMethodCallParams
{
    type Error = UtilsError;

    fn try_from(value: AppCallMethodCallParams) -> Result<Self, Self::Error> {
        Ok(
            algokit_utils::transactions::app_call::AppCallMethodCallParams {
                sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                    message: format!("Invalid sender address: {}", e),
                })?,
                signer: value.signer.map(|s| {
                    Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                        as Arc<dyn algokit_utils::transactions::common::TransactionSigner>
                }),
                rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                    UtilsError::UtilsError {
                        message: format!("Invalid rekey_to address: {}", e),
                    }
                })?,
                note: value.note,
                lease: value.lease.map(|l| {
                    let mut lease_bytes = [0u8; 32];
                    lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                    lease_bytes
                }),
                static_fee: value.static_fee,
                extra_fee: value.extra_fee,
                max_fee: value.max_fee,
                validity_window: value.validity_window,
                first_valid_round: value.first_valid_round,
                last_valid_round: value.last_valid_round,
                app_id: value.app_id,
                method: value.method.into(),
                args: value.args.into_iter().map(|arg| arg.into()).collect(),
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
                on_complete: value.on_complete.into(),
            },
        )
    }
}

impl From<algokit_utils::transactions::app_call::AppCallMethodCallParams>
    for AppCallMethodCallParams
{
    fn from(value: algokit_utils::transactions::app_call::AppCallMethodCallParams) -> Self {
        AppCallMethodCallParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            method: value.method.into(),
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
            account_references: value
                .account_references
                .map(|accounts| accounts.into_iter().map(|a| a.to_string()).collect()),
            app_references: value.app_references,
            asset_references: value.asset_references,
            box_references: value
                .box_references
                .map(|boxes| boxes.into_iter().map(|b| b.into()).collect()),
            on_complete: value.on_complete.into(),
        }
    }
}

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppCallParams {
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
}

impl TryFrom<AppCallParams> for RustAppCallParams {
    type Error = UtilsError;

    fn try_from(value: AppCallParams) -> Result<Self, Self::Error> {
        Ok(RustAppCallParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
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
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
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

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppCreateParams {
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
}

impl TryFrom<AppCreateParams> for RustAppCreateParams {
    type Error = UtilsError;

    fn try_from(value: AppCreateParams) -> Result<Self, Self::Error> {
        Ok(RustAppCreateParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            on_complete: value.on_complete.into(),
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            global_state_schema: value.global_state_schema.map(|s| s.into()),
            local_state_schema: value.local_state_schema.map(|s| s.into()),
            extra_program_pages: value.extra_program_pages.map(|p| p as u32),
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
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            on_complete: value.on_complete.into(),
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            global_state_schema: value.global_state_schema.map(|s| s.into()),
            local_state_schema: value.local_state_schema.map(|s| s.into()),
            extra_program_pages: value.extra_program_pages.map(|p| p as u64),
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

use algokit_utils::transactions::app_call::{
    AppDeleteParams as RustAppDeleteParams, AppUpdateParams as RustAppUpdateParams,
};

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppDeleteParams {
        /// ID of the app being deleted.
        pub app_id: u64,
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
}

impl TryFrom<AppDeleteParams> for RustAppDeleteParams {
    type Error = UtilsError;

    fn try_from(value: AppDeleteParams) -> Result<Self, Self::Error> {
        Ok(RustAppDeleteParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
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

impl From<RustAppDeleteParams> for AppDeleteParams {
    fn from(value: RustAppDeleteParams) -> Self {
        AppDeleteParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
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

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppUpdateParams {
        /// ID of the app being updated.
        pub app_id: u64,
        /// Logic executed for every app call transaction, except when
        /// on-completion is set to "clear".
        ///
        /// Approval programs may reject the transaction.
        pub approval_program: Vec<u8>,
        /// Logic executed for app call transactions with on-completion set to "clear".
        ///
        /// Clear state programs cannot reject the transaction.
        pub clear_state_program: Vec<u8>,
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
}

impl TryFrom<AppUpdateParams> for RustAppUpdateParams {
    type Error = UtilsError;

    fn try_from(value: AppUpdateParams) -> Result<Self, Self::Error> {
        Ok(RustAppUpdateParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
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

impl From<RustAppUpdateParams> for AppUpdateParams {
    fn from(value: RustAppUpdateParams) -> Self {
        AppUpdateParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
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

use algokit_utils::transactions::app_call::{
    AppCreateMethodCallParams as RustAppCreateMethodCallParams,
    AppDeleteMethodCallParams as RustAppDeleteMethodCallParams,
    AppUpdateMethodCallParams as RustAppUpdateMethodCallParams,
};

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppCreateMethodCallParams {
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
        /// The ABI method to call.
        pub method: ABIMethod,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Vec<AppMethodCallArg>,
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
}

impl TryFrom<AppCreateMethodCallParams> for RustAppCreateMethodCallParams {
    type Error = UtilsError;

    fn try_from(value: AppCreateMethodCallParams) -> Result<Self, Self::Error> {
        Ok(RustAppCreateMethodCallParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            on_complete: value.on_complete.into(),
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            global_state_schema: value.global_state_schema.map(|s| s.into()),
            local_state_schema: value.local_state_schema.map(|s| s.into()),
            extra_program_pages: value.extra_program_pages.map(|p| p as u32),
            method: value.method.into(),
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
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

impl From<RustAppCreateMethodCallParams> for AppCreateMethodCallParams {
    fn from(value: RustAppCreateMethodCallParams) -> Self {
        AppCreateMethodCallParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            on_complete: value.on_complete.into(),
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            global_state_schema: value.global_state_schema.map(|s| s.into()),
            local_state_schema: value.local_state_schema.map(|s| s.into()),
            extra_program_pages: value.extra_program_pages.map(|p| p as u64),
            method: value.method.into(),
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
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

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppUpdateMethodCallParams {
        /// ID of the app being updated.
        pub app_id: u64,
        /// Logic executed for every app call transaction, except when
        /// on-completion is set to "clear".
        ///
        /// Approval programs may reject the transaction.
        pub approval_program: Vec<u8>,
        /// Logic executed for app call transactions with on-completion set to "clear".
        ///
        /// Clear state programs cannot reject the transaction.
        pub clear_state_program: Vec<u8>,
        /// The ABI method to call.
        pub method: ABIMethod,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Vec<AppMethodCallArg>,
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
}

impl TryFrom<AppUpdateMethodCallParams> for RustAppUpdateMethodCallParams {
    type Error = UtilsError;

    fn try_from(value: AppUpdateMethodCallParams) -> Result<Self, Self::Error> {
        Ok(RustAppUpdateMethodCallParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            method: value.method.into(),
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
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

impl From<RustAppUpdateMethodCallParams> for AppUpdateMethodCallParams {
    fn from(value: RustAppUpdateMethodCallParams) -> Self {
        AppUpdateMethodCallParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            approval_program: value.approval_program,
            clear_state_program: value.clear_state_program,
            method: value.method.into(),
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
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

create_transaction_params! {
    #[derive(uniffi::Record)]
    pub struct AppDeleteMethodCallParams {
        /// ID of the app being deleted.
        pub app_id: u64,
        /// The ABI method to call.
        pub method: ABIMethod,
        /// Transaction specific arguments available in the app's
        /// approval program and clear state program.
        pub args: Vec<AppMethodCallArg>,
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
}

impl TryFrom<AppDeleteMethodCallParams> for RustAppDeleteMethodCallParams {
    type Error = UtilsError;

    fn try_from(value: AppDeleteMethodCallParams) -> Result<Self, Self::Error> {
        Ok(RustAppDeleteMethodCallParams {
            sender: value.sender.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Invalid sender address: {}", e),
            })?,
            signer: value.signer.map(|s| {
                Arc::new(RustTransactionSignerFromFfi { ffi_signer: s })
                    as Arc<dyn algokit_utils::transactions::common::TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.parse()).transpose().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Invalid rekey_to address: {}", e),
                }
            })?,
            note: value.note,
            lease: value.lease.map(|l| {
                let mut lease_bytes = [0u8; 32];
                lease_bytes.copy_from_slice(&l[..32.min(l.len())]);
                lease_bytes
            }),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            method: value.method.into(),
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
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

impl From<RustAppDeleteMethodCallParams> for AppDeleteMethodCallParams {
    fn from(value: RustAppDeleteMethodCallParams) -> Self {
        AppDeleteMethodCallParams {
            sender: value.sender.to_string(),
            signer: value.signer.map(|s| {
                Arc::new(FfiTransactionSignerFromRust { rust_signer: s })
                    as Arc<dyn TransactionSigner>
            }),
            rekey_to: value.rekey_to.map(|r| r.to_string()),
            note: value.note,
            lease: value.lease.map(|l| l.to_vec()),
            static_fee: value.static_fee,
            extra_fee: value.extra_fee,
            max_fee: value.max_fee,
            validity_window: value.validity_window,
            first_valid_round: value.first_valid_round,
            last_valid_round: value.last_valid_round,
            app_id: value.app_id,
            method: value.method.into(),
            args: value.args.into_iter().map(|arg| arg.into()).collect(),
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
