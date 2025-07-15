use crate::{error::ABIError, ABIType, ABIValue};

pub fn encode_tuple(abi_type: ABIType, value: ABIValue) -> Result<Vec<u8>, ABIError> {
    let child_types = match abi_type {
        ABIType::ABITupleType(child_types) => child_types,
        _ => return Err(ABIError::EncodingError("Expected ABITupleType".to_string())),
    };
    let value = match value {
        ABIValue::Bytes(n) => n,
        _ => {
            return Err(ABIError::EncodingError(format!(
                "Cannot encode value",
                // TODO: error including tuple type name (to_string)
            )));
        }
    };

    let mut heads: Vec<u8>;
}
