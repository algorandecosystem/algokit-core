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
    fn is_dynamic(&self) -> Result<bool, AlgoKitTransactError> {
        Ok(false)
    }

    fn byte_len(&self) -> Result<u16, AlgoKitTransactError> {
        Ok(self.bit_size / 8)
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

        if value >= BigUint.pow(self.bit_size.into()).into() {
            return Err(AlgoKitTransactError::ABITypeError(format!(
                "{} is too big to fit in uint{}",
                value, self.bit_size
            )));
        }

        // Convert to bytes
        value.to_bytes_be();
        let byte_len = (self.bit_size / 8) as usize;
        let mut bytes = vec![0u8; byte_len];

        for i in 0..byte_len {
            bytes[byte_len - 1 - i] = ((value >> (i * 8)) & BigUint::from(0xFFu8))
                .to_u8()
                .unwrap();
        }

        Ok(bytes)
    }

    fn decode(&self, bytes: Vec<u8>) -> Result<Value, AlgoKitTransactError> {
        let expected_len = (self.bit_size / 8) as usize;
        if bytes.len() != expected_len {
            return Err(AlgoKitTransactError::ABITypeError(format!(
                "byte string must correspond to a uint{}, expected {} bytes, got {}",
                self.bit_size,
                expected_len,
                bytes.len()
            )));
        }

        let mut result = 0u64;
        for (i, &byte) in bytes.iter().enumerate() {
            result |= (byte as u64) << ((bytes.len() - 1 - i) * 8);
        }

        Ok(Value::Number(result))
    }
}
