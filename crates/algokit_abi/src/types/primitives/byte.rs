use crate::{error::ABIError, ABIType, ABIValue};

impl ABIType {
    /// Encode a byte value (0-255) to ABI format.
    /// Values must be in the range 0-255 inclusive.
    pub(crate) fn encode_byte(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::Byte => match value {
                ABIValue::Byte(n) => Ok(vec![*n]),
                _ => Err(ABIError::EncodingError(
                    "ABI value mismatch, expected byte".to_string(),
                )),
            },
            _ => Err(ABIError::EncodingError(
                "ABI type mismatch, expected byte".to_string(),
            )),
        }
    }

    /// Decode a byte value from ABI format.
    /// Expects exactly 1 byte and returns the value as a BigUint.
    pub(crate) fn decode_byte(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::Byte => {
                if bytes.len() != 1 {
                    return Err(ABIError::DecodingError(
                        "Byte array must be 1 byte long".to_string(),
                    ));
                }

                Ok(ABIValue::Byte(bytes[0]))
            }
            _ => Err(ABIError::DecodingError(
                "ABI type mismatch, expected byte".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_wrong_type() {
        let value = ABIValue::String("10".to_string());

        let result = ABIType::Byte.encode(&value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI encoding failed: ABI value mismatch, expected byte"
        );
    }

    #[test]
    fn test_decode_wrong_length() {
        let bytes = vec![10, 20]; // 2 bytes instead of 1

        let result = ABIType::Byte.decode(&bytes);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "ABI decoding failed: Byte array must be 1 byte long"
        );
    }
}
