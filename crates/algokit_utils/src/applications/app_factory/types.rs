use std::collections::HashMap;
use std::sync::Arc;

use algokit_abi::Arc56Contract;

use crate::AlgorandClient;
use crate::AppSourceMaps;
use crate::clients::app_manager::TealTemplateValue;
use crate::transactions::common::TransactionSigner;

pub struct AppFactoryParams {
    pub algorand: std::sync::Arc<AlgorandClient>,
    pub app_spec: Arc56Contract,
    pub app_name: Option<String>,
    pub default_sender: Option<String>,
    pub default_signer: Option<Arc<dyn TransactionSigner>>,
    pub version: Option<String>,
    pub deploy_time_params: Option<HashMap<String, TealTemplateValue>>,
    pub updatable: Option<bool>,
    pub deletable: Option<bool>,
    pub source_maps: Option<AppSourceMaps>,
}

#[derive(Debug, Clone, Default)]
pub struct AppFactoryCreateParams {
    pub on_complete: Option<algokit_transact::OnApplicationComplete>,
    pub args: Option<Vec<Vec<u8>>>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    pub global_state_schema: Option<algokit_transact::StateSchema>,
    pub local_state_schema: Option<algokit_transact::StateSchema>,
    pub extra_program_pages: Option<u32>,
    pub sender: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct AppFactoryCreateMethodCallParams {
    pub method: String,
    pub args: Option<Vec<crate::transactions::AppMethodCallArg>>, // raw args accepted; processing later
    pub on_complete: Option<algokit_transact::OnApplicationComplete>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    pub global_state_schema: Option<algokit_transact::StateSchema>,
    pub local_state_schema: Option<algokit_transact::StateSchema>,
    pub extra_program_pages: Option<u32>,
    pub sender: Option<String>,
}

pub type AppFactoryCreateMethodCallResult =
    crate::transactions::sender_results::SendAppCreateResult;

// Factory-specific type aliases to sender results (if needed later)
pub type SendAppCreateFactoryTransactionResult =
    crate::transactions::sender_results::SendAppCreateResult;
pub type SendAppUpdateFactoryTransactionResult =
    crate::transactions::sender_results::SendAppUpdateResult;

#[derive(Debug, Clone, Default)]
pub struct AppFactoryUpdateMethodCallParams {
    pub app_id: u64,
    pub method: String,
    pub args: Option<Vec<crate::transactions::AppMethodCallArg>>, // raw args accepted; processing later
    pub sender: Option<String>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
}

#[derive(Debug, Clone, Default)]
pub struct AppFactoryUpdateParams {
    pub app_id: u64,
    pub args: Option<Vec<Vec<u8>>>,
    pub sender: Option<String>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
}

#[derive(Debug, Clone, Default)]
pub struct AppFactoryDeleteMethodCallParams {
    pub app_id: u64,
    pub method: String,
    pub args: Option<Vec<crate::transactions::AppMethodCallArg>>, // raw args accepted; processing later
    pub sender: Option<String>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
}

#[derive(Debug, Clone, Default)]
pub struct AppFactoryDeleteParams {
    pub app_id: u64,
    pub args: Option<Vec<Vec<u8>>>,
    pub sender: Option<String>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
}

#[derive(Debug)]
pub struct AppFactoryDeployResult {
    pub app: crate::applications::app_deployer::AppMetadata,
    pub operation_performed: crate::applications::app_deployer::AppDeployResult,
    pub create_result: Option<crate::transactions::sender_results::SendAppCreateResult>,
    pub update_result: Option<crate::transactions::sender_results::SendAppUpdateResult>,
    pub delete_result: Option<crate::transactions::sender_results::SendAppCallResult>,
}
