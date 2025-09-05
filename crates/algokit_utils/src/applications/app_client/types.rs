use crate::AlgorandClient;
use crate::clients::app_manager::TealTemplateValue;
use crate::transactions::app_call::AppMethodCallArg;
use algokit_abi::Arc56Contract;
use algokit_transact::BoxReference;
use std::collections::HashMap;

/// Container for source maps captured during compilation/simulation.
#[derive(Debug, Clone, Default)]
pub struct AppSourceMaps {
    pub approval_source_map: Option<serde_json::Value>,
    pub clear_source_map: Option<serde_json::Value>,
}

/// Parameters required to construct an AppClient instance.
// Important: do NOT derive Clone for this struct while it contains `AlgorandClient`.
// `AlgorandClient` is intentionally non-Clone: it owns live HTTP clients, internal caches,
// and shared mutable state (e.g., signer registry via Arc<Mutex<_>>). Forcing Clone here
// would either require making `AlgorandClient` Clone or wrapping it in Arc implicitly,
// which encourages accidental copying of a process-wide client and confusing ownership/
// lifetime semantics. If you need to share the client, wrap it in Arc at the call site
// and pass that explicitly, rather than deriving Clone on this params type.
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
// See note above on not deriving Clone while this contains `AlgorandClient`.
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
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
    pub close_remainder_to: Option<String>,
}

/// Parameters for ABI method call operations
#[derive(Debug, Clone, Default)]
pub struct AppClientMethodCallParams {
    pub method: String,
    pub args: Vec<AppMethodCallArg>, // I think this should be Vec<AppMethodCallArg> because the user must use AppMethodCallArg.DefaultValue
    pub sender: Option<String>,
    pub rekey_to: Option<String>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
    pub account_references: Option<Vec<String>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<BoxReference>>,
    // removed the on_complete because we overwrite it anyway later on
    // don't feel right when the user has to specify "None" for a value which will be overwritten
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
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
    pub account_references: Option<Vec<String>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<BoxReference>>,
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
