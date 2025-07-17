use crate::{error::ABIError, ABIType, ABIValue};

/// Encode an address value to ABI format.
/// Addresses are encoded as 32-byte public keys.
pub fn encode_address(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIAddressType => {
            let address_bytes = match value {
                ABIValue::Address(a) => a,
                _ => {
                    return Err(ABIError::EncodingError(
                        "Cannot encode value as address: expected a 32-byte array".to_string(),
                    ));
                }
            };

            Ok(address_bytes.to_vec())
        }
        _ => Err(ABIError::EncodingError(
            "Expected ABIAddressType".to_string(),
        )),
    }
}

/// Decode an address value from ABI format.
/// Expects exactly 32 bytes and returns an Address ABIValue.
pub fn decode_address(abi_type: &ABIType, bytes: Vec<u8>) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::ABIAddressType => {
            if bytes.len() != 32 {
                return Err(ABIError::DecodingError(
                    "Address byte string must be 32 bytes long".to_string(),
                ));
            }

            let mut address_bytes = [0u8; 32];
            address_bytes.copy_from_slice(&bytes);
            Ok(ABIValue::Address(address_bytes))
        }
        _ => Err(ABIError::DecodingError(
            "Expected ABIAddressType".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ADDRESS_BYTES: [u8; 32] = [
        0x61, 0x37, 0x3f, 0x95, 0x3e, 0x87, 0x9f, 0x48, 0x9f, 0x95, 0x53, 0x42, 0x20, 0x20, 0x91,
        0x35, 0x2f, 0x59, 0x56, 0x02, 0x19, 0x99, 0x91, 0x11, 0x61, 0x07, 0x49, 0x5a, 0x5c, 0x4f,
        0x71, 0x00,
    ];

    #[test]
    fn test_address_round_trip() {
        let value = ABIValue::Address(TEST_ADDRESS_BYTES);
        let encoded = encode_address(&ABIType::ABIAddressType, &value).unwrap();
        let decoded = decode_address(&ABIType::ABIAddressType, encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_address_encoding() {
        let value = ABIValue::Address(TEST_ADDRESS_BYTES);
        let encoded = encode_address(&ABIType::ABIAddressType, &value).unwrap();
        assert_eq!(encoded, TEST_ADDRESS_BYTES.to_vec());
    }

    #[test]
    fn test_address_decoding() {
        let bytes = TEST_ADDRESS_BYTES.to_vec();
        let decoded = decode_address(&ABIType::ABIAddressType, bytes).unwrap();
        assert_eq!(decoded, ABIValue::Address(TEST_ADDRESS_BYTES));
    }

    #[test]
    fn test_multiple_addresses_round_trip() {
        let test_addresses = vec![
            [0u8; 32],
            [0xFFu8; 32],
            TEST_ADDRESS_BYTES,
            [
                0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
                0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x01, 0x02, 0x03, 0x04,
                0x05, 0x06, 0x07, 0x08,
            ],
        ];

        for test_address in test_addresses {
            let value = ABIValue::Address(test_address);
            let encoded = encode_address(&ABIType::ABIAddressType, &value).unwrap();
            let decoded = decode_address(&ABIType::ABIAddressType, encoded).unwrap();
            assert_eq!(decoded, value);
        }
    }

    #[test]
    fn test_encode_wrong_type() {
        let value = ABIValue::String("not an address".to_string());
        let result = encode_address(&ABIType::ABIAddressType, &value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot encode value as address"));
    }

    #[test]
    fn test_encode_wrong_abi_type() {
        let value = ABIValue::Address(TEST_ADDRESS_BYTES);
        let result = encode_address(&ABIType::ABIString, &value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Expected ABIAddressType"));
    }

    #[test]
    fn test_decode_wrong_length_too_short() {
        let bytes = vec![0u8; 31];
        let result = decode_address(&ABIType::ABIAddressType, bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Address byte string must be 32 bytes long"));
    }

    #[test]
    fn test_decode_wrong_length_too_long() {
        let bytes = vec![0u8; 33];
        let result = decode_address(&ABIType::ABIAddressType, bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Address byte string must be 32 bytes long"));
    }

    #[test]
    fn test_decode_empty_bytes() {
        let bytes = vec![];
        let result = decode_address(&ABIType::ABIAddressType, bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Address byte string must be 32 bytes long"));
    }

    #[test]
    fn test_decode_wrong_abi_type() {
        let bytes = TEST_ADDRESS_BYTES.to_vec();
        let result = decode_address(&ABIType::ABIString, bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Expected ABIAddressType"));
    }
}
