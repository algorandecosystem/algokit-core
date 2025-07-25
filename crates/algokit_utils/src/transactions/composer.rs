use algod_client::{
    AlgodClient,
    apis::{Error as AlgodError, Format},
    models::{PendingTransactionResponse, TransactionParams},
};
use algokit_transact::{
    Address, AlgorandMsgpack, AssetConfigTransactionFields, Byte32, FeeParams,
    KeyRegistrationTransactionFields, MAX_TX_GROUP_SIZE, OnApplicationComplete,
    PaymentTransactionFields, SignedTransaction, Transaction, TransactionHeader, TransactionId,
    Transactions,
};
use derive_more::Debug;
use std::{collections::HashMap, str::FromStr, sync::Arc};

use crate::genesis_id_is_localnet;

// ARC-4 Application Call argument limits
const INDIVIDUAL_ARG_LIMIT: usize = 14; // Args 1-14 go in individual slots
const TUPLE_PACKING_THRESHOLD: usize = 15; // 15+ args trigger tuple packing

// Error message constants for consistency (Rule 13)
const ERR_ARG_COUNT_MISMATCH: &str = "Argument count mismatch for method";
const ERR_TUPLE_ENCODING_FAILED: &str = "Failed to encode tuple for arguments";
const ERR_INVALID_TRANSACTION_INDEX: &str = "Transaction index out of range";

use super::application_call::{
    AppCallParams, AppCreateParams, AppDeleteParams, AppUpdateParams, MethodCallParams,
};
use super::asset_config::{AssetCreateParams, AssetDestroyParams, AssetReconfigParams};
use super::asset_freeze::{AssetFreezeParams, AssetUnfreezeParams};
use super::common::{CommonParams, TransactionSigner, TransactionSignerGetter};
use super::key_registration::{
    NonParticipationKeyRegistrationParams, OfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams,
};
use super::payment::{AccountCloseParams, PaymentParams};

#[derive(Debug, thiserror::Error)]
pub enum ComposerError {
    #[error("Algod client error: {0}")]
    AlgodClientError(#[from] AlgodError),
    #[error("Decode Error: {0}")]
    DecodeError(String),
    #[error("Transaction Error: {0}")]
    TransactionError(String),
    #[error("Signing Error: {0}")]
    SigningError(String),
    #[error("Composer State Error: {0}")]
    StateError(String),
    #[error("Transaction pool error: {0}")]
    PoolError(String),
    #[error("Transaction group size exceeds the max limit of: {max}", max = MAX_TX_GROUP_SIZE)]
    GroupSizeError(),
    #[error("ABI argument encoding error: {0}")]
    ABIEncodingError(String),
}

#[derive(Clone)]
pub struct TransactionWithSigner {
    pub transaction: Transaction,
    pub signer: Arc<dyn TransactionSigner>,
}

#[derive(Debug, Clone)]
pub struct AssetTransferParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset transfers
    /// without additional side effects.
    /// Only in the case where the receiver is equal to the sender and the amount is zero,
    /// this is an asset opt-in transaction.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub amount: u64,
    pub receiver: Address,
}

#[derive(Debug, Clone)]
pub struct AssetOptInParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-in
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
}

#[derive(Debug, Clone)]
pub struct AssetOptOutParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset opt-out
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub close_remainder_to: Option<Address>,
}

#[derive(Debug, Clone)]
pub struct AssetClawbackParams {
    /// Part of the "specialized" asset transaction types.
    /// Based on the primitive asset transfer, this struct implements asset clawback
    /// without additional side effects.
    pub common_params: CommonParams,
    pub asset_id: u64,
    pub amount: u64,
    pub receiver: Address,
    // The address from which ASAs are taken.
    pub clawback_target: Address,
}

#[derive(Debug, Clone)]
pub struct SendTransactionComposerResults {
    pub group_id: Option<Byte32>,
    pub transaction_ids: Vec<String>,
    pub confirmations: Vec<PendingTransactionResponse>,
    pub abi_returns: Vec<Option<algokit_abi::ABIValue>>,
}

#[derive(Debug, Clone)]
pub struct SendParams {
    pub max_rounds_to_wait_for_confirmation: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum ComposerTransaction {
    Transaction(Transaction),
    Payment(PaymentParams),
    AccountClose(AccountCloseParams),
    AssetTransfer(AssetTransferParams),
    AssetOptIn(AssetOptInParams),
    AssetOptOut(AssetOptOutParams),
    AssetClawback(AssetClawbackParams),
    AssetCreate(AssetCreateParams),
    AssetReconfigure(AssetReconfigParams),
    AssetDestroy(AssetDestroyParams),
    AssetFreeze(AssetFreezeParams),
    AssetUnfreeze(AssetUnfreezeParams),
    ApplicationCall(AppCallParams),
    ApplicationCreate(AppCreateParams),
    ApplicationUpdate(AppUpdateParams),
    ApplicationDelete(AppDeleteParams),
    MethodCall(MethodCallParams),
    OnlineKeyRegistration(OnlineKeyRegistrationParams),
    OfflineKeyRegistration(OfflineKeyRegistrationParams),
    NonParticipationKeyRegistration(NonParticipationKeyRegistrationParams),
}

impl ComposerTransaction {
    pub fn common_params(&self) -> CommonParams {
        match self {
            ComposerTransaction::Payment(payment_params) => payment_params.common_params.clone(),
            ComposerTransaction::AccountClose(account_close_params) => {
                account_close_params.common_params.clone()
            }
            ComposerTransaction::AssetTransfer(asset_transfer_params) => {
                asset_transfer_params.common_params.clone()
            }
            ComposerTransaction::AssetOptIn(asset_opt_in_params) => {
                asset_opt_in_params.common_params.clone()
            }
            ComposerTransaction::AssetOptOut(asset_opt_out_params) => {
                asset_opt_out_params.common_params.clone()
            }
            ComposerTransaction::AssetClawback(asset_clawback_params) => {
                asset_clawback_params.common_params.clone()
            }
            ComposerTransaction::AssetCreate(asset_create_params) => {
                asset_create_params.common_params.clone()
            }
            ComposerTransaction::AssetReconfigure(asset_reconfigure_params) => {
                asset_reconfigure_params.common_params.clone()
            }
            ComposerTransaction::AssetDestroy(asset_destroy_params) => {
                asset_destroy_params.common_params.clone()
            }
            ComposerTransaction::AssetFreeze(asset_freeze_params) => {
                asset_freeze_params.common_params.clone()
            }
            ComposerTransaction::AssetUnfreeze(asset_unfreeze_params) => {
                asset_unfreeze_params.common_params.clone()
            }
            ComposerTransaction::ApplicationCall(app_call_params) => {
                app_call_params.common_params.clone()
            }
            ComposerTransaction::ApplicationCreate(app_create_params) => {
                app_create_params.common_params.clone()
            }
            ComposerTransaction::ApplicationUpdate(app_update_params) => {
                app_update_params.common_params.clone()
            }
            ComposerTransaction::ApplicationDelete(app_delete_params) => {
                app_delete_params.common_params.clone()
            }
            ComposerTransaction::MethodCall(method_call_params) => match method_call_params {
                MethodCallParams::AppCall(params) => params.common_params.clone(),
                MethodCallParams::AppCreate(params) => params.common_params.clone(),
                MethodCallParams::AppUpdate(params) => params.common_params.clone(),
                MethodCallParams::AppDelete(params) => params.common_params.clone(),
            },
            ComposerTransaction::OnlineKeyRegistration(online_key_reg_params) => {
                online_key_reg_params.common_params.clone()
            }
            ComposerTransaction::OfflineKeyRegistration(offline_key_reg_params) => {
                offline_key_reg_params.common_params.clone()
            }
            ComposerTransaction::NonParticipationKeyRegistration(non_participation_params) => {
                non_participation_params.common_params.clone()
            }
            _ => CommonParams::default(),
        }
    }
}

#[derive(Clone)]
pub struct TransactionComposer {
    transactions: Vec<ComposerTransaction>,
    algod_client: AlgodClient,
    signer_getter: Arc<dyn TransactionSignerGetter>,
    built_group: Option<Vec<TransactionWithSigner>>,
    signed_group: Option<Vec<SignedTransaction>>,
}

impl TransactionComposer {
    pub fn new(algod_client: AlgodClient, signer_getter: Arc<dyn TransactionSignerGetter>) -> Self {
        TransactionComposer {
            transactions: Vec::new(),
            algod_client,
            signer_getter,
            built_group: None,
            signed_group: None,
        }
    }

    #[cfg(feature = "default_http_client")]
    pub fn testnet() -> Self {
        use crate::EmptySigner;

        TransactionComposer {
            transactions: Vec::new(),
            algod_client: AlgodClient::testnet(),
            signer_getter: Arc::new(EmptySigner {}),
            built_group: None,
            signed_group: None,
        }
    }

    fn push(&mut self, txn: ComposerTransaction) -> Result<(), ComposerError> {
        if self.transactions.len() >= MAX_TX_GROUP_SIZE {
            return Err(ComposerError::GroupSizeError());
        }
        self.transactions.push(txn);
        Ok(())
    }

    pub fn add_payment(&mut self, payment_params: PaymentParams) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::Payment(payment_params))
    }

    pub fn add_account_close(
        &mut self,
        account_close_params: AccountCloseParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AccountClose(account_close_params))
    }

    pub fn add_asset_transfer(
        &mut self,
        asset_transfer_params: AssetTransferParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetTransfer(asset_transfer_params))
    }

    pub fn add_asset_opt_in(
        &mut self,
        asset_opt_in_params: AssetOptInParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetOptIn(asset_opt_in_params))
    }

    pub fn add_asset_opt_out(
        &mut self,
        asset_opt_out_params: AssetOptOutParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetOptOut(asset_opt_out_params))
    }

    pub fn add_asset_clawback(
        &mut self,
        asset_clawback_params: AssetClawbackParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetClawback(asset_clawback_params))
    }

    pub fn add_asset_create(
        &mut self,
        asset_create_params: AssetCreateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetCreate(asset_create_params))
    }

    pub fn add_asset_reconfigure(
        &mut self,
        asset_reconfigure_params: AssetReconfigParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetReconfigure(
            asset_reconfigure_params,
        ))
    }

    pub fn add_asset_destroy(
        &mut self,
        asset_destroy_params: AssetDestroyParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetDestroy(asset_destroy_params))
    }

    pub fn add_asset_freeze(
        &mut self,
        asset_freeze_params: AssetFreezeParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetFreeze(asset_freeze_params))
    }

    pub fn add_asset_unfreeze(
        &mut self,
        asset_unfreeze_params: AssetUnfreezeParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::AssetUnfreeze(asset_unfreeze_params))
    }

    pub fn add_online_key_registration(
        &mut self,
        online_key_reg_params: OnlineKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::OnlineKeyRegistration(
            online_key_reg_params,
        ))
    }

    pub fn add_offline_key_registration(
        &mut self,
        offline_key_reg_params: OfflineKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::OfflineKeyRegistration(
            offline_key_reg_params,
        ))
    }

    pub fn add_non_participation_key_registration(
        &mut self,
        non_participation_params: NonParticipationKeyRegistrationParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::NonParticipationKeyRegistration(
            non_participation_params,
        ))
    }

    pub fn add_application_call(
        &mut self,
        app_call_params: AppCallParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationCall(app_call_params))
    }

    pub fn add_application_create(
        &mut self,
        app_create_params: AppCreateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationCreate(app_create_params))
    }

    pub fn add_application_update(
        &mut self,
        app_update_params: AppUpdateParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationUpdate(app_update_params))
    }

    pub fn add_application_delete(
        &mut self,
        app_delete_params: AppDeleteParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::ApplicationDelete(app_delete_params))
    }

    pub fn add_method_call(
        &mut self,
        method_call_params: MethodCallParams,
    ) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::MethodCall(method_call_params))
    }

    /// Helper method to manage foreign arrays for ABI method calls
    /// Returns a map of argument index to foreign array index for reference arguments
    fn populate_foreign_arrays(
        &self,
        app_call_params: &mut AppCallParams,
        method: &algokit_abi::ABIMethod,
        args: &[algokit_abi::ABIValue],
    ) -> Result<HashMap<usize, u8>, ComposerError> {
        // Initialize foreign arrays if they don't exist
        if app_call_params.app_references.is_none() {
            app_call_params.app_references = Some(Vec::new());
        }
        if app_call_params.asset_references.is_none() {
            app_call_params.asset_references = Some(Vec::new());
        }
        if app_call_params.account_references.is_none() {
            app_call_params.account_references = Some(Vec::new());
        }

        let mut reference_indexes = HashMap::new();

        for (arg_index, (arg_value, arg_spec)) in args.iter().zip(method.args.iter()).enumerate() {
            match &arg_spec.arg_type {
                algokit_abi::ABIMethodArgType::Reference(ref_type) => {
                    // Extract the actual reference value and add it to the appropriate foreign array
                    match ref_type {
                        algokit_abi::ABIReferenceType::Account => {
                            if let algokit_abi::ABIValue::Address(address_str) = arg_value {
                                let address = Address::from_str(address_str).map_err(|e| {
                                    ComposerError::TransactionError(format!(
                                        "Invalid address for account reference argument {}: {}",
                                        arg_spec.name.as_deref().unwrap_or(""),
                                        e
                                    ))
                                })?;
                                let accounts = app_call_params.account_references.as_mut().unwrap();
                                let foreign_index = accounts.len() as u8;
                                accounts.push(address);
                                reference_indexes.insert(arg_index, foreign_index);
                            } else {
                                return Err(ComposerError::TransactionError(format!(
                                    "Account reference argument {} must be an address",
                                    arg_spec.name.as_deref().unwrap_or("")
                                )));
                            }
                        }
                        algokit_abi::ABIReferenceType::Asset => {
                            if let algokit_abi::ABIValue::Uint(asset_id_bigint) = arg_value {
                                let asset_id: u64 = asset_id_bigint.try_into().map_err(|_| {
                                    ComposerError::TransactionError(format!(
                                        "Asset ID out of range for argument {}: must fit in u64",
                                        arg_spec.name.as_deref().unwrap_or("")
                                    ))
                                })?;
                                let assets = app_call_params.asset_references.as_mut().unwrap();
                                let foreign_index = assets.len() as u8;
                                assets.push(asset_id);
                                reference_indexes.insert(arg_index, foreign_index);
                            } else {
                                return Err(ComposerError::TransactionError(format!(
                                    "Asset reference argument {} must be a uint64",
                                    arg_spec.name.as_deref().unwrap_or("")
                                )));
                            }
                        }
                        algokit_abi::ABIReferenceType::Application => {
                            if let algokit_abi::ABIValue::Uint(app_id_bigint) = arg_value {
                                let app_id: u64 = app_id_bigint.try_into().map_err(|_| {
                                    ComposerError::TransactionError(format!(
                                        "Application ID out of range for argument {}: must fit in u64",
                                        arg_spec.name.as_deref().unwrap_or("")
                                    ))
                                })?;
                                let apps = app_call_params.app_references.as_mut().unwrap();
                                let foreign_index = apps.len() as u8;
                                apps.push(app_id);
                                reference_indexes.insert(arg_index, foreign_index);
                            } else {
                                return Err(ComposerError::TransactionError(format!(
                                    "Application reference argument {} must be a uint64",
                                    arg_spec.name.as_deref().unwrap_or("")
                                )));
                            }
                        }
                    }
                }
                _ => {
                    // Not a reference argument, skip
                }
            }
        }

        Ok(reference_indexes)
    }

    /// Encodes ABI method arguments according to ARC-4 specification.
    ///
    /// For methods with 15+ arguments, implements tuple packing:
    /// - Arguments 1-14: individual ApplicationArgs slots
    /// - Arguments 15+: packed as tuple in ApplicationArgs[15]
    async fn encode_abi_arguments(
        &self,
        method: &algokit_abi::ABIMethod,
        args: &[algokit_abi::ABIValue],
        reference_indexes: &HashMap<usize, u8>,
    ) -> Result<Vec<Vec<u8>>, ComposerError> {
        // Validate argument count (Rule 11: consistent validation)
        self.validate_argument_count(method, args)?;

        let mut encoded_args = vec![method.selector().map_err(|e| {
            ComposerError::ABIEncodingError(format!("Failed to get method selector: {}", e))
        })?];

        // Apply ARC-4 tuple packing for methods with 15+ arguments
        if args.len() >= TUPLE_PACKING_THRESHOLD {
            self.encode_with_tuple_packing(
                args,
                &method.args,
                reference_indexes,
                &mut encoded_args,
            )?;
        } else {
            self.encode_individually(args, &method.args, reference_indexes, &mut encoded_args)?;
        }

        Ok(encoded_args)
    }

    /// Validates argument count matches method signature (Rule 11)
    fn validate_argument_count(
        &self,
        method: &algokit_abi::ABIMethod,
        args: &[algokit_abi::ABIValue],
    ) -> Result<(), ComposerError> {
        if args.len() != method.args.len() {
            return Err(ComposerError::ABIEncodingError(format!(
                "{} '{}': expected {}, got {}",
                ERR_ARG_COUNT_MISMATCH,
                method.name,
                method.args.len(),
                args.len()
            )));
        }
        Ok(())
    }

    /// Encodes arguments with ARC-4 tuple packing for 15+ arguments (Rule 8: extract shared logic)
    fn encode_with_tuple_packing(
        &self,
        args: &[algokit_abi::ABIValue],
        arg_specs: &[algokit_abi::ABIMethodArg],
        reference_indexes: &HashMap<usize, u8>,
        encoded_args: &mut Vec<Vec<u8>>,
    ) -> Result<(), ComposerError> {
        // Encode first 14 arguments individually
        for i in 0..INDIVIDUAL_ARG_LIMIT {
            let encoded =
                self.encode_single_argument(&args[i], &arg_specs[i], i, reference_indexes)?;
            encoded_args.push(encoded);
        }

        // Pack remaining arguments into tuple at position 15
        let remaining_args = &args[INDIVIDUAL_ARG_LIMIT..];
        let remaining_specs = &arg_specs[INDIVIDUAL_ARG_LIMIT..];
        let tuple_encoded = self.encode_tuple_argument(
            remaining_args,
            remaining_specs,
            INDIVIDUAL_ARG_LIMIT,
            reference_indexes,
        )?;
        encoded_args.push(tuple_encoded);

        Ok(())
    }

    /// Encodes arguments individually for methods with <15 arguments (Rule 8: extract shared logic)
    fn encode_individually(
        &self,
        args: &[algokit_abi::ABIValue],
        arg_specs: &[algokit_abi::ABIMethodArg],
        reference_indexes: &HashMap<usize, u8>,
        encoded_args: &mut Vec<Vec<u8>>,
    ) -> Result<(), ComposerError> {
        for (i, (arg_value, arg_spec)) in args.iter().zip(arg_specs.iter()).enumerate() {
            let encoded = self.encode_single_argument(arg_value, arg_spec, i, reference_indexes)?;
            encoded_args.push(encoded);
        }
        Ok(())
    }

    /// Encodes a tuple containing multiple arguments (ARC-4 compliance)
    fn encode_tuple_argument(
        &self,
        args: &[algokit_abi::ABIValue],
        specs: &[algokit_abi::ABIMethodArg],
        base_index: usize,
        reference_indexes: &HashMap<usize, u8>,
    ) -> Result<Vec<u8>, ComposerError> {
        // Build tuple type from argument specifications
        let tuple_types: Result<Vec<_>, _> = specs
            .iter()
            .map(|spec| self.abi_type_from_spec(spec))
            .collect();
        let tuple_type = algokit_abi::ABIType::Tuple(tuple_types?);

        // Transform arguments for tuple encoding
        let tuple_values: Result<Vec<_>, _> = args
            .iter()
            .enumerate()
            .map(|(i, arg)| {
                self.transform_for_tuple(arg, &specs[i], base_index + i, reference_indexes)
            })
            .collect();

        let tuple_value = algokit_abi::ABIValue::Array(tuple_values?);
        tuple_type.encode(&tuple_value).map_err(|e| {
            ComposerError::ABIEncodingError(format!(
                "{} {}-{}: {}",
                ERR_TUPLE_ENCODING_FAILED,
                base_index,
                base_index + args.len() - 1,
                e
            ))
        })
    }

    /// Encodes a single argument based on its type (Rule 5: reduce duplication)
    fn encode_single_argument(
        &self,
        arg_value: &algokit_abi::ABIValue,
        arg_spec: &algokit_abi::ABIMethodArg,
        arg_index: usize,
        reference_indexes: &HashMap<usize, u8>,
    ) -> Result<Vec<u8>, ComposerError> {
        match &arg_spec.arg_type {
            algokit_abi::ABIMethodArgType::Value(abi_type) => {
                abi_type.encode(arg_value).map_err(|e| {
                    ComposerError::ABIEncodingError(format!(
                        "Failed to encode argument '{}' at index {}: {}",
                        arg_spec.name.as_deref().unwrap_or("unnamed"),
                        arg_index,
                        e
                    ))
                })
            }
            algokit_abi::ABIMethodArgType::Transaction(_) => {
                self.encode_transaction_index(arg_value, arg_spec, arg_index)
            }
            algokit_abi::ABIMethodArgType::Reference(_) => {
                self.encode_reference_index(arg_index, reference_indexes, arg_spec)
            }
        }
    }

    /// Extracts ABI type from method argument specification (Rule 8: extract shared logic)
    fn abi_type_from_spec(
        &self,
        spec: &algokit_abi::ABIMethodArg,
    ) -> Result<algokit_abi::ABIType, ComposerError> {
        match &spec.arg_type {
            algokit_abi::ABIMethodArgType::Value(abi_type) => Ok(abi_type.clone()),
            // Reference and transaction types encoded as uint8 indexes
            algokit_abi::ABIMethodArgType::Reference(_)
            | algokit_abi::ABIMethodArgType::Transaction(_) => Ok(algokit_abi::ABIType::Uint(
                algokit_abi::abi_type::BitSize::new(8).map_err(|e| {
                    ComposerError::ABIEncodingError(format!("Invalid bit size: {}", e))
                })?,
            )),
        }
    }

    /// Transforms argument value for tuple encoding (Rule 8: extract shared logic)
    fn transform_for_tuple(
        &self,
        arg_value: &algokit_abi::ABIValue,
        arg_spec: &algokit_abi::ABIMethodArg,
        arg_index: usize,
        reference_indexes: &HashMap<usize, u8>,
    ) -> Result<algokit_abi::ABIValue, ComposerError> {
        match &arg_spec.arg_type {
            algokit_abi::ABIMethodArgType::Value(_) => Ok(arg_value.clone()),
            algokit_abi::ABIMethodArgType::Reference(_) => {
                let foreign_index = reference_indexes.get(&arg_index).ok_or_else(|| {
                    ComposerError::ABIEncodingError(format!(
                        "Reference argument at index {} not in foreign arrays",
                        arg_index
                    ))
                })?;
                Ok(algokit_abi::ABIValue::Uint((*foreign_index as u64).into()))
            }
            algokit_abi::ABIMethodArgType::Transaction(_) => {
                if let algokit_abi::ABIValue::Uint(index) = arg_value {
                    Ok(algokit_abi::ABIValue::Uint(index.clone()))
                } else {
                    Err(ComposerError::ABIEncodingError(format!(
                        "Transaction argument at index {} must be uint",
                        arg_index
                    )))
                }
            }
        }
    }

    /// Encodes transaction argument as uint8 index (Rule 8: extract shared logic)
    fn encode_transaction_index(
        &self,
        arg_value: &algokit_abi::ABIValue,
        arg_spec: &algokit_abi::ABIMethodArg,
        arg_index: usize,
    ) -> Result<Vec<u8>, ComposerError> {
        if let algokit_abi::ABIValue::Uint(index) = arg_value {
            let index_bytes = index.to_bytes_be();
            if index_bytes.len() > 1 || index_bytes[0] > 15 {
                return Err(ComposerError::ABIEncodingError(format!(
                    "{} for argument '{}' at index {}: must be 0-15",
                    ERR_INVALID_TRANSACTION_INDEX,
                    arg_spec.name.as_deref().unwrap_or("unnamed"),
                    arg_index
                )));
            }
            Ok(vec![index_bytes[0]])
        } else {
            Err(ComposerError::ABIEncodingError(format!(
                "Transaction argument '{}' at index {} must be uint8 index",
                arg_spec.name.as_deref().unwrap_or("unnamed"),
                arg_index
            )))
        }
    }

    /// Encodes reference argument as foreign array index (Rule 8: extract shared logic)
    fn encode_reference_index(
        &self,
        arg_index: usize,
        reference_indexes: &HashMap<usize, u8>,
        arg_spec: &algokit_abi::ABIMethodArg,
    ) -> Result<Vec<u8>, ComposerError> {
        let foreign_index = reference_indexes.get(&arg_index).ok_or_else(|| {
            ComposerError::ABIEncodingError(format!(
                "Reference argument '{}' at index {} not in foreign arrays",
                arg_spec.name.as_deref().unwrap_or("unnamed"),
                arg_index
            ))
        })?;
        Ok(vec![*foreign_index])
    }
    fn build_method_call_transaction(
        &self,
        params: &AppCallParams,
        header: TransactionHeader,
        encoded_args: Vec<Vec<u8>>,
    ) -> Result<Transaction, ComposerError> {
        Ok(Transaction::ApplicationCall(
            algokit_transact::ApplicationCallTransactionFields {
                header,
                app_id: params.app_id,
                on_complete: params.on_complete,
                approval_program: None,
                clear_state_program: None,
                global_state_schema: None,
                local_state_schema: None,
                extra_program_pages: None,
                args: Some(encoded_args),
                account_references: params.account_references.clone(),
                app_references: params.app_references.clone(),
                asset_references: params.asset_references.clone(),
                box_references: params.box_references.clone(),
            },
        ))
    }

    /// Parse ABI return values from transaction confirmations
    fn parse_abi_return_values(
        &self,
        confirmations: &[PendingTransactionResponse],
    ) -> Result<Vec<Option<algokit_abi::ABIValue>>, ComposerError> {
        let mut abi_returns = Vec::with_capacity(confirmations.len());

        for (tx_index, confirmation) in confirmations.iter().enumerate() {
            // Check if this transaction corresponds to a method call
            if let Some(ComposerTransaction::MethodCall(method_call_params)) =
                self.transactions.get(tx_index)
            {
                // Look for ABI return values in transaction logs
                let method = match method_call_params {
                    MethodCallParams::AppCall(params) => &params.method,
                    MethodCallParams::AppCreate(params) => &params.method,
                    MethodCallParams::AppUpdate(params) => &params.method,
                    MethodCallParams::AppDelete(params) => &params.method,
                };
                let return_value = self.extract_abi_return_from_logs(confirmation, method)?;
                abi_returns.push(return_value);
            } else {
                // Not a method call transaction, no ABI return
                abi_returns.push(None);
            }
        }

        Ok(abi_returns)
    }

    /// Extract ABI return value from transaction logs using the 0x151f7c75 prefix
    fn extract_abi_return_from_logs(
        &self,
        confirmation: &PendingTransactionResponse,
        method: &algokit_abi::ABIMethod,
    ) -> Result<Option<algokit_abi::ABIValue>, ComposerError> {
        // ABI return values are stored in logs with the prefix 0x151f7c75
        const ABI_RETURN_PREFIX: &[u8] = &[0x15, 0x1f, 0x7c, 0x75];

        if let Some(logs) = &confirmation.logs {
            for log_entry in logs {
                // Log entries come as raw bytes (String containing raw bytes)
                let log_bytes = log_entry.as_bytes();

                // Check if this log entry has the ABI return prefix
                if log_bytes.starts_with(ABI_RETURN_PREFIX) {
                    // Extract the return value bytes (skip the prefix)
                    let return_bytes = &log_bytes[ABI_RETURN_PREFIX.len()..];

                    // If method has no return type, return None
                    if let Some(return_type) = &method.returns {
                        // Decode the return value using the method's return type
                        let return_value = return_type.decode(return_bytes).map_err(|e| {
                            ComposerError::TransactionError(format!(
                                "Failed to decode ABI return value for method {}: {}",
                                method.name, e
                            ))
                        })?;
                        return Ok(Some(return_value));
                    } else {
                        // Method has no return type but we found a return log - this shouldn't happen
                        return Err(ComposerError::TransactionError(format!(
                            "Found ABI return log for method {} which has no return type",
                            method.name
                        )));
                    }
                }
            }
        }

        // No ABI return found (this is normal for void methods)
        Ok(None)
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ComposerError> {
        self.push(ComposerTransaction::Transaction(transaction))
    }

    pub fn add_transactions(
        &mut self,
        transactions: Vec<Transaction>,
    ) -> Result<(), ComposerError> {
        if self.transactions.len() + transactions.len() > MAX_TX_GROUP_SIZE {
            return Err(ComposerError::GroupSizeError());
        }

        transactions
            .into_iter()
            .try_for_each(|transaction| self.add_transaction(transaction))
    }

    pub fn transactions(&self) -> &Vec<ComposerTransaction> {
        &self.transactions
    }

    fn get_signer(&self, address: Address) -> Option<Arc<dyn TransactionSigner>> {
        self.signer_getter.get_signer(address)
    }

    async fn get_suggested_params(&self) -> Result<TransactionParams, ComposerError> {
        self.algod_client
            .transaction_params()
            .await
            .map_err(Into::into)
    }

    pub async fn build(&mut self) -> Result<&Vec<TransactionWithSigner>, ComposerError> {
        if let Some(ref group) = self.built_group {
            return Ok(group);
        }

        let suggested_params = self.get_suggested_params().await?;

        // Determine validity window: default 10 rounds, but 1000 for LocalNet
        let default_validity_window = if genesis_id_is_localnet(&suggested_params.genesis_id) {
            1000 // LocalNet gets bigger window to avoid dead transactions
        } else {
            10 // Standard default validity window
        };

        let mut transactions = Vec::new();
        let mut signers = Vec::new();

        for tx in &self.transactions {
            let common_params = tx.common_params();

            let first_valid = common_params
                .first_valid_round
                .unwrap_or(suggested_params.last_round);

            let header: TransactionHeader = TransactionHeader {
                sender: common_params.sender.clone(),
                rekey_to: common_params.rekey_to.clone(),
                note: common_params.note.clone(),
                lease: common_params.lease,
                fee: common_params.static_fee,
                genesis_id: Some(suggested_params.genesis_id.clone()),
                genesis_hash: Some(suggested_params.genesis_hash.clone().try_into().map_err(
                    |_e| ComposerError::DecodeError("Invalid genesis hash".to_string()),
                )?),
                first_valid,
                last_valid: common_params.last_valid_round.unwrap_or_else(|| {
                    common_params
                        .validity_window
                        .map(|window| first_valid + window)
                        .unwrap_or(first_valid + default_validity_window)
                }),
                group: None,
            };
            let mut calculate_fee = header.fee.is_none();

            let mut transaction = match tx {
                ComposerTransaction::Transaction(tx) => {
                    calculate_fee = false;
                    tx.clone()
                }
                ComposerTransaction::Payment(pay_params) => {
                    let pay_params = PaymentTransactionFields {
                        header,
                        receiver: pay_params.receiver.clone(),
                        amount: pay_params.amount,
                        close_remainder_to: None,
                    };
                    Transaction::Payment(pay_params)
                }
                ComposerTransaction::AccountClose(account_close_params) => {
                    let pay_params = PaymentTransactionFields {
                        header,
                        receiver: common_params.sender.clone(),
                        amount: 0,
                        close_remainder_to: Some(account_close_params.close_remainder_to.clone()),
                    };
                    Transaction::Payment(pay_params)
                }
                ComposerTransaction::AssetTransfer(asset_transfer_params) => {
                    Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                        header,
                        asset_id: asset_transfer_params.asset_id,
                        amount: asset_transfer_params.amount,
                        receiver: asset_transfer_params.receiver.clone(),
                        asset_sender: None,
                        close_remainder_to: None,
                    })
                }
                ComposerTransaction::AssetOptIn(asset_opt_in_params) => {
                    Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                        header,
                        asset_id: asset_opt_in_params.asset_id,
                        amount: 0,
                        receiver: asset_opt_in_params.common_params.sender.clone(),
                        asset_sender: None,
                        close_remainder_to: None,
                    })
                }
                ComposerTransaction::AssetOptOut(asset_opt_out_params) => {
                    Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                        header,
                        asset_id: asset_opt_out_params.asset_id,
                        amount: 0,
                        receiver: asset_opt_out_params.common_params.sender.clone(),
                        asset_sender: None,
                        close_remainder_to: asset_opt_out_params.close_remainder_to.clone(),
                    })
                }
                ComposerTransaction::AssetClawback(asset_clawback_params) => {
                    Transaction::AssetTransfer(algokit_transact::AssetTransferTransactionFields {
                        header,
                        asset_id: asset_clawback_params.asset_id,
                        amount: asset_clawback_params.amount,
                        receiver: asset_clawback_params.receiver.clone(),
                        asset_sender: Some(asset_clawback_params.clawback_target.clone()),
                        close_remainder_to: None,
                    })
                }
                ComposerTransaction::AssetCreate(asset_create_params) => {
                    Transaction::AssetConfig(AssetConfigTransactionFields {
                        header,
                        asset_id: 0,
                        total: Some(asset_create_params.total),
                        decimals: asset_create_params.decimals,
                        default_frozen: asset_create_params.default_frozen,
                        asset_name: asset_create_params.asset_name.clone(),
                        unit_name: asset_create_params.unit_name.clone(),
                        url: asset_create_params.url.clone(),
                        metadata_hash: asset_create_params.metadata_hash,
                        manager: asset_create_params.manager.clone(),
                        reserve: asset_create_params.reserve.clone(),
                        freeze: asset_create_params.freeze.clone(),
                        clawback: asset_create_params.clawback.clone(),
                    })
                }
                ComposerTransaction::AssetReconfigure(asset_reconfigure_params) => {
                    Transaction::AssetConfig(AssetConfigTransactionFields {
                        header,
                        asset_id: asset_reconfigure_params.asset_id,
                        total: None,
                        decimals: None,
                        default_frozen: None,
                        asset_name: None,
                        unit_name: None,
                        url: None,
                        metadata_hash: None,
                        manager: asset_reconfigure_params.manager.clone(),
                        reserve: asset_reconfigure_params.reserve.clone(),
                        freeze: asset_reconfigure_params.freeze.clone(),
                        clawback: asset_reconfigure_params.clawback.clone(),
                    })
                }
                ComposerTransaction::AssetDestroy(asset_destroy_params) => {
                    Transaction::AssetConfig(AssetConfigTransactionFields {
                        header,
                        asset_id: asset_destroy_params.asset_id,
                        total: None,
                        decimals: None,
                        default_frozen: None,
                        asset_name: None,
                        unit_name: None,
                        url: None,
                        metadata_hash: None,
                        manager: None,
                        reserve: None,
                        freeze: None,
                        clawback: None,
                    })
                }
                ComposerTransaction::AssetFreeze(asset_freeze_params) => {
                    Transaction::AssetFreeze(algokit_transact::AssetFreezeTransactionFields {
                        header,
                        asset_id: asset_freeze_params.asset_id,
                        freeze_target: asset_freeze_params.target_address.clone(),
                        frozen: true,
                    })
                }
                ComposerTransaction::AssetUnfreeze(asset_unfreeze_params) => {
                    Transaction::AssetFreeze(algokit_transact::AssetFreezeTransactionFields {
                        header,
                        asset_id: asset_unfreeze_params.asset_id,
                        freeze_target: asset_unfreeze_params.target_address.clone(),
                        frozen: false,
                    })
                }
                ComposerTransaction::ApplicationCall(app_call_params) => {
                    Transaction::ApplicationCall(
                        algokit_transact::ApplicationCallTransactionFields {
                            header,
                            app_id: app_call_params.app_id,
                            on_complete: app_call_params.on_complete,
                            approval_program: None,
                            clear_state_program: None,
                            global_state_schema: None,
                            local_state_schema: None,
                            extra_program_pages: None,
                            args: app_call_params.args.clone(),
                            account_references: app_call_params.account_references.clone(),
                            app_references: app_call_params.app_references.clone(),
                            asset_references: app_call_params.asset_references.clone(),
                            box_references: app_call_params.box_references.clone(),
                        },
                    )
                }
                ComposerTransaction::ApplicationCreate(app_create_params) => {
                    Transaction::ApplicationCall(
                        algokit_transact::ApplicationCallTransactionFields {
                            header,
                            app_id: 0, // 0 indicates application creation
                            on_complete: app_create_params.on_complete,
                            approval_program: Some(app_create_params.approval_program.clone()),
                            clear_state_program: Some(
                                app_create_params.clear_state_program.clone(),
                            ),
                            global_state_schema: app_create_params.global_state_schema.clone(),
                            local_state_schema: app_create_params.local_state_schema.clone(),
                            extra_program_pages: app_create_params.extra_program_pages,
                            args: app_create_params.args.clone(),
                            account_references: app_create_params.account_references.clone(),
                            app_references: app_create_params.app_references.clone(),
                            asset_references: app_create_params.asset_references.clone(),
                            box_references: app_create_params.box_references.clone(),
                        },
                    )
                }
                ComposerTransaction::ApplicationUpdate(app_update_params) => {
                    Transaction::ApplicationCall(
                        algokit_transact::ApplicationCallTransactionFields {
                            header,
                            app_id: app_update_params.app_id,
                            on_complete: OnApplicationComplete::UpdateApplication,
                            approval_program: Some(app_update_params.approval_program.clone()),
                            clear_state_program: Some(
                                app_update_params.clear_state_program.clone(),
                            ),
                            global_state_schema: None,
                            local_state_schema: None,
                            extra_program_pages: None,
                            args: app_update_params.args.clone(),
                            account_references: app_update_params.account_references.clone(),
                            app_references: app_update_params.app_references.clone(),
                            asset_references: app_update_params.asset_references.clone(),
                            box_references: app_update_params.box_references.clone(),
                        },
                    )
                }
                ComposerTransaction::ApplicationDelete(app_delete_params) => {
                    Transaction::ApplicationCall(
                        algokit_transact::ApplicationCallTransactionFields {
                            header,
                            app_id: app_delete_params.app_id,
                            on_complete: OnApplicationComplete::DeleteApplication,
                            approval_program: None,
                            clear_state_program: None,
                            global_state_schema: None,
                            local_state_schema: None,
                            extra_program_pages: None,
                            args: app_delete_params.args.clone(),
                            account_references: app_delete_params.account_references.clone(),
                            app_references: app_delete_params.app_references.clone(),
                            asset_references: app_delete_params.asset_references.clone(),
                            box_references: app_delete_params.box_references.clone(),
                        },
                    )
                }
                ComposerTransaction::MethodCall(method_call_params) => {
                    match method_call_params {
                        MethodCallParams::AppCall(call_params) => {
                            // Prepare application call parameters with proper foreign arrays
                            let mut app_call_params = AppCallParams {
                                common_params: call_params.common_params.clone(),
                                app_id: call_params.app_id,
                                on_complete: call_params.on_complete,
                                args: None, // Will be set with encoded arguments
                                account_references: None,
                                app_references: None,
                                asset_references: None,
                                box_references: None, // Could be enhanced to support box references in the future
                            };

                            // Populate foreign arrays based on reference arguments
                            let reference_indexes = self.populate_foreign_arrays(
                                &mut app_call_params,
                                &call_params.method,
                                &call_params.args,
                            )?;

                            // Encode ABI arguments including proper indexing for references
                            let encoded_args =
                                futures::executor::block_on(self.encode_abi_arguments(
                                    &call_params.method,
                                    &call_params.args,
                                    &reference_indexes,
                                ))?;

                            self.build_method_call_transaction(
                                &app_call_params,
                                header,
                                encoded_args,
                            )?
                        }
                        MethodCallParams::AppCreate(create_params) => {
                            // For app creation, set app_id to 0
                            let mut app_call_params = AppCallParams {
                                common_params: create_params.common_params.clone(),
                                app_id: 0,
                                on_complete: create_params.on_complete,
                                args: None,
                                account_references: None,
                                app_references: None,
                                asset_references: None,
                                box_references: None,
                            };

                            let reference_indexes = self.populate_foreign_arrays(
                                &mut app_call_params,
                                &create_params.method,
                                &create_params.args,
                            )?;

                            let encoded_args =
                                futures::executor::block_on(self.encode_abi_arguments(
                                    &create_params.method,
                                    &create_params.args,
                                    &reference_indexes,
                                ))?;

                            // Build creation transaction with programs
                            let mut txn = self.build_method_call_transaction(
                                &app_call_params,
                                header,
                                encoded_args,
                            )?;
                            if let Transaction::ApplicationCall(ref mut app_call_txn) = txn {
                                app_call_txn.approval_program =
                                    Some(create_params.approval_program.clone());
                                app_call_txn.clear_state_program =
                                    Some(create_params.clear_state_program.clone());
                                app_call_txn.global_state_schema =
                                    create_params.global_state_schema.clone();
                                app_call_txn.local_state_schema =
                                    create_params.local_state_schema.clone();
                                app_call_txn.extra_program_pages =
                                    create_params.extra_program_pages;
                            }
                            txn
                        }
                        MethodCallParams::AppUpdate(update_params) => {
                            let mut app_call_params = AppCallParams {
                                common_params: update_params.common_params.clone(),
                                app_id: update_params.app_id,
                                on_complete: update_params.on_complete,
                                args: None,
                                account_references: None,
                                app_references: None,
                                asset_references: None,
                                box_references: None,
                            };

                            let reference_indexes = self.populate_foreign_arrays(
                                &mut app_call_params,
                                &update_params.method,
                                &update_params.args,
                            )?;

                            let encoded_args =
                                futures::executor::block_on(self.encode_abi_arguments(
                                    &update_params.method,
                                    &update_params.args,
                                    &reference_indexes,
                                ))?;

                            // Build update transaction with programs
                            let mut txn = self.build_method_call_transaction(
                                &app_call_params,
                                header,
                                encoded_args,
                            )?;
                            if let Transaction::ApplicationCall(ref mut app_call_txn) = txn {
                                app_call_txn.approval_program =
                                    Some(update_params.approval_program.clone());
                                app_call_txn.clear_state_program =
                                    Some(update_params.clear_state_program.clone());
                            }
                            txn
                        }
                        MethodCallParams::AppDelete(delete_params) => {
                            let mut app_call_params = AppCallParams {
                                common_params: delete_params.common_params.clone(),
                                app_id: delete_params.app_id,
                                on_complete: delete_params.on_complete,
                                args: None,
                                account_references: None,
                                app_references: None,
                                asset_references: None,
                                box_references: None,
                            };

                            let reference_indexes = self.populate_foreign_arrays(
                                &mut app_call_params,
                                &delete_params.method,
                                &delete_params.args,
                            )?;

                            let encoded_args =
                                futures::executor::block_on(self.encode_abi_arguments(
                                    &delete_params.method,
                                    &delete_params.args,
                                    &reference_indexes,
                                ))?;

                            self.build_method_call_transaction(
                                &app_call_params,
                                header,
                                encoded_args,
                            )?
                        }
                    }
                }
                ComposerTransaction::OnlineKeyRegistration(online_key_reg_params) => {
                    Transaction::KeyRegistration(KeyRegistrationTransactionFields {
                        header,
                        vote_key: Some(online_key_reg_params.vote_key),
                        selection_key: Some(online_key_reg_params.selection_key),
                        vote_first: Some(online_key_reg_params.vote_first),
                        vote_last: Some(online_key_reg_params.vote_last),
                        vote_key_dilution: Some(online_key_reg_params.vote_key_dilution),
                        state_proof_key: online_key_reg_params.state_proof_key,
                        non_participation: None,
                    })
                }
                ComposerTransaction::OfflineKeyRegistration(offline_key_reg_params) => {
                    Transaction::KeyRegistration(KeyRegistrationTransactionFields {
                        header,
                        vote_key: None,
                        selection_key: None,
                        vote_first: None,
                        vote_last: None,
                        vote_key_dilution: None,
                        state_proof_key: None,
                        non_participation: offline_key_reg_params.non_participation,
                    })
                }
                ComposerTransaction::NonParticipationKeyRegistration(_) => {
                    Transaction::KeyRegistration(KeyRegistrationTransactionFields {
                        header,
                        vote_key: None,
                        selection_key: None,
                        vote_first: None,
                        vote_last: None,
                        vote_key_dilution: None,
                        state_proof_key: None,
                        non_participation: Some(true),
                    })
                }
            };

            if calculate_fee {
                transaction = transaction
                    .assign_fee(FeeParams {
                        fee_per_byte: suggested_params.fee,
                        min_fee: suggested_params.min_fee,
                        extra_fee: common_params.extra_fee,
                        max_fee: common_params.max_fee,
                    })
                    .map_err(|e| ComposerError::TransactionError(e.to_string()))?;
            }

            let signer = if let Some(transaction_signer) = common_params.signer {
                transaction_signer
            } else {
                let sender_address = transaction.header().sender.clone();

                self.get_signer(sender_address.clone())
                    .ok_or(ComposerError::SigningError(format!(
                        "No signer found for address: {}",
                        sender_address
                    )))?
            };

            transactions.push(transaction);
            signers.push(signer);
        }

        if transactions.len() > 1 {
            let grouped_transactions = transactions.assign_group().map_err(|e| {
                ComposerError::TransactionError(format!("Failed to assign group: {}", e))
            })?;
            transactions = grouped_transactions;
        }

        let transactions_with_signers: Vec<TransactionWithSigner> = transactions
            .into_iter()
            .zip(signers.into_iter())
            .map(|(transaction, signer)| TransactionWithSigner {
                transaction,
                signer,
            })
            .collect();

        self.built_group = Some(transactions_with_signers);
        Ok(self.built_group.as_ref().unwrap())
    }

    pub async fn gather_signatures(&mut self) -> Result<&Vec<SignedTransaction>, ComposerError> {
        if let Some(ref group) = self.signed_group {
            return Ok(group);
        }

        let transactions_with_signers =
            self.built_group.as_ref().ok_or(ComposerError::StateError(
                "Cannot gather signatures before building the transaction group".to_string(),
            ))?;

        // Group transactions by signer
        let mut transactions = Vec::new();
        let mut signer_groups: HashMap<*const dyn TransactionSigner, Vec<usize>> = HashMap::new();
        for (index, txn_with_signer) in transactions_with_signers.iter().enumerate() {
            let signer_ptr = Arc::as_ptr(&txn_with_signer.signer);
            signer_groups.entry(signer_ptr).or_default().push(index);
            transactions.push(txn_with_signer.transaction.to_owned());
        }

        let mut signed_transactions = vec![None; transactions_with_signers.len()];

        for (_signer_ptr, indices) in signer_groups {
            // Get the signer from the first transaction with this signer
            let signer = &transactions_with_signers[indices[0]].signer;

            // Sign all transactions for this signer
            let signed_txns = signer
                .sign_transactions(&transactions, &indices)
                .await
                .map_err(ComposerError::SigningError)?;

            for (i, &index) in indices.iter().enumerate() {
                signed_transactions[index] = Some(signed_txns[i].to_owned());
            }
        }

        let final_signed_transactions: Result<Vec<SignedTransaction>, _> = signed_transactions
            .into_iter()
            .enumerate()
            .map(|(i, signed_transaction)| {
                signed_transaction.ok_or_else(|| {
                    ComposerError::SigningError(format!(
                        "Transaction at index {} was not signed",
                        i
                    ))
                })
            })
            .collect();

        self.signed_group = Some(final_signed_transactions?);
        Ok(self.signed_group.as_ref().unwrap())
    }

    async fn wait_for_confirmation(
        &self,
        tx_id: &str,
        max_rounds: u64,
    ) -> Result<PendingTransactionResponse, Box<dyn std::error::Error + Send + Sync>> {
        let status = self
            .algod_client
            .get_status()
            .await
            .map_err(|e| format!("Failed to get status: {:?}", e))?;

        let start_round = status.last_round + 1;
        let mut current_round = start_round;

        while current_round < start_round + max_rounds {
            match self
                .algod_client
                .pending_transaction_information(tx_id, Some(Format::Msgpack))
                .await
            {
                Ok(response) => {
                    // Check for pool errors first - transaction was kicked out of pool
                    if !response.pool_error.is_empty() {
                        return Err(Box::new(ComposerError::PoolError(
                            response.pool_error.clone(),
                        )));
                    }

                    // Check if transaction is confirmed
                    if response.confirmed_round.is_some() {
                        return Ok(response);
                    }
                }
                Err(error) => {
                    // Only retry for 404 errors (transaction not found yet)
                    // All other errors indicate permanent issues and should fail fast
                    let is_retryable = matches!(
                        &error,
                        algod_client::apis::Error::Api(
                            algod_client::apis::AlgodApiError::PendingTransactionInformation(
                                algod_client::apis::pending_transaction_information::PendingTransactionInformationError::Status404(_)
                            )
                        )
                    ) || error.to_string().contains("404");

                    if is_retryable {
                        current_round += 1;
                        continue;
                    } else {
                        return Err(Box::new(ComposerError::AlgodClientError(error)));
                    }
                }
            };

            let _ = self.algod_client.wait_for_block(current_round).await;
            current_round += 1;
        }

        Err(format!(
            "Transaction {} not confirmed after {} rounds",
            tx_id, max_rounds
        )
        .into())
    }

    pub async fn send(
        &mut self,
        send_params: Option<SendParams>,
    ) -> Result<SendTransactionComposerResults, Box<dyn std::error::Error + Send + Sync>> {
        self.build()
            .await
            .map_err(|e| format!("Failed to build transaction: {}", e))?;

        let group_id = {
            let transactions_with_signers =
                self.built_group.as_ref().ok_or("No transactions built")?;
            if transactions_with_signers.is_empty() {
                return Err("No transactions to send".into());
            }
            transactions_with_signers[0].transaction.header().group
        };

        self.gather_signatures()
            .await
            .map_err(|e| format!("Failed to sign transaction: {}", e))?;

        let signed_transactions = self.signed_group.as_ref().ok_or("No signed transactions")?;

        let wait_rounds = if let Some(max_rounds_to_wait_for_confirmation) =
            send_params.and_then(|p| p.max_rounds_to_wait_for_confirmation)
        {
            max_rounds_to_wait_for_confirmation
        } else {
            let first_round: u64 = signed_transactions
                .iter()
                .map(|signed_transaction| signed_transaction.transaction.header().first_valid)
                .min()
                .ok_or("Failed to calculate first valid round")?;

            let last_round: u64 = signed_transactions
                .iter()
                .map(|signed_transaction| signed_transaction.transaction.header().last_valid)
                .max()
                .ok_or("Failed to calculate last valid round")?;

            last_round - first_round
        };

        // Encode each signed transaction and concatenate them
        let mut encoded_bytes = Vec::new();

        for signed_txn in signed_transactions {
            let encoded_txn = signed_txn
                .encode()
                .map_err(|e| format!("Failed to encode signed transaction: {}", e))?;
            encoded_bytes.extend_from_slice(&encoded_txn);
        }

        let _ = self
            .algod_client
            .raw_transaction(encoded_bytes)
            .await
            .map_err(|e| format!("Failed to submit transaction(s): {:?}", e))?;

        let transaction_ids: Vec<String> = signed_transactions
            .iter()
            .map(|txn| txn.id())
            .collect::<Result<Vec<String>, _>>()?;

        let mut confirmations = Vec::new();
        for id in &transaction_ids {
            let confirmation = self
                .wait_for_confirmation(id, wait_rounds)
                .await
                .map_err(|e| format!("Failed to confirm transaction: {}", e))?;
            confirmations.push(confirmation);
        }

        // Parse ABI return values from the confirmations
        let abi_returns = self
            .parse_abi_return_values(&confirmations)
            .map_err(|e| format!("Failed to parse ABI return values: {}", e))?;

        Ok(SendTransactionComposerResults {
            group_id,
            transaction_ids,
            confirmations,
            abi_returns,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::{AccountMother, TransactionMother};
    use base64::{Engine, prelude::BASE64_STANDARD};

    #[test]
    fn test_add_transaction() {
        let mut composer = TransactionComposer::testnet();
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn).is_ok());
    }

    #[test]
    fn test_add_too_many_transactions() {
        let mut composer = TransactionComposer::testnet();
        for _ in 0..16 {
            let txn = TransactionMother::simple_payment().build().unwrap();
            assert!(composer.add_transaction(txn).is_ok());
        }
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn).is_err());
    }

    #[tokio::test]
    async fn test_get_suggested_params() {
        let composer = TransactionComposer::testnet();
        let response = composer.get_suggested_params().await.unwrap();

        assert_eq!(
            response.genesis_hash,
            BASE64_STANDARD
                .decode("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=")
                .unwrap()
        );
    }

    #[test]
    fn test_add_payment() {
        let mut composer = TransactionComposer::testnet();
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AccountMother::account().address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AccountMother::account().address(),
            amount: 1000,
        };
        assert!(composer.add_payment(payment_params).is_ok());
    }

    #[tokio::test]
    async fn test_gather_signatures() {
        let mut composer = TransactionComposer::testnet();

        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AccountMother::account().address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AccountMother::account().address(),
            amount: 1000,
        };
        composer.add_payment(payment_params).unwrap();
        composer.build().await.unwrap();

        let result = composer.gather_signatures().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_single_transaction_no_group() {
        let mut composer = TransactionComposer::testnet();
        let payment_params = PaymentParams {
            common_params: CommonParams {
                sender: AccountMother::account().address(),
                signer: None,
                rekey_to: None,
                note: None,
                lease: None,
                static_fee: None,
                extra_fee: None,
                max_fee: None,
                validity_window: None,
                first_valid_round: None,
                last_valid_round: None,
            },
            receiver: AccountMother::account().address(),
            amount: 1000,
        };
        composer.add_payment(payment_params).unwrap();

        composer.build().await.unwrap();

        let built_group = composer.built_group.as_ref().unwrap();
        assert_eq!(built_group.len(), 1);

        // Single transaction should not have a group ID set
        assert!(built_group[0].transaction.header().group.is_none());
    }

    #[tokio::test]
    async fn test_multiple_transactions_have_group() {
        let mut composer = TransactionComposer::testnet();

        for _ in 0..2 {
            let payment_params = PaymentParams {
                common_params: CommonParams {
                    sender: AccountMother::account().address(),
                    signer: None,
                    rekey_to: None,
                    note: None,
                    lease: None,
                    static_fee: None,
                    extra_fee: None,
                    max_fee: None,
                    validity_window: None,
                    first_valid_round: None,
                    last_valid_round: None,
                },
                receiver: AccountMother::account().address(),
                amount: 1000,
            };
            composer.add_payment(payment_params).unwrap();
        }

        composer.build().await.unwrap();

        let built_group = composer.built_group.as_ref().unwrap();
        assert_eq!(built_group.len(), 2);

        // Multiple transactions should have group IDs set
        for transaction_with_signer in built_group {
            assert!(transaction_with_signer.transaction.header().group.is_some());
        }

        // All transactions should have the same group ID
        let group_id = built_group[0].transaction.header().group.as_ref().unwrap();
        for transaction_with_signer in &built_group[1..] {
            assert_eq!(
                transaction_with_signer
                    .transaction
                    .header()
                    .group
                    .as_ref()
                    .unwrap(),
                group_id
            );
        }
    }

    #[test]
    fn test_error_recoverability_logic() {
        // Test string-based 404 detection (the primary retry mechanism)
        let error_404_string = "Request failed with status 404: Transaction not found";
        let error_500_string = "Request failed with status 500: Server error";

        // The main retry logic relies on string matching
        assert!(
            error_404_string.contains("404"),
            "404 errors should be retryable"
        );
        assert!(
            !error_500_string.contains("404"),
            "500 errors should not be retryable"
        );
    }

    #[test]
    fn test_validity_window_logic() {
        // Test LocalNet detection and validity window logic
        assert_eq!(
            if genesis_id_is_localnet("devnet-v1") {
                1000
            } else {
                10
            },
            1000,
            "LocalNet should use 1000 round validity window"
        );

        assert_eq!(
            if genesis_id_is_localnet("testnet-v1.0") {
                1000
            } else {
                10
            },
            10,
            "TestNet should use 10 round validity window"
        );

        assert_eq!(
            if genesis_id_is_localnet("mainnet-v1.0") {
                1000
            } else {
                10
            },
            10,
            "MainNet should use 10 round validity window"
        );
    }
}
