use crate::{
    Address, ApplicationCallTransactionFields, AssetConfigTransactionFields,
    AssetFreezeTransactionFields, AssetTransferTransactionFields, KeyRegistrationTransactionFields,
    PaymentTransactionFields, Transaction,
};

/// Extension trait providing accessor methods for Transaction enum variants
pub trait TransactionExt {
    /// Returns true if this is a payment transaction
    fn is_payment(&self) -> bool;

    /// Returns true if this is an asset transfer transaction
    fn is_asset_transfer(&self) -> bool;

    /// Returns true if this is an asset config transaction
    fn is_asset_config(&self) -> bool;

    /// Returns true if this is an asset freeze transaction
    fn is_asset_freeze(&self) -> bool;

    /// Returns true if this is an application call transaction
    fn is_application_call(&self) -> bool;

    /// Returns true if this is a key registration transaction
    fn is_key_registration(&self) -> bool;

    /// Returns a reference to the payment transaction if this is a payment
    fn as_payment(&self) -> Option<&PaymentTransactionFields>;

    /// Returns a reference to the asset transfer transaction if this is an asset transfer
    fn as_asset_transfer(&self) -> Option<&AssetTransferTransactionFields>;

    /// Returns a reference to the asset config transaction if this is an asset config
    fn as_asset_config(&self) -> Option<&AssetConfigTransactionFields>;

    /// Returns a reference to the asset freeze transaction if this is an asset freeze
    fn as_asset_freeze(&self) -> Option<&AssetFreezeTransactionFields>;

    /// Returns a reference to the application call transaction if this is an application call
    fn as_application_call(&self) -> Option<&ApplicationCallTransactionFields>;

    /// Returns a reference to the key registration transaction if this is a key registration
    fn as_key_registration(&self) -> Option<&KeyRegistrationTransactionFields>;

    /// Returns the transaction type as a string for display purposes
    fn transaction_type(&self) -> &'static str;

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
    fn is_payment(&self) -> bool {
        matches!(self, Transaction::Payment(_))
    }

    fn is_asset_transfer(&self) -> bool {
        matches!(self, Transaction::AssetTransfer(_))
    }

    fn is_asset_config(&self) -> bool {
        matches!(self, Transaction::AssetConfig(_))
    }

    fn is_asset_freeze(&self) -> bool {
        matches!(self, Transaction::AssetFreeze(_))
    }

    fn is_application_call(&self) -> bool {
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

    fn as_asset_config(&self) -> Option<&AssetConfigTransactionFields> {
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

    fn as_application_call(&self) -> Option<&ApplicationCallTransactionFields> {
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

    fn transaction_type(&self) -> &'static str {
        match self {
            Transaction::Payment(_) => "Payment",
            Transaction::AssetTransfer(_) => "AssetTransfer",
            Transaction::AssetConfig(_) => "AssetConfig",
            Transaction::AssetFreeze(_) => "AssetFreeze",
            Transaction::ApplicationCall(_) => "ApplicationCall",
            Transaction::KeyRegistration(_) => "KeyRegistration",
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
mod tests {
    use super::*;
    use crate::{PaymentTransactionFields, TransactionHeader};

    #[test]
    fn test_payment_transaction_ext() {
        let header = TransactionHeader {
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
        };

        let payment = PaymentTransactionFields {
            header,
            receiver: Address([1u8; 32]),
            amount: 1000,
            close_remainder_to: None,
        };
        let transaction = Transaction::Payment(payment);

        assert!(transaction.is_payment());
        assert!(!transaction.is_asset_transfer());
        assert_eq!(transaction.transaction_type(), "Payment");

        let payment_ref = transaction.as_payment().unwrap();
        assert_eq!(payment_ref.amount, 1000);

        // Test header accessors
        assert_eq!(transaction.fee(), Some(1000));
        assert_eq!(transaction.first_valid_round(), 100);
        assert_eq!(transaction.last_valid_round(), 200);
        assert_eq!(transaction.sender(), &Address([0u8; 32]));
        assert_eq!(transaction.note(), None);
    }
}