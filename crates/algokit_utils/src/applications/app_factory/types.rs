use crate::AlgorandClient;
use crate::AppSourceMaps;
use crate::applications::AppMetadata;
use crate::clients::app_manager::TealTemplateValue;
use crate::transactions::{AppMethodCallArg, TransactionComposerConfig, TransactionSigner};
use algod_client::models::PendingTransactionResponse;
use algokit_abi::{ABIReturn, Arc56Contract};
use algokit_transact::Byte32;
use algokit_transact::{Address, Transaction};
use std::collections::HashMap;
use std::sync::Arc;

/// Result from sending an app create call via AppFactory.
#[derive(Clone, Debug)]
pub struct AppFactoryCreateResult {
    /// The create transaction
    pub transaction: Transaction,
    /// The response from sending and waiting for the create transaction
    pub confirmation: PendingTransactionResponse,
    /// The create transaction ID
    pub transaction_id: String,
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
}

/// Result from sending an app create method call via AppFactory.
#[derive(Clone, Debug)]
pub struct AppFactoryCreateMethodCallResult {
    /// The create transaction
    pub transaction: Transaction,
    /// The response from sending and waiting for the create transaction
    pub confirmation: PendingTransactionResponse,
    /// The create transaction ID
    pub transaction_id: String,
    /// The ABI return value of the create
    pub abi_return: Option<ABIReturn>,
    /// The group ID for the transaction group (if any)
    pub group: Option<Byte32>,
    /// All transaction IDs in the group
    pub transaction_ids: Vec<String>,
    /// All transactions in the group
    pub transactions: Vec<Transaction>,
    /// All confirmations in the group
    pub confirmations: Vec<PendingTransactionResponse>,
    // TODO: don't need this, it's in the app metadata
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

/// Result from sending an app update method call via AppFactory.
#[derive(Clone, Debug)]
pub struct AppFactoryUpdateMethodCallResult {
    /// The update transaction
    pub transaction: Transaction,
    /// The response from sending and waiting for the update transaction
    pub confirmation: PendingTransactionResponse,
    /// The update transaction ID
    pub transaction_id: String,
    /// The ABI return value of the update transaction
    pub abi_return: Option<ABIReturn>,
    /// The group ID for the transaction group (if any)
    pub group: Option<Byte32>,
    /// All transaction IDs in the group
    pub transaction_ids: Vec<String>,
    /// All transactions in the group
    pub transactions: Vec<Transaction>,
    /// All confirmations in the group
    pub confirmations: Vec<PendingTransactionResponse>,
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

/// Result from replacing an app with method calls via AppFactory.
#[derive(Clone, Debug)]
pub struct AppFactoryReplaceMethodCallResult {
    /// The create transaction
    pub create_transaction: Transaction,
    /// The response from sending and waiting for the create transaction
    pub create_confirmation: PendingTransactionResponse,
    /// The create transaction ID
    pub create_transaction_id: String,
    /// The ABI return value of the create transaction
    pub create_abi_return: Option<ABIReturn>,
    /// The delete transaction
    pub delete_transaction: Transaction,
    /// The response from sending and waiting for the delete transaction
    pub delete_confirmation: PendingTransactionResponse,
    /// The delete transaction ID
    pub delete_transaction_id: String,
    /// The ABI return value of the delete transaction
    pub delete_abi_return: Option<ABIReturn>,
    /// The group ID for the transaction group (if any)
    pub group: Option<Byte32>,
    /// All transaction IDs in the group
    pub transaction_ids: Vec<String>,
    /// All transactions in the group
    pub transactions: Vec<Transaction>,
    /// All confirmations in the group
    pub confirmations: Vec<PendingTransactionResponse>,
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

// TODO: delete these ctors
impl AppFactoryCreateResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction: Transaction,
        confirmation: PendingTransactionResponse,
        transaction_id: String,
        app_id: u64,
        app_address: Address,
        compiled_approval: Option<Vec<u8>>,
        compiled_clear: Option<Vec<u8>>,
        approval_source_map: Option<serde_json::Value>,
        clear_source_map: Option<serde_json::Value>,
    ) -> Self {
        Self {
            transaction,
            confirmation,
            transaction_id,
            app_id,
            app_address,
            compiled_approval,
            compiled_clear,
            approval_source_map,
            clear_source_map,
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

impl AppFactoryUpdateMethodCallResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction: Transaction,
        confirmation: PendingTransactionResponse,
        transaction_id: String,
        group: Option<Byte32>,
        transaction_ids: Vec<String>,
        transactions: Vec<Transaction>,
        confirmations: Vec<PendingTransactionResponse>,
        compiled_approval: Option<Vec<u8>>,
        compiled_clear: Option<Vec<u8>>,
        approval_source_map: Option<serde_json::Value>,
        clear_source_map: Option<serde_json::Value>,
        abi_return: Option<ABIReturn>,
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

/// The result of an app deployment operation
#[derive(Debug)]
pub enum AppFactoryDeployResult {
    /// Application was created
    Create {
        app: AppMetadata,
        result: AppFactoryCreateMethodCallResult,
    },
    /// Application was updated
    Update {
        app: AppMetadata,
        result: AppFactoryUpdateMethodCallResult,
    },
    /// Application was replaced (deleted and recreated)
    Replace {
        app: AppMetadata,
        result: AppFactoryReplaceMethodCallResult,
    },
    /// No operation was performed
    Nothing { app: AppMetadata },
}
