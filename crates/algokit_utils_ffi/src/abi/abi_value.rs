use std::{str::FromStr, sync::Arc};

use algokit_abi::ABIType as RustABIType;
use algokit_abi::ABIValue as RustABIValue;

use crate::transactions::common::UtilsError;

use super::abi_type::ABIType;

#[derive(uniffi::Object, Debug, Clone, PartialEq)]
pub struct ABIValue {
    bytes: Vec<u8>,
    abi_value: RustABIValue,
    abi_type_str: String,
}

#[uniffi::export]
impl ABIValue {
    pub fn encoded_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn abi_type(&self) -> String {
        self.abi_type_str.clone()
    }

    pub fn get_bool(&self) -> Result<bool, UtilsError> {
        if let RustABIValue::Bool(b) = self.abi_value {
            Ok(b)
        } else {
            Err(UtilsError::UtilsError {
                message: format!("ABIValue is not a bool: {:?}", self.abi_value),
            })
        }
    }

    #[uniffi::constructor]
    pub fn from_bytes(bytes: Vec<u8>, abi_type: &ABIType) -> Result<Self, UtilsError> {
        let rust_abi_type =
            RustABIType::from_str(&abi_type.to_string()).map_err(|e| UtilsError::UtilsError {
                message: format!("Failed to parse ABI type: {}", e),
            })?;
        let abi_value = rust_abi_type
            .decode(&bytes)
            .map_err(|e| UtilsError::UtilsError {
                message: format!("Failed to decode bytes: {}", e),
            })?;

        Ok(ABIValue {
            bytes,
            abi_type_str: abi_type.to_string(),
            abi_value,
        })
    }

    // TODO: support > u64
    #[uniffi::constructor]
    pub fn uint(value: u64, width: u16) -> Self {
        let abi_type_str = format!("uint{}", width);
        let abi_type = RustABIType::from_str(&abi_type_str).unwrap();
        let abi_value = RustABIValue::Uint(value.into());
        let bytes = abi_type.encode(&abi_value).unwrap();

        ABIValue {
            bytes,
            abi_type_str,
            abi_value,
        }
    }

    #[uniffi::constructor]
    pub fn bool(value: bool) -> Self {
        let abi_type_str = "bool".to_string();
        let rust_abi_type = RustABIType::from_str(&abi_type_str).unwrap();
        let abi_value = RustABIValue::Bool(value);
        let bytes = rust_abi_type.encode(&abi_value).unwrap();

        ABIValue {
            bytes,
            abi_type_str,
            abi_value,
        }
    }

    #[uniffi::constructor]
    pub fn string(value: String) -> Self {
        let abi_type_str = "string".to_string();
        let rust_abi_type = RustABIType::from_str(&abi_type_str).unwrap();
        let abi_value = RustABIValue::String(value.clone());
        let bytes = rust_abi_type.encode(&abi_value).unwrap();

        ABIValue {
            bytes,
            abi_type_str,
            abi_value,
        }
    }

    #[uniffi::constructor]
    pub fn byte(value: u8) -> Self {
        let abi_type_str = "byte".to_string();
        let rust_abi_type = RustABIType::from_str(&abi_type_str).unwrap();
        let abi_value = RustABIValue::Byte(value);
        let bytes = rust_abi_type.encode(&abi_value).unwrap();

        ABIValue {
            bytes,
            abi_type_str,
            abi_value,
        }
    }

    #[uniffi::constructor]
    pub fn address(value: String) -> Self {
        let abi_type_str = "address".to_string();
        let rust_abi_type = RustABIType::from_str(&abi_type_str).unwrap();
        let abi_value = RustABIValue::Address(value.clone());
        let bytes = rust_abi_type.encode(&abi_value).unwrap();
        ABIValue {
            bytes,
            abi_type_str,
            abi_value,
        }
    }

    #[uniffi::constructor]
    pub fn array(element_type: &ABIType, values: Vec<Arc<ABIValue>>) -> Self {
        let abi_type_str = format!("{}[]", element_type.to_string());
        let rust_abi_type = RustABIType::from_str(&abi_type_str).unwrap();
        let abi_value = RustABIValue::Array(
            values
                .iter()
                .map(|v| v.abi_value.clone())
                .collect::<Vec<RustABIValue>>(),
        );

        let bytes = rust_abi_type.encode(&abi_value).unwrap();
        ABIValue {
            bytes,
            abi_type_str,
            abi_value,
        }
    }

    #[uniffi::constructor]
    pub fn static_array(element_type: &ABIType, size: u64, values: Vec<Arc<ABIValue>>) -> Self {
        let abi_type_str = format!("{}[{}]", element_type.to_string(), size);
        let rust_abi_type = RustABIType::from_str(&abi_type_str).unwrap();
        let abi_value = RustABIValue::Array(
            values
                .iter()
                .map(|v| v.abi_value.clone())
                .collect::<Vec<RustABIValue>>(),
        );
        let bytes = rust_abi_type.encode(&abi_value).unwrap();
        ABIValue {
            bytes,
            abi_type_str,
            abi_value,
        }
    }
}
