use crate::{abi_tuple_type::encode_tuple, abi_type::get_name, error::ABIError, ABIType, ABIValue};

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
        ABIType::ABIDynamicArray(child_type) => {
            let elements = vec![*child_type; values.len()];
            ABIType::ABITupleType(elements)
        }
        _ => return Err(ABIError::EncodingError("Expected StaticArray".to_string())),
    };

    return encode_tuple(&tuple_type, value);
}

pub fn decode_dynamic_array(abi_type: &ABIType, value: &[u8]) -> Result<Vec<u8>, ABIError> {
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
        ABIType::ABIDynamicArray(child_type) => {
            let elements = vec![*child_type; values.len()];
            ABIType::ABITupleType(elements)
        }
        _ => return Err(ABIError::EncodingError("Expected StaticArray".to_string())),
    };

    return encode_tuple(&tuple_type, value);
}
