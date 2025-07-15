use crate::{error::ABIError, ABIType, ABIValue};

pub fn encode_address(abi_type: ABIType, value: ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIAddressType => {
            let value = match value {
                ABIValue::Address(a) => a,
                _ => {
                    return Err(ABIError::EncodingError(format!(
                        "Cannot encode value as address: expected a 32-byte array",
                    )));
                }
            };

            return Ok(value.to_vec());
        }
        _ => Err(ABIError::EncodingError(
            "Expected ABIAddressType".to_string(),
        )),
    }
}
