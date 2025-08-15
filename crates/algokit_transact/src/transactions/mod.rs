//! Transaction module for AlgoKit Core that provides functionality for creating, manipulating,
//! and managing different types of Algorand transactions.
//!
//! This module includes support for various transaction types, along with the ability to sign,
//! serialize, and deserialize them.

mod application_call;
mod asset_config;
mod asset_freeze;
mod asset_transfer;
mod common;
mod key_registration;
mod payment;

pub use application_call::{
    ApplicationCallTransactionBuilder, ApplicationCallTransactionFields, BoxReference,
    OnApplicationComplete, StateSchema,
};
use application_call::{application_call_deserializer, application_call_serializer};
pub use asset_config::{
    AssetConfigTransactionBuilder, AssetConfigTransactionFields, asset_config_deserializer,
    asset_config_serializer,
};
pub use asset_freeze::{AssetFreezeTransactionBuilder, AssetFreezeTransactionFields};
pub use asset_transfer::{AssetTransferTransactionBuilder, AssetTransferTransactionFields};
pub use common::{TransactionHeader, TransactionHeaderBuilder};
pub use key_registration::{KeyRegistrationTransactionBuilder, KeyRegistrationTransactionFields};
pub use payment::{PaymentTransactionBuilder, PaymentTransactionFields};

use crate::constants::{
    ALGORAND_SIGNATURE_BYTE_LENGTH, ALGORAND_SIGNATURE_ENCODING_INCR, HASH_BYTES_LENGTH,
    MAX_TX_GROUP_SIZE,
};
use crate::error::AlgoKitTransactError;
use crate::traits::{AlgorandMsgpack, EstimateTransactionSize, TransactionId, Transactions};
use crate::utils::{compute_group_id, is_zero_addr_opt};
use crate::{Address, MultisigSignature};
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};
use std::any::Any;

/// Enumeration of transaction types for pattern matching and type identification.
///
/// This enum provides a convenient way to identify transaction types without
/// inspecting the full transaction structure. It complements the existing
/// `is_*()` methods by enabling ergonomic pattern matching.
///
/// # Example
/// ```rust
/// use algokit_transact::{Transaction, TransactionExt, TransactionType};
///
/// fn handle_transaction(tx: &Transaction) {
///     match tx.transaction_type() {
///         TransactionType::Payment => println!("Processing payment"),
///         TransactionType::ApplicationCall => println!("Processing app call"),
///         TransactionType::AssetTransfer => println!("Processing asset transfer"),
///         _ => println!("Processing other transaction type"),
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransactionType {
    /// Payment transaction
    Payment,
    /// Asset transfer transaction
    AssetTransfer,
    /// Asset configuration transaction (create/update/destroy)
    AssetCreate,
    /// Asset freeze transaction
    AssetFreeze,
    /// Application call transaction
    ApplicationCall,
    /// Key registration transaction
    KeyRegistration,
}

/// Enumeration of all transaction types.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum Transaction {
    #[serde(rename = "pay")]
    Payment(PaymentTransactionFields),

    #[serde(rename = "axfer")]
    AssetTransfer(AssetTransferTransactionFields),

    #[serde(serialize_with = "asset_config_serializer")]
    #[serde(deserialize_with = "asset_config_deserializer")]
    #[serde(rename = "acfg")]
    AssetConfig(AssetConfigTransactionFields),

    #[serde(serialize_with = "application_call_serializer")]
    #[serde(deserialize_with = "application_call_deserializer")]
    #[serde(rename = "appl")]
    ApplicationCall(ApplicationCallTransactionFields),

    #[serde(rename = "afrz")]
    AssetFreeze(AssetFreezeTransactionFields),

    #[serde(rename = "keyreg")]
    KeyRegistration(KeyRegistrationTransactionFields),
}

#[derive(Default)]
pub struct FeeParams {
    pub fee_per_byte: u64,
    pub min_fee: u64,
    pub extra_fee: Option<u64>,
    pub max_fee: Option<u64>,
}

impl Transaction {
    pub fn header(&self) -> &TransactionHeader {
        match self {
            Transaction::Payment(p) => &p.header,
            Transaction::AssetTransfer(a) => &a.header,
            Transaction::AssetConfig(a) => &a.header,
            Transaction::ApplicationCall(a) => &a.header,
            Transaction::KeyRegistration(k) => &k.header,
            Transaction::AssetFreeze(f) => &f.header,
        }
    }

    pub fn header_mut(&mut self) -> &mut TransactionHeader {
        match self {
            Transaction::Payment(p) => &mut p.header,
            Transaction::AssetTransfer(a) => &mut a.header,
            Transaction::AssetConfig(a) => &mut a.header,
            Transaction::ApplicationCall(a) => &mut a.header,
            Transaction::KeyRegistration(k) => &mut k.header,
            Transaction::AssetFreeze(f) => &mut f.header,
        }
    }

    pub fn calculate_fee(&self, request: FeeParams) -> Result<u64, AlgoKitTransactError> {
        let mut calculated_fee: u64 = 0;

        if request.fee_per_byte > 0 {
            let estimated_size = self.estimate_size()?;
            calculated_fee = request.fee_per_byte * estimated_size as u64;
        }

        if calculated_fee < request.min_fee {
            calculated_fee = request.min_fee;
        }

        if let Some(extra_fee) = request.extra_fee {
            calculated_fee += extra_fee;
        }

        if let Some(max_fee) = request.max_fee {
            if calculated_fee > max_fee {
                return Err(AlgoKitTransactError::InputError(format!(
                    "Transaction fee {} µALGO is greater than max fee {} µALGO",
                    calculated_fee, max_fee
                )));
            }
        }

        Ok(calculated_fee)
    }

    pub fn assign_fee(&self, request: FeeParams) -> Result<Transaction, AlgoKitTransactError> {
        let mut tx = self.clone();
        let header = tx.header_mut();
        header.fee = Some(self.calculate_fee(request)?);

        Ok(tx)
    }
}

impl AlgorandMsgpack for Transaction {
    const PREFIX: &'static [u8] = b"TX";
}

impl TransactionId for Transaction {}

impl EstimateTransactionSize for Transaction {
    fn estimate_size(&self) -> Result<usize, AlgoKitTransactError> {
        Ok(self.encode_raw()?.len() + ALGORAND_SIGNATURE_ENCODING_INCR)
    }
}

/// A signed transaction.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SignedTransaction {
    /// The transaction that has been signed.
    #[serde(rename = "txn")]
    pub transaction: Transaction,

    /// Optional Ed25519 signature authorizing the transaction.
    #[serde(rename = "sig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Bytes>")]
    pub signature: Option<[u8; ALGORAND_SIGNATURE_BYTE_LENGTH]>,

    /// Optional auth address applicable if the transaction sender is a rekeyed account.
    #[serde(rename = "sgnr")]
    #[serde(skip_serializing_if = "is_zero_addr_opt")]
    #[serde(default)]
    pub auth_address: Option<Address>,

    /// Optional multisignature signature for the transaction.
    #[serde(rename = "msig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multisignature: Option<MultisigSignature>,
}

impl AlgorandMsgpack for SignedTransaction {
    /// Decodes MsgPack bytes into a SignedTransaction.
    ///
    /// # Parameters
    /// * `bytes` - The MsgPack encoded signed transaction bytes
    ///
    /// # Returns
    /// The decoded SignedTransaction or an error if decoding fails or the transaction type is not recognized.
    // Since we provide default values for all transaction fields, serde will not know which
    // transaction type the bytes actually correspond with. To fix this we need to manually
    // decode the transaction using Transaction::decode (which does check the type) and
    // then add it to the decoded struct
    fn decode(bytes: &[u8]) -> Result<Self, AlgoKitTransactError> {
        let value: rmpv::Value = rmp_serde::from_slice(bytes)?;

        match value {
            rmpv::Value::Map(map) => {
                let txn_value = &map
                    .iter()
                    .find(|(k, _)| k.as_str() == Some("txn"))
                    .unwrap()
                    .1;

                let mut txn_buf = Vec::new();
                rmpv::encode::write_value(&mut txn_buf, txn_value)?;

                let stxn = SignedTransaction {
                    transaction: Transaction::decode(&txn_buf)?,
                    ..rmp_serde::from_slice(bytes)?
                };

                Ok(stxn)
            }
            _ => Err(AlgoKitTransactError::InputError(format!(
                "expected signed transaction to be a map, but got a: {:#?}",
                value.type_id()
            ))),
        }
    }
}
impl TransactionId for SignedTransaction {
    /// Generates the raw transaction ID as a hash of the transaction data.
    ///
    /// # Returns
    /// The transaction ID as a byte array or an error if generation fails.
    fn id_raw(&self) -> Result<[u8; HASH_BYTES_LENGTH], AlgoKitTransactError> {
        self.transaction.id_raw()
    }
}

impl EstimateTransactionSize for SignedTransaction {
    fn estimate_size(&self) -> Result<usize, AlgoKitTransactError> {
        Ok(self.encode()?.len())
    }
}

impl Transactions for &[Transaction] {
    /// Groups the supplied transactions by calculating and assigning the group to each transaction.
    ///
    /// # Returns
    /// A result containing the transactions with group assign or an error if grouping fails.
    fn assign_group(self) -> Result<Vec<Transaction>, AlgoKitTransactError> {
        if self.len() > MAX_TX_GROUP_SIZE {
            return Err(AlgoKitTransactError::InputError(format!(
                "Transaction group size exceeds the max limit of {}",
                MAX_TX_GROUP_SIZE
            )));
        }

        if self.is_empty() {
            return Err(AlgoKitTransactError::InputError(String::from(
                "Transaction group size cannot be 0",
            )));
        }

        let group_id = compute_group_id(self)?;
        Ok(self
            .iter()
            .map(|tx| {
                let mut tx = tx.clone();
                tx.header_mut().group = Some(group_id);
                tx
            })
            .collect())
    }
}

/// Extension trait providing accessor methods for Transaction enum variants
pub trait TransactionExt {
    /// Returns the transaction type as an enum for pattern matching
    ///
    /// This method provides a convenient way to identify transaction types
    /// for use in match statements and other type-based logic.
    ///
    /// # Example
    /// ```rust
    /// use algokit_transact::{Transaction, TransactionExt, TransactionType};
    ///
    /// fn process_transaction(tx: &Transaction) {
    ///     match tx.transaction_type() {
    ///         TransactionType::Payment => { /* handle payment */ },
    ///         TransactionType::ApplicationCall => { /* handle app call */ },
    ///         _ => { /* handle other types */ },
    ///     }
    /// }
    /// ```
    fn transaction_type(&self) -> TransactionType;

    /// Returns true if this is a payment transaction
    fn is_payment(&self) -> bool;

    /// Returns true if this is an asset transfer transaction
    fn is_asset_transfer(&self) -> bool;

    /// Returns true if this is an asset create transaction
    fn is_asset_create(&self) -> bool;

    /// Returns true if this is an asset freeze transaction
    fn is_asset_freeze(&self) -> bool;

    /// Returns true if this is an app call transaction
    fn is_app_call(&self) -> bool;

    /// Returns true if this is a key registration transaction
    fn is_key_registration(&self) -> bool;

    /// Returns a reference to the payment transaction if this is a payment
    fn as_payment(&self) -> Option<&PaymentTransactionFields>;

    /// Returns a reference to the asset transfer transaction if this is an asset transfer
    fn as_asset_transfer(&self) -> Option<&AssetTransferTransactionFields>;

    /// Returns a reference to the asset create transaction if this is an asset create
    fn as_asset_create(&self) -> Option<&AssetConfigTransactionFields>;

    /// Returns a reference to the asset freeze transaction if this is an asset freeze
    fn as_asset_freeze(&self) -> Option<&AssetFreezeTransactionFields>;

    /// Returns a reference to the app call transaction if this is an app call
    fn as_app_call(&self) -> Option<&ApplicationCallTransactionFields>;

    /// Returns a reference to the key registration transaction if this is a key registration
    fn as_key_registration(&self) -> Option<&KeyRegistrationTransactionFields>;

    // Header field accessors
    /// Returns the sender address of the transaction
    fn sender(&self) -> &Address;

    /// Returns the fee of the transaction
    fn fee(&self) -> Option<u64>;

    /// Returns the first valid round of the transaction
    fn first_valid_round(&self) -> u64;

    /// Returns the last valid round of the transaction
    fn last_valid_round(&self) -> u64;

    /// Returns the note of the transaction
    fn note(&self) -> Option<&Vec<u8>>;
}

impl TransactionExt for Transaction {
    fn transaction_type(&self) -> TransactionType {
        match self {
            Transaction::Payment(_) => TransactionType::Payment,
            Transaction::AssetTransfer(_) => TransactionType::AssetTransfer,
            Transaction::AssetConfig(_) => TransactionType::AssetCreate,
            Transaction::AssetFreeze(_) => TransactionType::AssetFreeze,
            Transaction::ApplicationCall(_) => TransactionType::ApplicationCall,
            Transaction::KeyRegistration(_) => TransactionType::KeyRegistration,
        }
    }

    fn is_payment(&self) -> bool {
        matches!(self, Transaction::Payment(_))
    }

    fn is_asset_transfer(&self) -> bool {
        matches!(self, Transaction::AssetTransfer(_))
    }

    fn is_asset_create(&self) -> bool {
        matches!(self, Transaction::AssetConfig(_))
    }

    fn is_asset_freeze(&self) -> bool {
        matches!(self, Transaction::AssetFreeze(_))
    }

    fn is_app_call(&self) -> bool {
        matches!(self, Transaction::ApplicationCall(_))
    }

    fn is_key_registration(&self) -> bool {
        matches!(self, Transaction::KeyRegistration(_))
    }

    fn as_payment(&self) -> Option<&PaymentTransactionFields> {
        if let Transaction::Payment(payment) = self {
            Some(payment)
        } else {
            None
        }
    }

    fn as_asset_transfer(&self) -> Option<&AssetTransferTransactionFields> {
        if let Transaction::AssetTransfer(asset_transfer) = self {
            Some(asset_transfer)
        } else {
            None
        }
    }

    fn as_asset_create(&self) -> Option<&AssetConfigTransactionFields> {
        if let Transaction::AssetConfig(asset_config) = self {
            Some(asset_config)
        } else {
            None
        }
    }

    fn as_asset_freeze(&self) -> Option<&AssetFreezeTransactionFields> {
        if let Transaction::AssetFreeze(asset_freeze) = self {
            Some(asset_freeze)
        } else {
            None
        }
    }

    fn as_app_call(&self) -> Option<&ApplicationCallTransactionFields> {
        if let Transaction::ApplicationCall(app_call) = self {
            Some(app_call)
        } else {
            None
        }
    }

    fn as_key_registration(&self) -> Option<&KeyRegistrationTransactionFields> {
        if let Transaction::KeyRegistration(key_reg) = self {
            Some(key_reg)
        } else {
            None
        }
    }

    fn sender(&self) -> &Address {
        &self.header().sender
    }

    fn fee(&self) -> Option<u64> {
        self.header().fee
    }

    fn first_valid_round(&self) -> u64 {
        self.header().first_valid
    }

    fn last_valid_round(&self) -> u64 {
        self.header().last_valid
    }

    fn note(&self) -> Option<&Vec<u8>> {
        self.header().note.as_ref()
    }
}

#[cfg(test)]
mod transaction_ext_tests {
    use super::*;

    fn create_test_header() -> TransactionHeader {
        TransactionHeader {
            sender: Address([0u8; 32]),
            fee: Some(1000),
            first_valid: 100,
            last_valid: 200,
            genesis_hash: None,
            genesis_id: None,
            note: None,
            rekey_to: None,
            lease: None,
            group: None,
        }
    }

    #[test]
    fn test_payment_transaction_ext() {
        let payment = PaymentTransactionFields {
            header: create_test_header(),
            receiver: Address([1u8; 32]),
            amount: 1000,
            close_remainder_to: None,
        };
        let transaction = Transaction::Payment(payment);

        assert!(transaction.is_payment());
        assert!(!transaction.is_asset_transfer());

        let payment_ref = transaction.as_payment().unwrap();
        assert_eq!(payment_ref.amount, 1000);

        // Test header accessors
        assert_eq!(transaction.fee(), Some(1000));
        assert_eq!(transaction.first_valid_round(), 100);
        assert_eq!(transaction.last_valid_round(), 200);
        assert_eq!(transaction.sender(), &Address([0u8; 32]));
        assert_eq!(transaction.note(), None);
    }

    #[test]
    fn test_transaction_type_enum() {
        // Test Payment transaction
        let payment = Transaction::Payment(PaymentTransactionFields {
            header: create_test_header(),
            receiver: Address([1u8; 32]),
            amount: 1000,
            close_remainder_to: None,
        });
        assert_eq!(payment.transaction_type(), TransactionType::Payment);

        // Test AssetTransfer transaction
        let asset_transfer = Transaction::AssetTransfer(AssetTransferTransactionFields {
            header: create_test_header(),
            asset_id: 123,
            amount: 500,
            receiver: Address([2u8; 32]),
            asset_sender: None,
            close_remainder_to: None,
        });
        assert_eq!(
            asset_transfer.transaction_type(),
            TransactionType::AssetTransfer
        );

        // Test AssetConfig transaction
        let asset_config = Transaction::AssetConfig(AssetConfigTransactionFields {
            header: create_test_header(),
            asset_id: 456,
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
        });
        assert_eq!(
            asset_config.transaction_type(),
            TransactionType::AssetCreate
        );

        // Test AssetFreeze transaction
        let asset_freeze = Transaction::AssetFreeze(AssetFreezeTransactionFields {
            header: create_test_header(),
            asset_id: 789,
            freeze_target: Address([3u8; 32]),
            frozen: true,
        });
        assert_eq!(
            asset_freeze.transaction_type(),
            TransactionType::AssetFreeze
        );

        // Test ApplicationCall transaction
        let app_call = Transaction::ApplicationCall(ApplicationCallTransactionFields {
            header: create_test_header(),
            app_id: 321,
            on_complete: OnApplicationComplete::NoOp,
            approval_program: None,
            clear_state_program: None,
            global_state_schema: None,
            local_state_schema: None,
            extra_program_pages: None,
            args: None,
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: None,
        });
        assert_eq!(
            app_call.transaction_type(),
            TransactionType::ApplicationCall
        );

        // Test KeyRegistration transaction
        let key_reg = Transaction::KeyRegistration(KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: None,
            selection_key: None,
            state_proof_key: None,
            vote_first: None,
            vote_last: None,
            vote_key_dilution: None,
            non_participation: None,
        });
        assert_eq!(key_reg.transaction_type(), TransactionType::KeyRegistration);
    }

    #[test]
    fn test_transaction_type_pattern_matching() {
        let transactions = vec![
            Transaction::Payment(PaymentTransactionFields {
                header: create_test_header(),
                receiver: Address([1u8; 32]),
                amount: 1000,
                close_remainder_to: None,
            }),
            Transaction::AssetTransfer(AssetTransferTransactionFields {
                header: create_test_header(),
                asset_id: 123,
                amount: 500,
                receiver: Address([2u8; 32]),
                asset_sender: None,
                close_remainder_to: None,
            }),
            Transaction::ApplicationCall(ApplicationCallTransactionFields {
                header: create_test_header(),
                app_id: 321,
                on_complete: OnApplicationComplete::NoOp,
                approval_program: None,
                clear_state_program: None,
                global_state_schema: None,
                local_state_schema: None,
                extra_program_pages: None,
                args: None,
                account_references: None,
                app_references: None,
                asset_references: None,
                box_references: None,
            }),
        ];

        let mut payment_count = 0;
        let mut asset_count = 0;
        let mut app_count = 0;

        for tx in transactions {
            match tx.transaction_type() {
                TransactionType::Payment => payment_count += 1,
                TransactionType::AssetTransfer => asset_count += 1,
                TransactionType::ApplicationCall => app_count += 1,
                _ => {}
            }
        }

        assert_eq!(payment_count, 1);
        assert_eq!(asset_count, 1);
        assert_eq!(app_count, 1);
    }

    #[test]
    fn test_transaction_type_enum_equality_and_hash() {
        // Test PartialEq
        assert_eq!(TransactionType::Payment, TransactionType::Payment);
        assert_ne!(TransactionType::Payment, TransactionType::AssetTransfer);

        // Test that enum can be used in collections that require Hash
        use std::collections::HashSet;
        let mut type_set = HashSet::new();
        type_set.insert(TransactionType::Payment);
        type_set.insert(TransactionType::AssetTransfer);
        type_set.insert(TransactionType::Payment); // Duplicate should not increase size

        assert_eq!(type_set.len(), 2);
        assert!(type_set.contains(&TransactionType::Payment));
        assert!(type_set.contains(&TransactionType::AssetTransfer));
        assert!(!type_set.contains(&TransactionType::ApplicationCall));
    }
}
