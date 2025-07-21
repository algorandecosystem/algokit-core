use sha2::{Digest, Sha512_256};

use crate::{
    common::{
        ADDR_BYTE_SIZE, ALGORAND_ADDRESS_LENGTH, ALGORAND_CHECKSUM_BYTE_LENGTH, HASH_BYTES_LENGTH,
    },
    error::ABIError,
    ABIType, ABIValue,
};

/// Encode an address value to ABI format.
/// Addresses are encoded as 32-byte public keys.
pub fn encode_address(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::Address => {
            let address_str = match value {
                ABIValue::Address(a) => a,
                _ => {
                    return Err(ABIError::EncodingError(
                        "Cannot encode value as address: expected a String".to_string(),
                    ));
                }
            };

            if address_str.len() != ALGORAND_ADDRESS_LENGTH {
                return Err(ABIError::FormatError(
                    "Algorand address must be exactly 58 characters".into(),
                ));
            }
            let decoded_address =
                base32::decode(base32::Alphabet::Rfc4648 { padding: false }, address_str)
                    .ok_or_else(|| {
                        ABIError::FormatError("Invalid base32 encoding for Algorand address".into())
                    })?[..ADDR_BYTE_SIZE]
                    .to_vec();

            Ok(decoded_address)
        }
        _ => Err(ABIError::EncodingError("Expected Address".to_string())),
    }
}

/// Decode an address value from ABI format.
/// Expects exactly 32 bytes and returns an Address ABIValue.
pub fn decode_address(abi_type: &ABIType, bytes: &[u8]) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::Address => {
            if bytes.len() != ADDR_BYTE_SIZE {
                return Err(ABIError::DecodingError(format!(
                    "Address byte string must be {} bytes long",
                    ADDR_BYTE_SIZE
                )));
            }

            let mut buffer = [0u8; ADDR_BYTE_SIZE + ALGORAND_CHECKSUM_BYTE_LENGTH];
            buffer[..ADDR_BYTE_SIZE].copy_from_slice(bytes);

            let checksum = bytes_to_checksum(&bytes);
            buffer[ADDR_BYTE_SIZE..].copy_from_slice(&checksum);

            let address_str = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &buffer);

            Ok(ABIValue::Address(address_str))
        }
        _ => Err(ABIError::DecodingError("Expected Address".to_string())),
    }
}

// TODO: do we need [u8,32]
fn bytes_to_checksum(pub_key: &[u8]) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
    let mut hasher = Sha512_256::new();
    hasher.update(pub_key);

    let mut checksum = [0u8; ALGORAND_CHECKSUM_BYTE_LENGTH];
    checksum
        .copy_from_slice(&hasher.finalize()[(HASH_BYTES_LENGTH - ALGORAND_CHECKSUM_BYTE_LENGTH)..]);
    checksum
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const TEST_ADDRESS_BYTES: [u8; 32] = [
//         0x61, 0x37, 0x3f, 0x95, 0x3e, 0x87, 0x9f, 0x48, 0x9f, 0x95, 0x53, 0x42, 0x20, 0x20, 0x91,
//         0x35, 0x2f, 0x59, 0x56, 0x02, 0x19, 0x99, 0x91, 0x11, 0x61, 0x07, 0x49, 0x5a, 0x5c, 0x4f,
//         0x71, 0x00,
//     ];

//     #[test]
//     fn test_address_round_trip() {
//         let value = ABIValue::Address(TEST_ADDRESS_BYTES);
//         let encoded = encode_address(&ABIType::Address, &value).unwrap();
//         let decoded = decode_address(&ABIType::Address, &encoded).unwrap();
//         assert_eq!(decoded, value);
//     }

//     #[test]
//     fn test_address_encoding() {
//         let value = ABIValue::Address(TEST_ADDRESS_BYTES);
//         let encoded = encode_address(&ABIType::Address, &value).unwrap();
//         assert_eq!(encoded, TEST_ADDRESS_BYTES.to_vec());
//     }

//     #[test]
//     fn test_address_decoding() {
//         let bytes = TEST_ADDRESS_BYTES.to_vec();
//         let decoded = decode_address(&ABIType::Address, &bytes).unwrap();
//         assert_eq!(decoded, ABIValue::Address(TEST_ADDRESS_BYTES));
//     }

//     #[test]
//     fn test_multiple_addresses_round_trip() {
//         let test_addresses = vec![
//             [0u8; 32],
//             [0xFFu8; 32],
//             TEST_ADDRESS_BYTES,
//             [
//                 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
//                 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x01, 0x02, 0x03, 0x04,
//                 0x05, 0x06, 0x07, 0x08,
//             ],
//         ];

//         for test_address in test_addresses {
//             let value = ABIValue::Address(test_address);
//             let encoded = encode_address(&ABIType::Address, &value).unwrap();
//             let decoded = decode_address(&ABIType::Address, &encoded).unwrap();
//             assert_eq!(decoded, value);
//         }
//     }

//     #[test]
//     fn test_encode_wrong_type() {
//         let value = ABIValue::String("not an address".to_string());
//         let result = encode_address(&ABIType::Address, &value);
//         assert!(result.is_err());
//         assert!(result
//             .unwrap_err()
//             .to_string()
//             .contains("Cannot encode value as address"));
//     }

//     #[test]
//     fn test_encode_wrong_abi_type() {
//         let value = ABIValue::Address(TEST_ADDRESS_BYTES);
//         let result = encode_address(&ABIType::String, &value);
//         assert!(result.is_err());
//         assert!(result.unwrap_err().to_string().contains("Expected Address"));
//     }

//     #[test]
//     fn test_decode_wrong_length_too_short() {
//         let bytes = vec![0u8; 31];
//         let result = decode_address(&ABIType::Address, &bytes);
//         assert!(result.is_err());
//         assert!(result
//             .unwrap_err()
//             .to_string()
//             .contains("Address byte string must be 32 bytes long"));
//     }

//     #[test]
//     fn test_decode_wrong_length_too_long() {
//         let bytes = vec![0u8; 33];
//         let result = decode_address(&ABIType::Address, &bytes);
//         assert!(result.is_err());
//         assert!(result
//             .unwrap_err()
//             .to_string()
//             .contains("Address byte string must be 32 bytes long"));
//     }

//     #[test]
//     fn test_decode_empty_bytes() {
//         let bytes = vec![];
//         let result = decode_address(&ABIType::Address, &bytes);
//         assert!(result.is_err());
//         assert!(result
//             .unwrap_err()
//             .to_string()
//             .contains("Address byte string must be 32 bytes long"));
//     }

//     #[test]
//     fn test_decode_wrong_abi_type() {
//         let bytes = TEST_ADDRESS_BYTES.to_vec();
//         let result = decode_address(&ABIType::String, &bytes);
//         assert!(result.is_err());
//         assert!(result.unwrap_err().to_string().contains("Expected Address"));
//     }
// }
