use num_bigint::BigUint;

use crate::{abi_type::ABIType, abi_value::ABIValue, error::ABIError, utils};

impl ABIType {
    pub(crate) fn encode_ufixed(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::UFixed(bit_size, precision) => {
                let bit_size = bit_size.value();
                let precision = precision.value();

                let value = match value {
                    ABIValue::Uint(n) => n,
                    _ => {
                        return Err(ABIError::EncodingError(format!(
                            "Cannot encode value as ufixed{}x{}: expected number",
                            bit_size, precision
                        )));
                    }
                };

                if value >= &BigUint::from(2u64).pow(bit_size as u32) {
                    return Err(ABIError::EncodingError(format!(
                        "{} is too big to fit in ufixed{}x{}",
                        value, bit_size, precision
                    )));
                }

                Ok(utils::big_uint_to_bytes(&value, (bit_size / 8) as usize))
            }
            _ => Err(ABIError::EncodingError("Expected UFixedType".to_string())),
        }
    }

    pub(crate) fn decode_ufixed(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::UFixed(bit_size, _precision) => {
                let bit_size = bit_size.value();
                let expected_len = (bit_size / 8) as usize;
                if bytes.len() != expected_len {
                    return Err(ABIError::DecodingError(format!(
                        "Invalid byte array length, expected {} bytes, got {}",
                        expected_len,
                        bytes.len()
                    )));
                }

                Ok(ABIValue::Uint(BigUint::from_bytes_be(bytes)))
            }
            _ => Err(ABIError::DecodingError("Expected UFixedType".to_string())),
        }
    }
}
