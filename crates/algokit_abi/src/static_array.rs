use crate::{
    abi_tuple_type::{decode_tuple, encode_tuple},
    error::ABIError,
    ABIType, ABIValue,
};

pub fn encode_static_array(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    let tuple_type = match abi_type {
        ABIType::ABIStaticArray(child_type, size) => {
            let elements = vec![*child_type; *size];
            ABIType::ABITupleType(elements)
        }
        _ => return Err(ABIError::EncodingError("Expected StaticArray".to_string())),
    };

    return encode_tuple(&tuple_type, value);
}

pub fn decode_static_array(abi_type: &ABIType, value: &[u8]) -> Result<ABIValue, ABIError> {
    let tuple_type = match abi_type {
        ABIType::ABIStaticArray(child_type, size) => {
            let elements = vec![*child_type; *size];
            ABIType::ABITupleType(elements)
        }
        _ => {
            return Err(ABIError::EncodingError(
                "Expected ABIDynamicArray".to_string(),
            ))
        }
    };

    return decode_tuple(&tuple_type, value);
}
