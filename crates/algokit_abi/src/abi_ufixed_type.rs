use num_bigint::BigUint;

use super::ABIValue;
use crate::{abi_type::ABIType, error::ABIError};

pub fn encode_ufixed(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIUFixedType(bit_size, precision) => {
            let value = match value {
                ABIValue::Uint(n) => n,
                _ => {
                    return Err(ABIError::EncodingError(format!(
                        "Cannot encode value as ufixed{}x{}: expected number",
                        bit_size, precision
                    )));
                }
            };

            if value >= &BigUint::from(2u64).pow(*bit_size as u32) {
                return Err(ABIError::EncodingError(format!(
                    "{} is too big to fit in ufixed{}x{}",
                    value, bit_size, precision
                )));
            }

            Ok(super::utils::extend_bytes_to_length(
                &value.to_bytes_be(),
                (bit_size / 8) as usize,
            ))
        }
        _ => Err(ABIError::EncodingError(
            "Expected ABIUFixedType".to_string(),
        )),
    }
}

pub fn decode_ufixed(abi_type: ABIType, bytes: Vec<u8>) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::ABIUFixedType(bit_size, _precision) => {
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
        _ => Err(ABIError::DecodingError(
            "Expected ABIUFixedType".to_string(),
        )),
    }
}
