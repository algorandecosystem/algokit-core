use std::fmt::Display;

use num_bigint::BigUint;

use super::{Type, Value};
use crate::AlgoKitTransactError;

#[derive(Debug, Clone, PartialEq)]
pub struct ABIUintType {
    bit_size: u16,
}

impl ABIUintType {
    pub fn new(size: u16) -> Result<Self, AlgoKitTransactError> {
        if size % 8 != 0 || size < 8 || size > 512 {
            return Err(AlgoKitTransactError::ABITypeError(format!(
                "unsupported uint type bitSize: {}",
                size
            )));
        }

        Ok(ABIUintType { bit_size: size })
    }
}

impl Display for ABIUintType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "uint{}", self.bit_size)
    }
}

impl Type for ABIUintType {
    fn is_dynamic(&self) -> bool {
        false
    }

    fn byte_len(&self) -> usize {
        (self.bit_size / 8) as usize
    }

    fn encode(&self, value: Value) -> Result<Vec<u8>, AlgoKitTransactError> {
        let value = match value {
            Value::Uint(n) => n,
            _ => {
                return Err(AlgoKitTransactError::ABITypeError(format!(
                    "Cannot encode value as uint{}: expected number",
                    self.bit_size
                )));
            }
        };

        if value >= BigUint::from(2u64).pow(self.bit_size.into()).into() {
            return Err(AlgoKitTransactError::ABITypeError(format!(
                "{} is too big to fit in uint{}",
                value, self.bit_size
            )));
        }

        Ok(super::utils::big_uint_to_bytes(value, self.byte_len()))
    }

    fn decode(&self, bytes: Vec<u8>) -> Result<Value, AlgoKitTransactError> {
        let expected_len = self.byte_len();
        if bytes.len() != expected_len {
            return Err(AlgoKitTransactError::ABITypeError(format!(
                "Invalid byte array length, expected {} bytes, got {}",
                expected_len,
                bytes.len()
            )));
        }

        Ok(Value::Uint(BigUint::from_bytes_be(&bytes)))
    }
}
