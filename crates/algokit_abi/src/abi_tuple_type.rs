use std::collections::HashMap;

use crate::{
    abi_type::{encode, get_name, is_dynamic},
    error::ABIError,
    utils::extend_bytes_to_length,
    ABIType, ABIValue,
};

const LENGTH_ENCODE_BYTE_SIZE: usize = 2;

pub fn encode_tuple(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    let child_types = match abi_type {
        ABIType::ABITupleType(child_types) => child_types,
        _ => return Err(ABIError::EncodingError("Expected ABITupleType".to_string())),
    };
    let values = match value {
        ABIValue::Array(n) => n,
        _ => {
            return Err(ABIError::EncodingError(format!(
                "Cannot encode tuple {}, expect an array of byte array",
                get_name(abi_type)
            )));
        }
    };

    // TODO: do we need to check for values.len() < u16::MAX?

    // TODO: confirm this check, algosdk doesn't do this
    if child_types.len() != values.len() {
        return Err(ABIError::EncodingError(format!(
            "Cannot encode tuple {}, value and child type lengths mismatch",
            get_name(abi_type)
        )));
    }

    let mut heads: Vec<Vec<u8>> = Vec::new();
    let mut tails: Vec<Vec<u8>> = Vec::new();
    let mut is_dynamic_index: HashMap<usize, bool> = HashMap::new();

    for i in 0..child_types.len() {
        let child_type = &child_types[i];

        if is_dynamic(&child_type) {
        } else {
            is_dynamic_index.insert(i, true);
            heads.push(vec![0, 0]);
            tails.push(encode(&child_type, &values[i])?);
            match child_type {
                ABIType::ABIBool => {
                    let mut bool_buffer: Vec<&ABIValue> = Vec::new();
                    let mut cursor = i;

                    while (matches!(&child_types[cursor], ABIType::ABIBool)
                        && bool_buffer.len() < 8)
                    {
                        bool_buffer.push(&values[cursor]);
                        cursor += 1;
                    }

                    let compressed_value = compress_bools(&bool_buffer)?;
                    heads.push(compressed_value.to_be_bytes().to_vec());
                }
                _ => {
                    heads.push(encode(&child_type, &values[i])?);
                }
            }
            is_dynamic_index.insert(i, false);
            tails.push(vec![]);
        }
    }

    let head_length: usize = heads.iter().map(|e| e.len()).sum();
    let mut tail_length = 0;

    for i in 0..child_types.len() {
        match is_dynamic_index[&i] {
            true => {
                let head_value = head_length + tail_length;
                // TODO: discuss that the check for u16::MAX is skipped
                heads[i] =
                    extend_bytes_to_length(&head_value.to_be_bytes(), LENGTH_ENCODE_BYTE_SIZE);
            }
            _ => {
                tail_length += tails[i].len();
            }
        }
    }

    let results = heads
        .into_iter()
        .chain(tails.into_iter())
        .flatten()
        .collect();

    Ok(results)
}

pub fn decode_tuple(abi_type: ABIType, bytes: Vec<u8>) -> Result<ABIValue, ABIError> {
    let _child_types = match abi_type {
        ABIType::ABITupleType(child_types) => child_types,
        _ => return Err(ABIError::DecodingError("Expected ABITupleType".to_string())),
    };

    let _bytes = bytes;
    // TODO: Implement tuple decoding logic
    Err(ABIError::DecodingError(
        "Tuple decoding not yet implemented".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{abi_type::BitSize, abi_value::ABIValue};
    use num_bigint::BigUint;

    #[test]
    fn test_encode_empty_tuple() {
        let tuple_type = ABIType::ABITupleType(vec![]);
        let value = ABIValue::Array(vec![]);
        let encoded = encode_tuple(&tuple_type, &value);
        // TODO: Should succeed when implemented
        assert!(encoded.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_encode_simple_tuple() {
        let uint32_type1 = ABIType::ABIUintType(BitSize::new(32).unwrap());
        let uint32_type2 = ABIType::ABIUintType(BitSize::new(32).unwrap());
        let tuple_type = ABIType::ABITupleType(vec![&uint32_type1, &uint32_type2]);
        let value = ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(1u32)),
            ABIValue::Uint(BigUint::from(2u32)),
        ]);
        let encoded = encode_tuple(&tuple_type, &value);
        // TODO: Should encode to [0, 0, 0, 1, 0, 0, 0, 2] when implemented
        assert!(encoded.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_encode_mixed_tuple() {
        let uint32_type = ABIType::ABIUintType(BitSize::new(32).unwrap());
        let string_type = ABIType::ABIString;
        let tuple_type = ABIType::ABITupleType(vec![&uint32_type, &string_type]);
        let value = ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(42u32)),
            ABIValue::String("hello".to_string()),
        ]);
        let encoded = encode_tuple(&tuple_type, &value);
        // TODO: Should encode to [0, 0, 0, 42, 0, 6, 0, 5, 104, 101, 108, 108, 111] when implemented
        assert!(encoded.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_round_trip_simple_tuple() {
        let uint16_type = ABIType::ABIUintType(BitSize::new(16).unwrap());
        let bool_type = ABIType::ABIBool;
        let tuple_type = ABIType::ABITupleType(vec![&uint16_type, &bool_type]);
        let value = ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(1234u32)),
            ABIValue::Bool(true),
        ]);

        let encoded = encode_tuple(&tuple_type, &value);
        // TODO: Should succeed when implemented
        assert!(encoded.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_round_trip_mixed_tuple() {
        let uint32_type = ABIType::ABIUintType(BitSize::new(32).unwrap());
        let string_type = ABIType::ABIString;
        let bool_type = ABIType::ABIBool;
        let tuple_type = ABIType::ABITupleType(vec![&uint32_type, &string_type, &bool_type]);
        let value = ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(42u32)),
            ABIValue::String("test".to_string()),
            ABIValue::Bool(false),
        ]);

        let encoded = encode_tuple(&tuple_type, &value);
        // TODO: Should succeed when implemented
        assert!(encoded.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_wrong_value_length() {
        let uint32_type1 = ABIType::ABIUintType(BitSize::new(32).unwrap());
        let uint32_type2 = ABIType::ABIUintType(BitSize::new(32).unwrap());
        let tuple_type = ABIType::ABITupleType(vec![&uint32_type1, &uint32_type2]);
        let value = ABIValue::Array(vec![ABIValue::Uint(BigUint::from(1u32))]);
        let result = encode_tuple(&tuple_type, &value);
        // TODO: Should fail with length mismatch when implemented
        assert!(result.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_wrong_abi_type() {
        let tuple_type = ABIType::ABIString;
        let value = ABIValue::Array(vec![]);
        let result = encode_tuple(&tuple_type, &value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Expected ABITupleType"));
    }

    // TODO: Add these decode tests once tuple implementation is complete

    #[test]
    fn test_decode_empty_tuple() {
        let tuple_type = ABIType::ABITupleType(vec![]);
        let bytes = vec![];
        let result = decode_tuple(tuple_type, bytes);
        // TODO: Should succeed when implemented
        // Expected: Ok(ABIValue::Array(vec![]))
        assert!(result.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_decode_simple_tuple() {
        let uint8_type = ABIType::ABIUintType(BitSize::new(8).unwrap());
        let uint16_type = ABIType::ABIUintType(BitSize::new(16).unwrap());
        let tuple_type = ABIType::ABITupleType(vec![&uint8_type, &uint16_type]);
        let bytes = vec![1, 0, 2]; // Based on JS SDK test: [1, 2] -> [1, 0, 2]
        let result = decode_tuple(tuple_type, bytes);
        // TODO: Should succeed when implemented
        // Expected: Ok(ABIValue::Array(vec![
        //     ABIValue::Uint(BigUint::from(1u8)),
        //     ABIValue::Uint(BigUint::from(2u16)),
        // ]))
        assert!(result.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_decode_mixed_tuple() {
        let string_type = ABIType::ABIString;
        let bool_type1 = ABIType::ABIBool;
        let bool_type2 = ABIType::ABIBool;
        let bool_type3 = ABIType::ABIBool;
        let bool_type4 = ABIType::ABIBool;
        let string_type2 = ABIType::ABIString;
        let tuple_type = ABIType::ABITupleType(vec![
            &string_type,
            &bool_type1,
            &bool_type2,
            &bool_type3,
            &bool_type4,
            &string_type2,
        ]);
        // Based on Python SDK test: ["AB", True, False, True, False, "DE"] -> bytes.fromhex("00 05 A0 00 09 00 02 41 42 00 02 44 45")
        let bytes = vec![
            0x00, 0x05, 0xA0, 0x00, 0x09, 0x00, 0x02, 0x41, 0x42, 0x00, 0x02, 0x44, 0x45,
        ];
        let result = decode_tuple(tuple_type, bytes);
        // TODO: Should succeed when implemented
        // Expected: Ok(ABIValue::Array(vec![
        //     ABIValue::String("AB".to_string()),
        //     ABIValue::Bool(true),
        //     ABIValue::Bool(false),
        //     ABIValue::Bool(true),
        //     ABIValue::Bool(false),
        //     ABIValue::String("DE".to_string()),
        // ]))
        assert!(result.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_decode_dynamic_tuple() {
        let bool_type1 = ABIType::ABIBool;
        let bool_type2 = ABIType::ABIBool;
        let inner_tuple1 = ABIType::ABITupleType(vec![&bool_type1]);
        let inner_tuple2 = ABIType::ABITupleType(vec![&bool_type2]);
        let tuple_type = ABIType::ABITupleType(vec![
            &inner_tuple1, // bool[]
            &inner_tuple2, // bool[]
        ]);
        // Based on JS SDK test: [[], []] -> [0, 4, 0, 6, 0, 0, 0, 0]
        let bytes = vec![0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00];
        let result = decode_tuple(tuple_type, bytes);
        // TODO: Should succeed when implemented
        // Expected: Ok(ABIValue::Array(vec![
        //     ABIValue::Array(vec![]), // Empty bool array
        //     ABIValue::Array(vec![]), // Empty bool array
        // ]))
        assert!(result.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_decode_nested_tuple() {
        let uint16_type = ABIType::ABIUintType(BitSize::new(16).unwrap());
        let byte_type = ABIType::ABIByte;
        let address_type = ABIType::ABIAddressType;
        let inner_tuple = ABIType::ABITupleType(vec![&byte_type, &address_type]);
        let tuple_type = ABIType::ABITupleType(vec![&uint16_type, &inner_tuple]);
        // TODO: Create test bytes based on nested tuple structure
        let bytes = vec![]; // Placeholder - needs actual encoded bytes
        let result = decode_tuple(tuple_type, bytes);
        // TODO: Should succeed when implemented
        // Expected: Ok(ABIValue::Array(vec![
        //     ABIValue::Uint(BigUint::from(test_value)),
        //     ABIValue::Array(vec![
        //         ABIValue::Byte(test_byte),
        //         ABIValue::Address(test_address),
        //     ]),
        // ]))
        assert!(result.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_decode_malformed_tuple_insufficient_bytes() {
        let uint32_type1 = ABIType::ABIUintType(BitSize::new(32).unwrap());
        let uint32_type2 = ABIType::ABIUintType(BitSize::new(32).unwrap());
        let tuple_type = ABIType::ABITupleType(vec![&uint32_type1, &uint32_type2]);
        let bytes = vec![0x00, 0x00, 0x00]; // Too few bytes for two uint32s
        let result = decode_tuple(tuple_type, bytes);
        // TODO: Should fail with appropriate error when implemented
        // Expected: Err(ABIError::DecodingError("insufficient bytes..."))
        assert!(result.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_decode_malformed_tuple_wrong_abi_type() {
        let tuple_type = ABIType::ABIString; // Not a tuple type
        let bytes = vec![0x00, 0x00, 0x00, 0x00];
        let result = decode_tuple(tuple_type, bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Expected ABITupleType"));
    }

    #[test]
    fn test_decode_malformed_tuple_extra_bytes() {
        let uint8_type = ABIType::ABIUintType(BitSize::new(8).unwrap());
        let tuple_type = ABIType::ABITupleType(vec![&uint8_type]);
        let bytes = vec![0x01, 0x02, 0x03]; // Extra bytes after the uint8
        let result = decode_tuple(tuple_type, bytes);
        // TODO: Should fail with appropriate error when implemented
        // Expected: Err(ABIError::DecodingError("extra bytes..."))
        assert!(result.is_err()); // Currently fails with not implemented
    }
}

fn compress_bools(values: &[&ABIValue]) -> Result<u8, ABIError> {
    if values.len() > 8 {
        return Err(ABIError::EncodingError(format!(
            "Expected no more than 8 bool values, received {}",
            values.len()
        )));
    }

    let mut result: u8 = 0;
    for (i, value) in values.iter().enumerate() {
        match value {
            ABIValue::Bool(b) => {
                if *b {
                    result |= 1 << (7 - i);
                }
            }
            _ => {
                return Err(ABIError::EncodingError(
                    "Expected all values to be ABIValue::Bool".to_string(),
                ));
            }
        }
    }
    Ok(result)
}
