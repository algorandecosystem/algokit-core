use crate::{
    abi_type::ABIType,
    abi_value::ABIValue,
    common::LENGTH_ENCODE_BYTE_SIZE,
    error::ABIError,
    types::collections::tuple::{decode_abi_types, encode_abi_types},
};

pub fn encode_dynamic_array(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    let values = match value {
        ABIValue::Array(n) => n,
        _ => {
            return Err(ABIError::EncodingError(format!(
                "Cannot encode tuple {}, expect an array of byte array",
                abi_type
            )));
        }
    };

    let child_type = match abi_type {
        ABIType::DynamicArray(child_type) => child_type,
        _ => return Err(ABIError::EncodingError("Expected DynamicArray".to_string())),
    };

    let child_types = vec![child_type.as_ref(); values.len()];
    encode_abi_types(&child_types, &values)
}

pub fn decode_dynamic_array(abi_type: &ABIType, value: &[u8]) -> Result<ABIValue, ABIError> {
    if value.len() < LENGTH_ENCODE_BYTE_SIZE {
        return Err(ABIError::DecodingError(
            "Value is too short to be decoded as tuple".to_string(),
        ));
    }

    // The first 2 bytes in the value determines how many values in the array
    let values_count = u16::from_be_bytes([value[0], value[1]]);

    let child_type = match abi_type {
        ABIType::DynamicArray(child_type) => child_type,
        _ => return Err(ABIError::EncodingError("Expected DynamicArray".to_string())),
    };

    let child_types = vec![child_type.as_ref(); values_count as usize];

    decode_abi_types(&child_types, value)
}
