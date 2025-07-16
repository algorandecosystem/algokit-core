use crate::{error::ABIError, ABIType, ABIValue};

/// Encode a byte value (0-255) to ABI format.
/// Values must be in the range 0-255 inclusive.
pub fn encode_byte(abi_type: ABIType, value: ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIByteType => {
            let byte_value = match value {
                ABIValue::Uint(n) => {
                    // Convert BigUint to u64 for range checking
                    if n > num_bigint::BigUint::from(255u8) {
                        return Err(ABIError::EncodingError(format!(
                            "{} cannot be encoded into a byte",
                            n
                        )));
                    }
                    // Handle the case where BigUint is 0 (empty digits)
                    if n.to_u64_digits().is_empty() {
                        0u8
                    } else {
                        n.to_u64_digits()[0] as u8
                    }
                }
                _ => {
                    return Err(ABIError::EncodingError(
                        "Cannot encode value as byte: expected a number".to_string(),
                    ));
                }
            };

            Ok(vec![byte_value])
        }
        _ => Err(ABIError::EncodingError(
            "Expected ABIByteType".to_string(),
        )),
    }
}

/// Decode a byte value from ABI format.
/// Expects exactly 1 byte and returns the value as a BigUint.
pub fn decode_byte(abi_type: ABIType, bytes: Vec<u8>) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::ABIByteType => {
            if bytes.len() != 1 {
                return Err(ABIError::DecodingError(
                    "Byte string must be 1 byte long".to_string(),
                ));
            }

            Ok(ABIValue::Uint(num_bigint::BigUint::from(bytes[0])))
        }
        _ => Err(ABIError::DecodingError(
            "Expected ABIByteType".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_encode_byte_10() {
        let abi_type = ABIType::ABIByteType;
        let value = ABIValue::Uint(BigUint::from(10u8));
        let encoded = encode_byte(abi_type, value).unwrap();
        assert_eq!(encoded, vec![10]);
    }

    #[test]
    fn test_encode_byte_255() {
        let abi_type = ABIType::ABIByteType;
        let value = ABIValue::Uint(BigUint::from(255u8));
        let encoded = encode_byte(abi_type, value).unwrap();
        assert_eq!(encoded, vec![255]);
    }

    #[test]
    fn test_encode_byte_0() {
        let abi_type = ABIType::ABIByteType;
        let value = ABIValue::Uint(BigUint::from(0u8));
        let encoded = encode_byte(abi_type, value).unwrap();
        assert_eq!(encoded, vec![0]);
    }

    #[test]
    fn test_decode_byte_10() {
        let abi_type = ABIType::ABIByteType;
        let bytes = vec![10];
        let decoded = decode_byte(abi_type, bytes).unwrap();
        assert_eq!(decoded, ABIValue::Uint(BigUint::from(10u8)));
    }

    #[test]
    fn test_decode_byte_255() {
        let abi_type = ABIType::ABIByteType;
        let bytes = vec![255];
        let decoded = decode_byte(abi_type, bytes).unwrap();
        assert_eq!(decoded, ABIValue::Uint(BigUint::from(255u8)));
    }

    #[test]
    fn test_decode_byte_0() {
        let abi_type = ABIType::ABIByteType;
        let bytes = vec![0];
        let decoded = decode_byte(abi_type, bytes).unwrap();
        assert_eq!(decoded, ABIValue::Uint(BigUint::from(0u8)));
    }

    #[test]
    fn test_round_trip() {
        let test_cases = vec![0u8, 1u8, 10u8, 255u8];

        for test_byte in test_cases {
            let value = ABIValue::Uint(BigUint::from(test_byte));

            let encoded = encode_byte(ABIType::ABIByteType, value.clone()).unwrap();
            let decoded = decode_byte(ABIType::ABIByteType, encoded).unwrap();

            assert_eq!(decoded, value);
        }
    }

    #[test]
    fn test_encode_out_of_range() {
        let abi_type = ABIType::ABIByteType;
        let value = ABIValue::Uint(BigUint::from(256u32));

        let result = encode_byte(abi_type, value);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be encoded into a byte"));
    }

    #[test]
    fn test_encode_wrong_type() {
        let abi_type = ABIType::ABIByteType;
        let value = ABIValue::String("10".to_string());

        let result = encode_byte(abi_type, value);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot encode value as byte"));
    }

    #[test]
    fn test_decode_wrong_length() {
        let abi_type = ABIType::ABIByteType;
        let bytes = vec![10, 20]; // 2 bytes instead of 1

        let result = decode_byte(abi_type, bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Byte string must be 1 byte long"));
    }

    #[test]
    fn test_decode_wrong_abi_type() {
        let abi_type = ABIType::ABIStringType;
        let bytes = vec![10];

        let result = decode_byte(abi_type, bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Expected ABIByteType"));
    }
}