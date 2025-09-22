use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use algokit_abi::{ABIValue, Arc56Contract};

use crate::AlgorandClient;
use crate::AppSourceMaps;
use crate::clients::app_manager::TealTemplateValue;
use crate::transactions::{
    AppMethodCallArg, TransactionComposerConfig, TransactionSigner,
    sender_results::{SendAppCallResult, SendAppCreateResult, SendAppUpdateResult},
};

#[derive(Clone, Debug)]
pub struct AppFactoryCompilationResult {
    pub approval_program: Vec<u8>,
    pub clear_state_program: Vec<u8>,
    pub compiled_approval: crate::clients::app_manager::CompiledTeal,
    pub compiled_clear: crate::clients::app_manager::CompiledTeal,
}

#[derive(Clone, Debug)]
pub struct AppFactoryMethodCallResult<T> {
    inner: T,
    arc56_return: Option<ABIValue>,
}

impl<T> AppFactoryMethodCallResult<T> {
    pub fn new(inner: T, arc56_return: Option<ABIValue>) -> Self {
        Self {
            inner,
            arc56_return,
        }
    }

    pub fn arc56_return(&self) -> Option<&ABIValue> {
        self.arc56_return.as_ref()
    }

    pub fn into_parts(self) -> (T, Option<ABIValue>) {
        (self.inner, self.arc56_return)
    }
}

impl<T> Deref for AppFactoryMethodCallResult<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for AppFactoryMethodCallResult<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
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

pub type AppFactoryCreateMethodCallResult = AppFactoryMethodCallResult<SendAppCreateResult>;
pub type AppFactoryUpdateMethodCallResult = AppFactoryMethodCallResult<SendAppUpdateResult>;
pub type AppFactoryDeleteMethodCallResult = AppFactoryMethodCallResult<SendAppCallResult>;

#[derive(Clone, Default)]
pub struct AppFactoryUpdateMethodCallParams {
    pub app_id: u64,
    pub method: String,
    pub args: Option<Vec<AppMethodCallArg>>, // Accept ARC-56 literal arguments; merge step normalises before execution
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
pub struct AppFactoryUpdateParams {
    pub app_id: u64,
    pub args: Option<Vec<Vec<u8>>>,
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

#[derive(Clone, Default)]
pub struct AppFactoryDeleteParams {
    pub app_id: u64,
    pub args: Option<Vec<Vec<u8>>>,
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

#[derive(Debug)]
pub struct AppFactoryDeployResult {
    pub app: crate::applications::app_deployer::AppMetadata,
    pub operation_performed: crate::applications::app_deployer::AppDeployResult,
    pub create_result: Option<AppFactoryCreateMethodCallResult>,
    pub update_result: Option<AppFactoryUpdateMethodCallResult>,
    pub delete_result: Option<AppFactoryDeleteMethodCallResult>,
}
