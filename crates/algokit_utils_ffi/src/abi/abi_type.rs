use std::str::FromStr;

use algokit_abi::ABIType as RustABIType;
use derive_more::Display;

use crate::transactions::common::UtilsError;

#[derive(uniffi::Record, Display)]
pub struct ABIType {
    pub abi_type: String,
}

#[uniffi::export]
pub fn normalize_abi_type(abi_type: String) -> Result<String, UtilsError> {
    Ok(RustABIType::from_str(&abi_type)
        .map_err(|e| UtilsError::UtilsError {
            message: format!("Failed to parse ABI type {}: {}", abi_type, e),
        })?
        .to_string())
}
