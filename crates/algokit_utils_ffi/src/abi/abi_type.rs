use std::str::FromStr;

use algokit_abi::ABIType as RustABIType;

use crate::transactions::common::UtilsError;

#[derive(uniffi::Object, Debug)]
pub struct ABIType {
    pub abi_type: RustABIType,
}

#[uniffi::export]
impl ABIType {
    #[uniffi::constructor]
    pub fn new(type_str: &str) -> Result<Self, UtilsError> {
        RustABIType::from_str(type_str)
            .map(|abi_type| ABIType { abi_type })
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn encode(&self, value: &crate::abi::abi_value::ABIValue) -> Result<Vec<u8>, UtilsError> {
        self.abi_type
            .encode(&value.rust_value)
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    pub fn decode(&self, data: &[u8]) -> Result<crate::abi::abi_value::ABIValue, UtilsError> {
        self.abi_type
            .decode(data)
            .map(|v| crate::abi::abi_value::ABIValue { rust_value: v })
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }
}
