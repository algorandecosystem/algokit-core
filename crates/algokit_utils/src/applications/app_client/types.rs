use crate::AlgorandClient;
use crate::clients::app_manager::TealTemplateValue;
use crate::transactions::app_call::AppMethodCallArg;
use algokit_abi::Arc56Contract;
use algokit_transact::{BoxReference, OnApplicationComplete};
use std::collections::HashMap;

/// Container for source maps captured during compilation/simulation.
#[derive(Clone)]
pub struct AppSourceMaps {
    pub approval_source_map: Option<serde_json::Value>,
    pub clear_source_map: Option<serde_json::Value>,
}

/// Parameters required to construct an AppClient instance.
// Note: Do not derive Clone for AlgorandClient field
pub struct AppClientParams {
    pub app_id: Option<u64>,
    pub app_spec: Arc56Contract,
    pub algorand: AlgorandClient,
    pub app_name: Option<String>,
    pub default_sender: Option<String>,
    pub source_maps: Option<AppSourceMaps>,
}

/// Parameters for constructing an AppClient from a JSON app spec.
/// The JSON must be a valid ARC-56 contract specification string.
pub struct AppClientJsonParams<'a> {
    pub app_id: Option<u64>,
    pub app_spec_json: &'a str,
    pub algorand: AlgorandClient,
    pub app_name: Option<String>,
    pub default_sender: Option<String>,
    pub source_maps: Option<AppSourceMaps>,
}

/// Parameters for funding an application's account.
#[derive(Debug, Clone, Default)]
pub struct FundAppAccountParams {
    pub amount: u64,
    pub sender: Option<String>,
    pub rekey_to: Option<String>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u64>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
    pub close_remainder_to: Option<String>,
}

/// Parameters for ABI method call operations
#[derive(Debug, Clone, Default)]
pub struct AppClientMethodCallParams {
    pub method: String,
    pub args: Option<Vec<AppMethodCallArg>>,
    pub sender: Option<String>,
    pub rekey_to: Option<String>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u64>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
    pub account_references: Option<Vec<String>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<BoxReference>>,
    pub on_complete: Option<OnApplicationComplete>,
}

/// Parameters for bare (non-ABI) app call operations
#[derive(Debug, Clone, Default)]
pub struct AppClientBareCallParams {
    pub args: Option<Vec<Vec<u8>>>,
    pub sender: Option<String>,
    pub rekey_to: Option<String>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u64>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
    pub account_references: Option<Vec<String>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<BoxReference>>,
    pub on_complete: Option<OnApplicationComplete>,
}

/// Enriched logic error details with source map information.
#[derive(Debug, Clone, Default)]
pub struct LogicError {
    pub logic_error_str: String,
    pub program: Option<Vec<u8>>,
    pub source_map: Option<serde_json::Value>,
    pub transaction_id: Option<String>,
    pub pc: Option<u64>,
    pub line_no: Option<u64>,
    pub lines: Option<Vec<String>>,
    pub traces: Option<Vec<serde_json::Value>>,
}

/// Compilation configuration for update/compile flows
#[derive(Debug, Clone, Default)]
pub struct CompilationParams {
    pub deploy_time_params: Option<HashMap<String, TealTemplateValue>>,
    pub updatable: Option<bool>,
    pub deletable: Option<bool>,
}
