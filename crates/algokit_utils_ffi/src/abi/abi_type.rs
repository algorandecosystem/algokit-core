use std::{str::FromStr, sync::Arc};

use algokit_abi::ABIType as RustABIType;

use crate::transactions::common::UtilsError;

#[derive(uniffi::Object, Debug)]
pub struct ABIType {
    pub abi_type: RustABIType,
}

#[uniffi::export]
impl ABIType {
    #[uniffi::constructor]
    pub fn from_string(type_str: &str) -> Result<Self, UtilsError> {
        RustABIType::from_str(type_str)
            .map(|abi_type| ABIType { abi_type })
            .map_err(|e| UtilsError::UtilsError {
                message: e.to_string(),
            })
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.abi_type.to_string()
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

    #[uniffi::constructor]
    pub fn uint(bit_size: u16) -> Result<Self, UtilsError> {
        let abi_type =
            RustABIType::Uint(algokit_abi::abi_type::BitSize::new(bit_size).map_err(|_| {
                UtilsError::UtilsError {
                    message: format!("Invalid bit size: {}", bit_size),
                }
            })?);

        Ok(ABIType { abi_type })
    }

    #[uniffi::constructor]
    pub fn ufixed(bit_size: u16, precision: u8) -> Result<Self, UtilsError> {
        let abi_type = RustABIType::UFixed(
            algokit_abi::abi_type::BitSize::new(bit_size).map_err(|_| UtilsError::UtilsError {
                message: format!("Invalid bit size: {}", bit_size),
            })?,
            algokit_abi::abi_type::Precision::new(precision).map_err(|_| {
                UtilsError::UtilsError {
                    message: format!("Invalid precision: {}", precision),
                }
            })?,
        );

        Ok(ABIType { abi_type })
    }

    #[uniffi::constructor]
    pub fn address() -> Self {
        ABIType {
            abi_type: RustABIType::Address,
        }
    }

    #[uniffi::constructor]
    pub fn tuple(elements: Vec<Arc<ABIType>>) -> Self {
        let rust_elements = elements.into_iter().map(|e| e.abi_type.clone()).collect();
        ABIType {
            abi_type: RustABIType::Tuple(rust_elements),
        }
    }

    #[uniffi::constructor]
    pub fn string() -> Self {
        ABIType {
            abi_type: RustABIType::String,
        }
    }

    #[uniffi::constructor]
    pub fn byte() -> Self {
        ABIType {
            abi_type: RustABIType::Byte,
        }
    }

    #[uniffi::constructor]
    pub fn bool() -> Self {
        ABIType {
            abi_type: RustABIType::Bool,
        }
    }

    #[uniffi::constructor]
    pub fn static_array(element_type: Arc<ABIType>, length: u16) -> Result<Self, UtilsError> {
        let abi_type =
            RustABIType::StaticArray(Box::new(element_type.abi_type.clone()), length as usize);
        Ok(ABIType { abi_type })
    }

    #[uniffi::constructor]
    pub fn dynamic_array(element_type: Arc<ABIType>) -> Self {
        let abi_type = RustABIType::DynamicArray(Box::new(element_type.abi_type.clone()));
        ABIType { abi_type }
    }

    // TODO: Struct

    #[uniffi::constructor]
    pub fn avm_bytes() -> Self {
        ABIType {
            abi_type: RustABIType::AVMBytes,
        }
    }

    #[uniffi::constructor]
    pub fn avm_string() -> Self {
        ABIType {
            abi_type: RustABIType::AVMString,
        }
    }

    #[uniffi::constructor]
    pub fn avm_uint64() -> Self {
        ABIType {
            abi_type: RustABIType::AVMUint64,
        }
    }
}
