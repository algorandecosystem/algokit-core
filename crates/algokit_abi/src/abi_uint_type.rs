use num_bigint::BigUint;

use super::ABIValue;
use crate::{abi_type::ABIType, error::ABIError};

pub fn encode_uint(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIUintType(bit_size) => {
            let bit_size = bit_size.value();
            let value = match value {
                ABIValue::Uint(n) => n,
                _ => {
                    return Err(ABIError::EncodingError(format!(
                        "Cannot encode value as uint{}: expected number",
                        bit_size
                    )));
                }
            };

            if value >= &BigUint::from(2u64).pow(bit_size as u32) {
                return Err(ABIError::EncodingError(format!(
                    "{} is too big to fit in uint{}",
                    value, bit_size
                )));
            }

            Ok(super::utils::extend_bytes_to_length(
                &value.to_bytes_be(),
                (bit_size / 8) as usize,
            ))
        }
        _ => Err(ABIError::EncodingError("Expected ABIUintType".to_string())),
    }
}

pub fn decode_uint(abi_type: ABIType, bytes: Vec<u8>) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::ABIUintType(bit_size) => {
            let bit_size = bit_size.value();
            let expected_len = (bit_size / 8) as usize;
            if bytes.len() != expected_len {
                return Err(ABIError::DecodingError(format!(
                    "Invalid byte array length, expected {} bytes, got {}",
                    expected_len,
                    bytes.len()
                )));
            }

            Ok(ABIValue::Uint(BigUint::from_bytes_be(&bytes)))
        }
        _ => Err(ABIError::DecodingError("Expected ABIUintType".to_string())),
    }
}
