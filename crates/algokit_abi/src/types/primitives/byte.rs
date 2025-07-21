use crate::{error::ABIError, ABIType, ABIValue};

impl ABIType {
    /// Encode a byte value (0-255) to ABI format.
    /// Values must be in the range 0-255 inclusive.
    pub fn encode_byte(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::Byte => {
                match value {
                    ABIValue::Byte(n) => {
                        return Ok(vec![*n]);
                    }
                    _ => {
                        return Err(ABIError::EncodingError(
                            "Cannot encode value as byte: expected a number".to_string(),
                        ));
                    }
                };
            }
            _ => Err(ABIError::EncodingError("Expected Byte".to_string())),
        }
    }

    /// Decode a byte value from ABI format.
    /// Expects exactly 1 byte and returns the value as a BigUint.
    pub fn decode_byte(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::Byte => {
                if bytes.len() != 1 {
                    return Err(ABIError::DecodingError(
                        "Byte array must be 1 byte long".to_string(),
                    ));
                }

                Ok(ABIValue::Byte(bytes[0]))
            }
            _ => Err(ABIError::DecodingError("Expected Byte".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_wrong_type() {
        let abi_type = ABIType::Byte;
        let value = ABIValue::String("10".to_string());

        let result = abi_type.encode_byte(&value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot encode value as byte"));
    }

    #[test]
    fn test_decode_wrong_length() {
        let abi_type = ABIType::Byte;
        let bytes = vec![10, 20]; // 2 bytes instead of 1

        let result = abi_type.decode_byte(&bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Byte array must be 1 byte long"));
    }
}
