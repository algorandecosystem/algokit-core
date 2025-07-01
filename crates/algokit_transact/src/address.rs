use crate::constants::Byte32;
use crate::error::AlgoKitTransactError;
use crate::utils::pub_key_to_checksum;
use crate::{ALGORAND_CHECKSUM_BYTE_LENGTH, ALGORAND_PUBLIC_KEY_BYTE_LENGTH};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Represents an Algorand address as decoded bytes without the checksum from a 58-character base32 string.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub Byte32);

impl Address {
    pub fn as_bytes(&self) -> &Byte32 {
        &self.0
    }
    pub fn as_str(&self) -> String {
        let mut buffer = [0u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH + ALGORAND_CHECKSUM_BYTE_LENGTH];
        buffer[..ALGORAND_PUBLIC_KEY_BYTE_LENGTH].copy_from_slice(&self.0);

        let checksum = self.checksum();
        buffer[ALGORAND_PUBLIC_KEY_BYTE_LENGTH..].copy_from_slice(&checksum);

        base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &buffer)
    }

    pub fn checksum(&self) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
        pub_key_to_checksum(&self.0)
    }
}

impl FromStr for Address {
    type Err = AlgoKitTransactError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 58 {
            return Err(AlgoKitTransactError::InvalidAddress(
                "Algorand address must be exactly 58 characters".into(),
            ));
        }
        let decoded_address = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, s)
            .ok_or_else(|| {
                AlgoKitTransactError::InvalidAddress(
                    "Invalid base32 encoding for Algorand address".into(),
                )
            })?;

        // Although this is called public key (and it actually is when the account is a single signature account),
        // it could be the digest of a hash when the address corresponds to a multisignature account or
        // logic signature account.
        let pub_key: [u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH] = decoded_address
            [..ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
            .try_into()
            .map_err(|_| {
                AlgoKitTransactError::InvalidAddress(
                    "Could not decode address into 32-byte public key".to_string(),
                )
            })?;
        let checksum: [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] = decoded_address
            [ALGORAND_PUBLIC_KEY_BYTE_LENGTH..]
            .try_into()
            .map_err(|_| {
                AlgoKitTransactError::InvalidAddress(
                    "Could not get 4-byte checksum from decoded address".to_string(),
                )
            })?;

        if pub_key_to_checksum(&pub_key) != checksum {
            return Err(AlgoKitTransactError::InvalidAddress(
                "Checksum is invalid".to_string(),
            ));
        }
        Ok(Address { 0: pub_key })
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}
