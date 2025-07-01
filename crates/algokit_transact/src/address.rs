use crate::error::AlgoKitTransactError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Represents an Algorand address as decoded bytes from a 58-character base32 string.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub Vec<u8>);

impl Address {
    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.0
    }
    pub fn as_str(&self) -> String {
        base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &self.0)
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
        let bytes =
            base32::decode(base32::Alphabet::Rfc4648 { padding: false }, s).ok_or_else(|| {
                AlgoKitTransactError::InvalidAddress(
                    "Invalid base32 encoding for Algorand address".into(),
                )
            })?;
        Ok(Address(bytes))
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}
