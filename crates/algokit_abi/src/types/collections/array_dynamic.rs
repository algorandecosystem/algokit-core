use crate::{
    abi_type::{get_name, ABIType},
    abi_value::ABIValue,
    common::LENGTH_ENCODE_BYTE_SIZE,
    error::ABIError,
};

use super::tuple::{decode_tuple, encode_tuple};

pub fn encode_dynamic_array(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    let values = match value {
        ABIValue::Array(n) => n,
        _ => {
            return Err(ABIError::EncodingError(format!(
                "Cannot encode tuple {}, expect an array of byte array",
                get_name(abi_type)
            )));
        }
    };

    let tuple_type = match abi_type {
        ABIType::DynamicArray(child_type) => {
            let elements = vec![*child_type; values.len()];
            ABIType::Tuple(elements)
        }
        _ => return Err(ABIError::EncodingError("Expected StaticArray".to_string())),
    };

    encode_tuple(&tuple_type, value)
}

pub fn decode_dynamic_array(abi_type: &ABIType, value: &[u8]) -> Result<ABIValue, ABIError> {
    if value.len() < LENGTH_ENCODE_BYTE_SIZE {
        return Err(ABIError::DecodingError(
            "Value is too short to be decoded as tuple".to_string(),
        ));
    }

    // The first 2 bytes in the value determines how many values in the array
    let values_count = u16::from_be_bytes([value[0], value[1]]);

    let tuple_type = match abi_type {
        ABIType::DynamicArray(child_type) => {
            let elements = vec![*child_type; values_count as usize];
            ABIType::Tuple(elements)
        }
        _ => return Err(ABIError::EncodingError("Expected DynamicArray".to_string())),
    };

    decode_tuple(&tuple_type, value)
}
