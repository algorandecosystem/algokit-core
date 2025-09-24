use crate::AlgorandClient;
use crate::AppSourceMaps;
use crate::clients::app_manager::TealTemplateValue;
use crate::transactions::{AppMethodCallArg, TransactionComposerConfig, TransactionSigner};
use algod_client::models::PendingTransactionResponse;
use algokit_abi::{ABIReturn, Arc56Contract};
use algokit_transact::Byte32;
use algokit_transact::{Address, Transaction};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppFactoryCompilationResult {
    pub approval_program: Vec<u8>,
    pub clear_state_program: Vec<u8>,
    pub compiled_approval: crate::clients::app_manager::CompiledTeal,
    pub compiled_clear: crate::clients::app_manager::CompiledTeal,
}

/// Result from sending an app create call via AppFactory.
#[derive(Clone, Debug)]
pub struct AppFactoryCreateResult {
    /// The transaction that has been sent
    pub transaction: Transaction,
    /// The response from sending and waiting for the transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID that has been sent
    pub transaction_id: String,
    /// The ID of the created app
    pub app_id: u64,
    /// The address of the created app
    pub app_address: Address,
}

/// Result from sending an app create method call via AppFactory.
/// Contains transaction details, confirmation, and parsed ARC-56 return value.
#[derive(Clone, Debug)]
pub struct AppFactoryCreateMethodCallResult {
    /// The transaction that has been sent
    pub transaction: Transaction,
    /// The response from sending and waiting for the transaction
    pub confirmation: PendingTransactionResponse,
    /// The transaction ID that has been sent
    pub transaction_id: String,
    /// The group ID for the transaction group (if any)
    pub group: Option<Byte32>,
    /// The ABI return value
    pub abi_return: Option<ABIReturn>,
    /// All transaction IDs in the group
    pub transaction_ids: Vec<String>,
    /// All transactions in the group
    pub transactions: Vec<Transaction>,
    /// All confirmations in the group
    pub confirmations: Vec<PendingTransactionResponse>,
    /// The ID of the created app
    pub app_id: u64,
    /// The address of the created app
    pub app_address: Address,
    /// The compiled approval program (if provided)
    pub compiled_approval: Option<Vec<u8>>,
    /// The compiled clear state program (if provided)
    pub compiled_clear: Option<Vec<u8>>,
    /// The approval program source map (if available)
    pub approval_source_map: Option<serde_json::Value>,
    /// The clear program source map (if available)
    pub clear_source_map: Option<serde_json::Value>,
    /// The ABI return value
    pub abi_returns: Vec<ABIReturn>,
}

pub struct AppFactoryParams {
    pub algorand: Arc<AlgorandClient>,
    pub app_spec: Arc56Contract,
    pub app_name: Option<String>,
    pub default_sender: Option<String>,
    pub default_signer: Option<Arc<dyn TransactionSigner>>,
    pub version: Option<String>,
    pub deploy_time_params: Option<HashMap<String, TealTemplateValue>>,
    pub updatable: Option<bool>,
    pub deletable: Option<bool>,
    pub source_maps: Option<AppSourceMaps>,
    pub transaction_composer_config: Option<TransactionComposerConfig>,
}

#[derive(Clone, Default)]
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
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<algokit_transact::Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

#[derive(Clone, Default)]
pub struct AppFactoryCreateMethodCallParams {
    pub method: String,
    pub args: Option<Vec<AppMethodCallArg>>,
    pub on_complete: Option<algokit_transact::OnApplicationComplete>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    pub global_state_schema: Option<algokit_transact::StateSchema>,
    pub local_state_schema: Option<algokit_transact::StateSchema>,
    pub extra_program_pages: Option<u32>,
    pub sender: Option<String>,
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<algokit_transact::Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

impl AppFactoryCreateResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction: Transaction,
        confirmation: PendingTransactionResponse,
        transaction_id: String,
        app_id: u64,
        app_address: Address,
    ) -> Self {
        Self {
            transaction,
            confirmation,
            transaction_id,
            app_id,
            app_address,
        }
    }
}

// Helper methods for creating these results
impl AppFactoryCreateMethodCallResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction: Transaction,
        confirmation: PendingTransactionResponse,
        transaction_id: String,
        group: Option<Byte32>,
        abi_return: Option<ABIReturn>,
        transaction_ids: Vec<String>,
        transactions: Vec<Transaction>,
        confirmations: Vec<PendingTransactionResponse>,
        app_id: u64,
        app_address: Address,
        compiled_approval: Option<Vec<u8>>,
        compiled_clear: Option<Vec<u8>>,
        approval_source_map: Option<serde_json::Value>,
        clear_source_map: Option<serde_json::Value>,
        abi_returns: Vec<ABIReturn>,
    ) -> Self {
        Self {
            transaction,
            confirmation,
            transaction_id,
            group,
            transaction_ids,
            transactions,
            confirmations,
            app_id,
            app_address,
            compiled_approval,
            compiled_clear,
            approval_source_map,
            clear_source_map,
            abi_return,
            abi_returns,
        }
    }
}

#[derive(Clone, Default)]
pub struct AppFactoryUpdateMethodCallParams {
    pub app_id: u64,
    pub method: String,
    pub args: Option<Vec<AppMethodCallArg>>,
    pub sender: Option<String>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<algokit_transact::Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}

#[derive(Clone, Default)]
pub struct AppFactoryDeleteMethodCallParams {
    pub app_id: u64,
    pub method: String,
    pub args: Option<Vec<AppMethodCallArg>>,
    pub sender: Option<String>,
    pub account_references: Option<Vec<algokit_transact::Address>>,
    pub app_references: Option<Vec<u64>>,
    pub asset_references: Option<Vec<u64>>,
    pub box_references: Option<Vec<algokit_transact::BoxReference>>,
    pub signer: Option<Arc<dyn TransactionSigner>>,
    pub rekey_to: Option<algokit_transact::Address>,
    pub note: Option<Vec<u8>>,
    pub lease: Option<[u8; 32]>,
    pub static_fee: Option<u64>,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
    pub validity_window: Option<u32>,
    pub first_valid_round: Option<u64>,
    pub last_valid_round: Option<u64>,
}
