use crate::{
    abi_type::ABIType,
    abi_value::ABIValue,
    error::ABIError,
    types::collections::tuple::{decode_abi_types, encode_abi_types},
};

impl ABIType {
    pub fn encode_static_array(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        let child_types = match self {
            ABIType::StaticArray(child_type, size) => vec![child_type.as_ref(); *size],
            _ => return Err(ABIError::EncodingError("Expected StaticArray".to_string())),
        };

        let values = match value {
            ABIValue::Array(n) => n,
            _ => {
                return Err(ABIError::EncodingError(format!(
                    "Cannot encode tuple {}, expect an array of byte array",
                    self
                )));
            }
        };

        encode_abi_types(&child_types, &values)
    }

    pub fn decode_static_array(&self, value: &[u8]) -> Result<ABIValue, ABIError> {
        let child_types = match self {
            ABIType::StaticArray(child_type, size) => vec![child_type.as_ref(); *size],
            _ => return Err(ABIError::EncodingError("Expected StaticArray".to_string())),
        };

        decode_abi_types(&child_types, value)
    }
}
