use crate::{abi_type::ABIType, abi_value::ABIValue, error::ABIError};

use super::tuple::{decode_tuple, encode_tuple};

pub fn encode_static_array(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    let tuple_type = match abi_type {
        ABIType::StaticArray(child_type, size) => {
            let elements = vec![child_type.clone(); *size];
            ABIType::Tuple(elements)
        }
        _ => return Err(ABIError::EncodingError("Expected StaticArray".to_string())),
    };

    encode_tuple(&tuple_type, value)
}

pub fn decode_static_array(abi_type: &ABIType, value: &[u8]) -> Result<ABIValue, ABIError> {
    let tuple_type = match abi_type {
        ABIType::StaticArray(child_type, size) => {
            let elements = vec![child_type.clone(); *size];
            ABIType::Tuple(elements)
        }
        _ => return Err(ABIError::EncodingError("Expected DynamicArray".to_string())),
    };

    decode_tuple(&tuple_type, value)
}
