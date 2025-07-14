use std::fmt::Display;

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
        // Current limitation: Value::Number is u64, so we can only handle up to 64 bits
        if size > 64 {
            return Err(AlgoKitTransactError::ABITypeError(format!(
                "uint{} not yet supported (current implementation limited to 64 bits)",
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
        let number_value = match value {
            Value::Number(n) => n,
            _ => {
                return Err(AlgoKitTransactError::ABITypeError(format!(
                    "Cannot encode value as uint{}: expected number",
                    self.bit_size
                )));
            }
        };

        if number_value >= 2.pow(self.bit_size) {
            return Err(AlgoKitTransactError::ABITypeError(format!(
                "{} is too big to fit in uint{}",
                number_value, self.bit_size
            )));
        }

        // Convert to bytes (big-endian)
        let byte_len = (self.bit_size / 8) as usize;
        let mut bytes = vec![0u8; byte_len];

        // Fill bytes from right to left (big-endian)
        for i in 0..byte_len {
            bytes[byte_len - 1 - i] = ((number_value >> (i * 8)) & 0xFF) as u8;
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

        // Convert bytes to u64 (big-endian)
        let mut value = 0u64;
        for &byte in &bytes {
            value = (value << 8) | (byte as u64);
        }

        Ok(Value::Number(value))
    }
}
