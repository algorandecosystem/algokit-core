use crate::{
    abi_type::ABIType,
    abi_value::ABIValue,
    constants::LENGTH_ENCODE_BYTE_SIZE,
    error::ABIError,
    types::collections::tuple::{decode_abi_types, encode_abi_types},
};

impl ABIType {
    pub(crate) fn encode_dynamic_array(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        let values = match value {
            ABIValue::Array(n) => n,
            _ => {
                return Err(ABIError::EncodingError(
                    "ABI value mismatch, expected an array of values".to_string(),
                ));
            }
        };

        let child_type = match self {
            ABIType::DynamicArray(child_type) => child_type,
            _ => {
                return Err(ABIError::EncodingError(
                    "ABI type mismatch, expected dynamic array".to_string(),
                ))
            }
        };

        let child_types = vec![child_type.as_ref(); values.len()];
        let encoded_value = encode_abi_types(&child_types, &values)?;
        let encoded_length = (child_types.len() as u16).to_be_bytes();

        let mut merged_bytes = encoded_length.to_vec();
        merged_bytes.extend(encoded_value);

        Ok(merged_bytes)
    }

    pub(crate) fn decode_dynamic_array(&self, value: &[u8]) -> Result<ABIValue, ABIError> {
        if value.len() < LENGTH_ENCODE_BYTE_SIZE {
            return Err(ABIError::DecodingError(
                "Value is too short to be decoded as tuple".to_string(),
            ));
        }

        // The first 2 bytes in the value determines how many values in the array
        let values_count = u16::from_be_bytes([value[0], value[1]]);

        let child_type = match self {
            ABIType::DynamicArray(child_type) => child_type,
            _ => {
                return Err(ABIError::EncodingError(
                    "ABI type mismatch, expected dynamic array".to_string(),
                ))
            }
        };

        let child_types = vec![child_type.as_ref(); values_count as usize];

        decode_abi_types(&child_types, &value[LENGTH_ENCODE_BYTE_SIZE..])
    }
}
