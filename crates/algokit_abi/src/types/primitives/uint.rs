use num_bigint::BigUint;

use crate::{abi_type::ABIType, abi_value::ABIValue, error::ABIError, utils};

pub fn encode_uint(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::Uint(bit_size) => {
            let bit_size = bit_size.value();
            let value = match value {
                ABIValue::Uint(n) => n,
                _ => {
                    return Err(ABIError::EncodingError(format!(
                        "Cannot encode value as uint{}: expected number",
                        bit_size
                    )));
                }
            };

            if value >= &BigUint::from(2u64).pow(bit_size as u32) {
                return Err(ABIError::EncodingError(format!(
                    "{} is too big to fit in uint{}",
                    value, bit_size
                )));
            }

            Ok(utils::extend_bytes_to_length(
                &value.to_bytes_be(),
                (bit_size / 8) as usize,
            ))
        }
        _ => Err(ABIError::EncodingError("Expected UintType".to_string())),
    }
}

pub fn decode_uint(abi_type: &ABIType, bytes: &[u8]) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::Uint(bit_size) => {
            let bit_size = bit_size.value();
            let expected_len = (bit_size / 8) as usize;
            if bytes.len() != expected_len {
                return Err(ABIError::DecodingError(format!(
                    "Invalid byte array length, expected {} bytes, got {}",
                    expected_len,
                    bytes.len()
                )));
            }

            Ok(ABIValue::Uint(BigUint::from_bytes_be(bytes)))
        }
        _ => Err(ABIError::DecodingError("Expected UintType".to_string())),
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::abi_type::BitSize;

//     use super::*;
//     use num_bigint::BigUint;

//     #[test]
//     fn test_uint_size_validation() {
//         // Test standard uint sizes
//         let valid_sizes = vec![8, 16, 32, 64, 128, 256, 512];

//         for size in valid_sizes {
//             let abi_type = ABIType::Uint(BitSize::new(size).unwrap());
//             let value = ABIValue::Uint(BigUint::from(1u32));
//             let result = encode_uint(&abi_type, &value);
//             assert!(result.is_ok(), "uint{} should be valid", size);
//         }
//     }

//     #[test]
//     fn test_uint_boundary_values() {
//         // Test uint8 boundaries
//         let abi_type = ABIType::Uint(BitSize::new(8).unwrap());

//         // Test 0 (minimum)
//         let value = ABIValue::Uint(BigUint::from(0u8));
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded, vec![0]);
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);

//         let abi_type = ABIType::Uint(BitSize::new(8).unwrap());
//         // Test 255 (maximum)
//         let value = ABIValue::Uint(BigUint::from(255u8));
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded, vec![255]);
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);

//         let abi_type = ABIType::Uint(BitSize::new(8).unwrap());
//         // Test 256 (should fail)
//         let value = ABIValue::Uint(BigUint::from(256u16));
//         let result = encode_uint(&abi_type, &value);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_uint16_boundary_values() {
//         let abi_type = ABIType::Uint(BitSize::new(16).unwrap());

//         // Test 0 (minimum)
//         let value = ABIValue::Uint(BigUint::from(0u16));
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded, vec![0, 0]);
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);

//         let abi_type = ABIType::Uint(BitSize::new(16).unwrap());
//         // Test 65535 (maximum)
//         let value = ABIValue::Uint(BigUint::from(65535u16));
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded, vec![255, 255]);
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);

//         let abi_type = ABIType::Uint(BitSize::new(16).unwrap());
//         // Test 65536 (should fail)
//         let value = ABIValue::Uint(BigUint::from(65536u32));
//         let result = encode_uint(&abi_type, &value);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_uint32_boundary_values() {
//         let abi_type = ABIType::Uint(BitSize::new(32).unwrap());

//         // Test 0 (minimum)
//         let value = ABIValue::Uint(BigUint::from(0u32));
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded, vec![0, 0, 0, 0]);
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);

//         let abi_type = ABIType::Uint(BitSize::new(32).unwrap());
//         // Test 4294967295 (maximum)
//         let value = ABIValue::Uint(BigUint::from(4294967295u32));
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded, vec![255, 255, 255, 255]);
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);

//         let abi_type = ABIType::Uint(BitSize::new(32).unwrap());
//         // Test 4294967296 (should fail)
//         let value = ABIValue::Uint(BigUint::from(4294967296u64));
//         let result = encode_uint(&abi_type, &value);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_uint64_boundary_values() {
//         let abi_type = ABIType::Uint(BitSize::new(64).unwrap());

//         // Test 0 (minimum)
//         let value = ABIValue::Uint(BigUint::from(0u64));
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded, vec![0, 0, 0, 0, 0, 0, 0, 0]);
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);

//         let abi_type = ABIType::Uint(BitSize::new(64).unwrap());
//         // Test 18446744073709551615 (maximum)
//         let value = ABIValue::Uint(BigUint::from(2u64).pow(64) - 1u64);
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded, vec![255, 255, 255, 255, 255, 255, 255, 255]);
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);

//         let abi_type = ABIType::Uint(BitSize::new(64).unwrap());
//         // Test overflow (should fail)
//         let value = ABIValue::Uint(BigUint::from(2u128).pow(64));
//         let result = encode_uint(&abi_type, &value);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_uint256_large_values() {
//         let abi_type = ABIType::Uint(BitSize::new(256).unwrap());

//         // Test very large value
//         let large_value = BigUint::from(2u64).pow(255) - 1u64;
//         let value = ABIValue::Uint(large_value);
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded.len(), 32); // 256 bits = 32 bytes
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);
//     }

//     #[test]
//     fn test_uint_wrong_type() {
//         let abi_type = ABIType::Uint(BitSize::new(32).unwrap());

//         // Test wrong value types
//         let wrong_values = vec![
//             ABIValue::Bool(true),
//             ABIValue::String("123".to_string()),
//             ABIValue::Address([0u8; 32]),
//             ABIValue::Array(vec![ABIValue::Uint(BigUint::from(1u32))]),
//         ];

//         for value in wrong_values {
//             let result = encode_uint(&abi_type, &value);
//             assert!(result.is_err());
//         }
//     }

//     #[test]
//     fn test_uint_decode_wrong_length() {
//         let test_cases = vec![
//             (ABIType::Uint(BitSize::new(8).unwrap()), vec![0u8, 0]), // 2 bytes for uint8
//             (ABIType::Uint(BitSize::new(16).unwrap()), vec![0u8]),   // 1 byte for uint16
//             (ABIType::Uint(BitSize::new(32).unwrap()), vec![0u8, 0, 0]), // 3 bytes for uint32
//             (ABIType::Uint(BitSize::new(64).unwrap()), vec![0u8; 7]), // 7 bytes for uint64
//         ];

//         for (abi_type, wrong_bytes) in test_cases {
//             let result = decode_uint(&abi_type, &wrong_bytes);
//             assert!(result.is_err());
//         }
//     }

//     #[test]
//     fn test_uint_decode_wrong_abi_type() {
//         let bytes = vec![0u8, 0, 0, 42];
//         let wrong_types = vec![
//             ABIType::String,
//             ABIType::Bool,
//             ABIType::Address,
//             ABIType::Byte,
//         ];

//         for abi_type in wrong_types {
//             let result = decode_uint(&abi_type, &bytes.clone());
//             assert!(result.is_err());
//         }
//     }

//     #[test]
//     fn test_uint_encoding_deterministic() {
//         let test_cases = vec![
//             (
//                 ABIType::Uint(BitSize::new(8).unwrap()),
//                 ABIValue::Uint(BigUint::from(42u8)),
//             ),
//             (
//                 ABIType::Uint(BitSize::new(16).unwrap()),
//                 ABIValue::Uint(BigUint::from(1234u16)),
//             ),
//             (
//                 ABIType::Uint(BitSize::new(32).unwrap()),
//                 ABIValue::Uint(BigUint::from(987654321u32)),
//             ),
//             (
//                 ABIType::Uint(BitSize::new(64).unwrap()),
//                 ABIValue::Uint(BigUint::from(1234567890123456789u64)),
//             ),
//         ];

//         for (abi_type, value) in test_cases {
//             let encoded1 = encode_uint(&abi_type, &value).unwrap();
//             let encoded2 = encode_uint(&abi_type, &value).unwrap();
//             assert_eq!(encoded1, encoded2, "Encoding should be deterministic");
//         }
//     }

//     #[test]
//     fn test_uint_odd_sizes() {
//         // Test non-standard but valid sizes
//         let odd_sizes = vec![
//             24, 40, 48, 56, 72, 80, 88, 96, 104, 112, 120, 136, 144, 152, 160, 168, 176, 184, 192,
//             200, 208, 216, 224, 232, 240, 248,
//         ];

//         for size in odd_sizes {
//             let abi_type = ABIType::Uint(BitSize::new(size).unwrap());
//             let value = ABIValue::Uint(BigUint::from(12345u32));
//             let encoded = encode_uint(&abi_type, &value).unwrap();
//             assert_eq!(encoded.len(), (size / 8) as usize);
//             let decoded = decode_uint(&abi_type, &encoded).unwrap();
//             assert_eq!(decoded, value);
//         }
//     }

//     #[test]
//     fn test_uint_max_size() {
//         let abi_type = ABIType::Uint(BitSize::new(512).unwrap());
//         let value = ABIValue::Uint(BigUint::from(1u64) << 511); // 2^511
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded.len(), 64); // 512 bits = 64 bytes
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);
//     }

//     #[test]
//     fn test_uint_leading_zeros() {
//         // Test that leading zeros are handled correctly
//         let abi_type = ABIType::Uint(BitSize::new(32).unwrap());
//         let value = ABIValue::Uint(BigUint::from(1u32));
//         let encoded = encode_uint(&abi_type, &value).unwrap();
//         assert_eq!(encoded, vec![0, 0, 0, 1]); // Should have leading zeros
//         let decoded = decode_uint(&abi_type, &encoded).unwrap();
//         assert_eq!(decoded, value);
//     }

//     #[test]
//     fn test_uint_power_of_two_boundaries() {
//         // Test powers of 2 that are at the boundaries
//         let test_cases = vec![
//             (
//                 ABIType::Uint(BitSize::new(8).unwrap()),
//                 7,
//                 BigUint::from(2u8).pow(7),
//             ),
//             (
//                 ABIType::Uint(BitSize::new(16).unwrap()),
//                 15,
//                 BigUint::from(2u16).pow(15),
//             ),
//             (
//                 ABIType::Uint(BitSize::new(32).unwrap()),
//                 31,
//                 BigUint::from(2u32).pow(31),
//             ),
//             (
//                 ABIType::Uint(BitSize::new(64).unwrap()),
//                 63,
//                 BigUint::from(2u64).pow(63),
//             ),
//         ];

//         for (abi_type, _power, expected_value) in test_cases {
//             let value = ABIValue::Uint(expected_value);
//             let encoded = encode_uint(&abi_type, &value).unwrap();
//             let decoded = decode_uint(&abi_type, &encoded).unwrap();
//             assert_eq!(decoded, value);
//         }
//     }
// }
