use std::{fmt::Display, str::FromStr};

use algokit_abi::ABIType as RustABIType;

use crate::transactions::common::UtilsError;

#[derive(uniffi::Record)]
pub struct ABIType {
    pub abi_type: String,
}

impl Display for ABIType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abi_type)
    }
}

#[uniffi::export]
pub fn normalize_abi_type(abi_type: String) -> Result<String, UtilsError> {
    Ok(RustABIType::from_str(&abi_type)
        .map_err(|e| UtilsError::UtilsError {
            message: format!("Failed to parse ABI type {}: {}", abi_type, e),
        })?
        .to_string())
}
