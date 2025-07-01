//! Algorand address handling and manipulation.
//!
//! This module provides functionality for working with Algorand addresses,
//! including creation, validation, encoding, and decoding. Algorand addresses
//! are base32-encoded strings that represent a public key with a checksum.

use crate::address::Address;
use crate::constants::{Byte32, ALGORAND_CHECKSUM_BYTE_LENGTH, ALGORAND_PUBLIC_KEY_BYTE_LENGTH};
use crate::error::AlgoKitTransactError;
use crate::utils::pub_key_to_checksum;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Represents a single signature account.
///
/// An Algorand single signature account is a Ed25519 keypair.
/// Its address is a base32 string represented as a 58-character base32-encoded string
/// consisting of the 32 bytes of the public key and a 4-byte checksum.
/// This struct encapsulates the underlying public key and provides
/// methods for creating, validating, and converting human-readable addresses.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(transparent)]
pub struct Account {
    /// The 32-byte Ed25519 public key associated with this account.
    #[serde_as(as = "Bytes")]
    pub pub_key: Byte32,
}

impl Account {
    /// Creates a new Account from a 32-byte public key.
    ///
    /// # Parameters
    /// * `pub_key` - The 32-byte Ed25519 public key
    ///
    /// # Returns
    /// A new Account instance containing the provided public key.
    pub fn from_pubkey(pub_key: &Byte32) -> Self {
        Account { pub_key: *pub_key }
    }

    /// Calculates the 4-byte checksum for this address's public key.
    ///
    /// # Returns
    /// A 4-byte array containing the checksum.
    pub fn checksum(&self) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
        pub_key_to_checksum(&self.pub_key)
    }
}

impl TryFrom<Address> for Account {
    type Error = AlgoKitTransactError;

    /// Creates a new Account from an Address.
    ///
    /// # Parameters
    /// * `addr` - A 58-character base32-encoded Algorand address string
    ///
    /// # Returns
    /// The Address or an error if the string is invalid (checksum mismatch, etc.).
    fn try_from(addr: Address) -> Result<Self, Self::Error> {
        let pub_key: [u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH] = addr.as_bytes()
            [..ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
            .try_into()
            .map_err(|_| {
                AlgoKitTransactError::InvalidAddress(
                    "could not decode address into 32-byte public key".to_string(),
                )
            })?;

        let checksum: [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] = addr.as_bytes()
            [ALGORAND_PUBLIC_KEY_BYTE_LENGTH..]
            .try_into()
            .map_err(|_| {
                AlgoKitTransactError::InvalidAddress(
                    "could not get 4-byte checksum from decoded address".to_string(),
                )
            })?;

        let computed_checksum = pub_key_to_checksum(&pub_key);

        if computed_checksum != checksum {
            return Err(AlgoKitTransactError::InvalidAddress(
                "checksum is invalid".to_string(),
            ));
        }

        Ok(Self { pub_key })
    }
}

impl From<Account> for Address {
    fn from(account: Account) -> Address {
        let mut address_bytes =
            [0u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH + ALGORAND_CHECKSUM_BYTE_LENGTH];

        address_bytes[..ALGORAND_PUBLIC_KEY_BYTE_LENGTH].copy_from_slice(&account.pub_key);

        let checksum = account.checksum();
        address_bytes[ALGORAND_PUBLIC_KEY_BYTE_LENGTH..].copy_from_slice(&checksum);

        Address(address_bytes.to_vec())
    }
}

impl FromStr for Account {
    type Err = AlgoKitTransactError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Address>()?.try_into()
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", Address::from(self.clone()).as_str())
    }
}
