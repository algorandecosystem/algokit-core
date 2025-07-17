use crate::{error::ABIError, ABIType, ABIValue};

/// Encode a boolean value to ABI format.
/// True values are encoded as 0x80, false values as 0x00.
pub fn encode_bool(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIBool => {
            let bool_value = match value {
                ABIValue::Bool(b) => b,
                _ => {
                    return Err(ABIError::EncodingError(
                        "Cannot encode value as bool: expected a boolean".to_string(),
                    ));
                }
            };

            if *bool_value {
                Ok(vec![0x80]) // true -> 128 (MSB set)
            } else {
                Ok(vec![0x00]) // false -> 0
            }
        }
        _ => Err(ABIError::EncodingError("Expected ABIBool".to_string())),
    }
}

/// Decode a boolean value from ABI format.
/// Expects exactly 1 byte: 0x80 for true, 0x00 for false.
pub fn decode_bool(abi_type: &ABIType, bytes: Vec<u8>) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::ABIBool => {
            if bytes.len() != 1 {
                return Err(ABIError::DecodingError(
                    "Bool string must be 1 byte long".to_string(),
                ));
            }

            match bytes[0] {
                0x80 => Ok(ABIValue::Bool(true)),
                0x00 => Ok(ABIValue::Bool(false)),
                _ => Err(ABIError::DecodingError(
                    "Boolean could not be decoded from the byte string".to_string(),
                )),
            }
        }
        _ => Err(ABIError::DecodingError("Expected ABIBool".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_true() {
        let abi_type = ABIType::ABIBool;
        let value = ABIValue::Bool(true);
        let encoded = encode_bool(&abi_type, &value).unwrap();
        assert_eq!(encoded, vec![128]); // 0x80
    }

    #[test]
    fn test_encode_false() {
        let abi_type = ABIType::ABIBool;
        let value = ABIValue::Bool(false);
        let encoded = encode_bool(&abi_type, &value).unwrap();
        assert_eq!(encoded, vec![0]); // 0x00
    }

    #[test]
    fn test_decode_true() {
        let abi_type = ABIType::ABIBool;
        let bytes = vec![128]; // 0x80
        let decoded = decode_bool(&abi_type, bytes).unwrap();
        assert_eq!(decoded, ABIValue::Bool(true));
    }

    #[test]
    fn test_decode_false() {
        let abi_type = ABIType::ABIBool;
        let bytes = vec![0]; // 0x00
        let decoded = decode_bool(&abi_type, bytes).unwrap();
        assert_eq!(decoded, ABIValue::Bool(false));
    }

    #[test]
    fn test_round_trip() {
        let test_cases = vec![true, false];

        for test_bool in test_cases {
            let value = ABIValue::Bool(test_bool);

            let encoded = encode_bool(&ABIType::ABIBool, &value).unwrap();
            let decoded = decode_bool(&ABIType::ABIBool, encoded).unwrap();

            assert_eq!(decoded, value);
        }
    }

    #[test]
    fn test_encode_wrong_type() {
        let abi_type = ABIType::ABIBool;
        let value = ABIValue::String("true".to_string());

        let result = encode_bool(&abi_type, &value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot encode value as bool"));
    }

    #[test]
    fn test_decode_wrong_length() {
        let abi_type = ABIType::ABIBool;
        let bytes = vec![0x80, 0x00]; // 2 bytes instead of 1

        let result = decode_bool(&abi_type, bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Bool string must be 1 byte long"));
    }

    #[test]
    fn test_decode_invalid_value() {
        let abi_type = ABIType::ABIBool;
        let bytes = vec![0x30]; // Invalid value (not 0x80 or 0x00)

        let result = decode_bool(&abi_type, bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Boolean could not be decoded"));
    }

    #[test]
    fn test_decode_wrong_abi_type() {
        let abi_type = ABIType::ABIString;
        let bytes = vec![0x80];

        let result = decode_bool(&abi_type, bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Expected ABIBool"));
    }
}
