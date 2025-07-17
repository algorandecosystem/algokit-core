use crate::{error::ABIError, ABIType, ABIValue};

const ABI_LENGTH_SIZE: usize = 2;

pub fn encode_string(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIString => {
            let value = match value {
                ABIValue::String(s) => s,
                _ => {
                    return Err(ABIError::EncodingError(
                        "Cannot encode value as string: expected a String".to_string(),
                    ));
                }
            };

            let utf8_bytes = value.as_bytes().to_vec();
            let length = utf8_bytes.len() as u16;
            let mut result = Vec::with_capacity(2 + utf8_bytes.len());
            result.extend_from_slice(&length.to_be_bytes());
            result.extend_from_slice(&utf8_bytes);

            Ok(result)
        }
        _ => Err(ABIError::EncodingError("Expected ABIString".to_string())),
    }
}

pub fn decode_string(abi_type: &ABIType, value: &[u8]) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::ABIString => {
            if value.len() < ABI_LENGTH_SIZE {
                return Err(ABIError::DecodingError(
                    "Byte array too short for string".to_string(),
                ));
            }

            let length = u16::from_be_bytes([value[0], value[1]]) as usize;
            let content_bytes = &value[ABI_LENGTH_SIZE..];
            if content_bytes.len() != length {
                return Err(ABIError::DecodingError(format!(
                    "Invalid byte array length for string, expected {} value, got {}",
                    length,
                    content_bytes.len()
                )));
            }

            let string_value = String::from_utf8(content_bytes.to_vec())
                .map_err(|_| ABIError::DecodingError("Invalid UTF-8 encoding".to_string()))?;
            Ok(ABIValue::String(string_value))
        }
        _ => Err(ABIError::DecodingError("Expected ABIString".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_asdf() {
        let abi_type = ABIType::ABIString;
        let value = ABIValue::String("asdf".to_string());
        let encoded = encode_string(&abi_type, &value).unwrap();
        assert_eq!(encoded, vec![0, 4, 97, 115, 100, 102]);
    }

    #[test]
    fn test_encode_whats_new() {
        let abi_type = ABIType::ABIString;
        let value = ABIValue::String("What's new?".to_string());
        let encoded = encode_string(&abi_type, &value).unwrap();
        assert_eq!(
            encoded,
            vec![0, 11, 87, 104, 97, 116, 39, 115, 32, 110, 101, 119, 63]
        );
    }

    #[test]
    fn test_encode_emoji() {
        let abi_type = ABIType::ABIString;
        let value = ABIValue::String("😅🔨".to_string());
        let encoded = encode_string(&abi_type, &value).unwrap();
        assert_eq!(encoded, vec![0, 8, 240, 159, 152, 133, 240, 159, 148, 168]);
    }

    #[test]
    fn test_decode_asdf() {
        let abi_type = ABIType::ABIString;
        let bytes = vec![0, 4, 97, 115, 100, 102];
        let decoded = decode_string(&abi_type, &bytes).unwrap();
        assert_eq!(decoded, ABIValue::String("asdf".to_string()));
    }

    #[test]
    fn test_decode_whats_new() {
        let abi_type = ABIType::ABIString;
        let bytes = vec![0, 11, 87, 104, 97, 116, 39, 115, 32, 110, 101, 119, 63];
        let decoded = decode_string(&abi_type, &bytes).unwrap();
        assert_eq!(decoded, ABIValue::String("What's new?".to_string()));
    }

    #[test]
    fn test_decode_emoji() {
        let abi_type = ABIType::ABIString;
        let bytes = vec![0, 8, 240, 159, 152, 133, 240, 159, 148, 168];
        let decoded = decode_string(&abi_type, &bytes).unwrap();
        assert_eq!(decoded, ABIValue::String("😅🔨".to_string()));
    }

    #[test]
    fn test_round_trip() {
        let test_cases = vec!["asdf", "What's new", "😅🔨"];

        for test_string in test_cases {
            let value = ABIValue::String(test_string.to_string());

            let encoded = encode_string(&ABIType::ABIString, &value).unwrap();
            let decoded = decode_string(&ABIType::ABIString, &encoded).unwrap();

            assert_eq!(decoded, value);
        }
    }

    #[test]
    fn test_insufficient_bytes() {
        let abi_type = ABIType::ABIString;
        let bytes = vec![0]; // Only 1 byte, need 2 for length

        let result = decode_string(&abi_type, &bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_length_mismatch() {
        let abi_type = ABIType::ABIString;
        let bytes = vec![0, 5, 65, 66]; // Claims 5 bytes but only has 2

        let result = decode_string(&abi_type, &bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_input_type() {
        let abi_type = ABIType::ABIString;
        let value = ABIValue::Uint(num_bigint::BigUint::from(42u32));

        let result = encode_string(&abi_type, &value);
        assert!(result.is_err());
    }
}
