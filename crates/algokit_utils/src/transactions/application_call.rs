use super::common::CommonParams;
use algokit_transact::{Address, BoxReference, OnApplicationComplete, StateSchema};

/// Parameters for basic application call transactions (non-ABI).
#[derive(Debug, Clone)]
pub struct AppCallParams {
    pub common_params: CommonParams,
    /// ID of the application being called.
    pub app_id: u64,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Arguments passed to the application.
    pub args: Option<Vec<Vec<u8>>>,
    /// List of accounts that may be accessed from the application logic.
    pub account_references: Option<Vec<Address>>,
    /// List of applications that may be accessed from the application logic.
    pub app_references: Option<Vec<u64>>,
    /// List of assets that may be accessed from the application logic.
    pub asset_references: Option<Vec<u64>>,
    /// List of boxes that may be accessed from the application logic.
    pub box_references: Option<Vec<BoxReference>>,
}

/// Parameters for application creation transactions (non-ABI).
#[derive(Debug, Clone)]
pub struct AppCreateParams {
    pub common_params: CommonParams,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Logic executed for every application call transaction, except when
    /// on-completion is set to "clear".
    pub approval_program: Vec<u8>,
    /// Logic executed for application call transactions with on-completion set to "clear".
    pub clear_state_program: Vec<u8>,
    /// Holds the maximum number of global state values.
    pub global_state_schema: Option<StateSchema>,
    /// Holds the maximum number of local state values.
    pub local_state_schema: Option<StateSchema>,
    /// Number of additional pages allocated to the application's approval
    /// and clear state programs.
    pub extra_program_pages: Option<u64>,
    /// Arguments passed to the application.
    pub args: Option<Vec<Vec<u8>>>,
    /// List of accounts that may be accessed from the application logic.
    pub account_references: Option<Vec<Address>>,
    /// List of applications that may be accessed from the application logic.
    pub app_references: Option<Vec<u64>>,
    /// List of assets that may be accessed from the application logic.
    pub asset_references: Option<Vec<u64>>,
    /// List of boxes that may be accessed from the application logic.
    pub box_references: Option<Vec<BoxReference>>,
}

/// Parameters for application update transactions (non-ABI).
#[derive(Debug, Clone)]
pub struct AppUpdateParams {
    pub common_params: CommonParams,
    /// ID of the application being updated.
    pub app_id: u64,
    /// Logic executed for every application call transaction, except when
    /// on-completion is set to "clear".
    pub approval_program: Vec<u8>,
    /// Logic executed for application call transactions with on-completion set to "clear".
    pub clear_state_program: Vec<u8>,
    /// Arguments passed to the application.
    pub args: Option<Vec<Vec<u8>>>,
    /// List of accounts that may be accessed from the application logic.
    pub account_references: Option<Vec<Address>>,
    /// List of applications that may be accessed from the application logic.
    pub app_references: Option<Vec<u64>>,
    /// List of assets that may be accessed from the application logic.
    pub asset_references: Option<Vec<u64>>,
    /// List of boxes that may be accessed from the application logic.
    pub box_references: Option<Vec<BoxReference>>,
}

/// Parameters for application deletion transactions (non-ABI).
#[derive(Debug, Clone)]
pub struct AppDeleteParams {
    pub common_params: CommonParams,
    /// ID of the application being deleted.
    pub app_id: u64,
    /// Arguments passed to the application.
    pub args: Option<Vec<Vec<u8>>>,
    /// List of accounts that may be accessed from the application logic.
    pub account_references: Option<Vec<Address>>,
    /// List of applications that may be accessed from the application logic.
    pub app_references: Option<Vec<u64>>,
    /// List of assets that may be accessed from the application logic.
    pub asset_references: Option<Vec<u64>>,
    /// List of boxes that may be accessed from the application logic.
    pub box_references: Option<Vec<BoxReference>>,
}

/// Parameters for ABI method call transactions.
#[derive(Debug, Clone)]
pub struct AppCallMethodCallParams {
    pub common_params: CommonParams,
    /// ID of the application being called.
    pub app_id: u64,
    /// The ABI method to call.
    pub method: algokit_abi::ABIMethod,
    /// Arguments to pass to the method.
    pub args: Vec<algokit_abi::ABIValue>,
    /// List of accounts that may be accessed from the application logic.
    pub account_references: Option<Vec<Address>>,
    /// List of applications that may be accessed from the application logic.
    pub app_references: Option<Vec<u64>>,
    /// List of assets that may be accessed from the application logic.
    pub asset_references: Option<Vec<u64>>,
    /// List of boxes that may be accessed from the application logic.
    pub box_references: Option<Vec<BoxReference>>,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Optional note field for the transaction.
    pub note: Option<Vec<u8>>,
}

/// Parameters for application create ABI method call transactions.
#[derive(Debug, Clone)]
pub struct AppCreateMethodCallParams {
    pub common_params: CommonParams,
    /// ID of the application being called (should be 0 for creation).
    pub app_id: u64,
    /// The ABI method to call.
    pub method: algokit_abi::ABIMethod,
    /// Arguments to pass to the method.
    pub args: Vec<algokit_abi::ABIValue>,
    /// List of accounts that may be accessed from the application logic.
    pub account_references: Option<Vec<Address>>,
    /// List of applications that may be accessed from the application logic.
    pub app_references: Option<Vec<u64>>,
    /// List of assets that may be accessed from the application logic.
    pub asset_references: Option<Vec<u64>>,
    /// List of boxes that may be accessed from the application logic.
    pub box_references: Option<Vec<BoxReference>>,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Optional note field for the transaction.
    pub note: Option<Vec<u8>>,
    /// Logic executed for every application call transaction, except when
    /// on-completion is set to "clear".
    ///
    /// Approval programs may reject the transaction.
    /// Only required for application creation and update transactions.
    pub approval_program: Vec<u8>,
    /// Logic executed for application call transactions with on-completion set to "clear".
    ///
    /// Clear state programs cannot reject the transaction.
    /// Only required for application creation and update transactions.
    pub clear_state_program: Vec<u8>,
    /// Holds the maximum number of global state values.
    ///
    /// Only required for application creation transactions.
    /// This cannot be changed after creation.
    pub global_state_schema: Option<StateSchema>,
    /// Holds the maximum number of local state values.
    ///
    /// Only required for application creation transactions.
    /// This cannot be changed after creation.
    pub local_state_schema: Option<StateSchema>,
    /// Number of additional pages allocated to the application's approval
    /// and clear state programs.
    ///
    /// Each extra program page is 2048 bytes. The sum of approval program
    /// and clear state program may not exceed 2048*(1+extra_program_pages) bytes.
    /// Currently, the maximum value is 3.
    /// This cannot be changed after creation.
    pub extra_program_pages: Option<u64>,
}

/// Parameters for application update ABI method call transactions.
#[derive(Debug, Clone)]
pub struct AppUpdateMethodCallParams {
    pub common_params: CommonParams,
    /// ID of the application being updated.
    pub app_id: u64,
    /// The ABI method to call.
    pub method: algokit_abi::ABIMethod,
    /// Arguments to pass to the method.
    pub args: Vec<algokit_abi::ABIValue>,
    /// List of accounts that may be accessed from the application logic.
    pub account_references: Option<Vec<Address>>,
    /// List of applications that may be accessed from the application logic.
    pub app_references: Option<Vec<u64>>,
    /// List of assets that may be accessed from the application logic.
    pub asset_references: Option<Vec<u64>>,
    /// List of boxes that may be accessed from the application logic.
    pub box_references: Option<Vec<BoxReference>>,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Optional note field for the transaction.
    pub note: Option<Vec<u8>>,
    /// Logic executed for every application call transaction, except when
    /// on-completion is set to "clear".
    ///
    /// Approval programs may reject the transaction.
    /// Only required for application creation and update transactions.
    pub approval_program: Vec<u8>,
    /// Logic executed for application call transactions with on-completion set to "clear".
    ///
    /// Clear state programs cannot reject the transaction.
    /// Only required for application creation and update transactions.
    pub clear_state_program: Vec<u8>,
}

/// Parameters for application delete ABI method call transactions.
#[derive(Debug, Clone)]
pub struct AppDeleteMethodCallParams {
    pub common_params: CommonParams,
    /// ID of the application being called.
    pub app_id: u64,
    /// The ABI method to call.
    pub method: algokit_abi::ABIMethod,
    /// Arguments to pass to the method.
    pub args: Vec<algokit_abi::ABIValue>,
    /// List of accounts that may be accessed from the application logic.
    pub account_references: Option<Vec<Address>>,
    /// List of applications that may be accessed from the application logic.
    pub app_references: Option<Vec<u64>>,
    /// List of assets that may be accessed from the application logic.
    pub asset_references: Option<Vec<u64>>,
    /// List of boxes that may be accessed from the application logic.
    pub box_references: Option<Vec<BoxReference>>,
    /// Defines what additional actions occur with the transaction.
    pub on_complete: OnApplicationComplete,
    /// Optional note field for the transaction.
    pub note: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub enum MethodCallParams {
    AppCall(AppCallMethodCallParams),
    AppCreate(AppCreateMethodCallParams),
    AppUpdate(AppUpdateMethodCallParams),
    AppDelete(AppDeleteMethodCallParams),
}
