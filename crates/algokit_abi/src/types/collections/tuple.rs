use std::collections::HashMap;

use crate::{
    abi_type::{encode, get_size, is_dynamic},
    common::{BOOL_FALSE_BYTE, BOOL_TRUE_BYTE, LENGTH_ENCODE_BYTE_SIZE},
    decode,
    error::ABIError,
    ABIType, ABIValue,
};

struct Segment {
    left: u16,
    right: u16,
}

pub fn encode_tuple(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    let child_types = match abi_type {
        ABIType::Tuple(child_types) => child_types.iter().map(|b| b.as_ref()).collect::<Vec<_>>(),
        _ => return Err(ABIError::EncodingError("Expected TupleType".to_string())),
    };

    let values = match value {
        ABIValue::Array(n) => n,
        _ => {
            return Err(ABIError::EncodingError(format!(
                "Cannot encode tuple {}, expect an array of byte array",
                abi_type
            )));
        }
    };

    encode_abi_types(&child_types, values)
}

// TODO: better name
pub fn encode_abi_types(abi_types: &[&ABIType], values: &[ABIValue]) -> Result<Vec<u8>, ABIError> {
    // TODO: do we need to check for values.len() < u16::MAX?

    // TODO: confirm this check, algosdk doesn't do this
    if abi_types.len() != values.len() {
        return Err(ABIError::EncodingError(
            "Cannot encode, values and types lengths mismatch".to_string(),
        ));
    }

    let mut heads: Vec<Vec<u8>> = Vec::new();
    let mut tails: Vec<Vec<u8>> = Vec::new();
    let mut is_dynamic_index: HashMap<usize, bool> = HashMap::new();

    let mut i = 0;
    while i < abi_types.len() {
        let child_type = &abi_types[i];

        if is_dynamic(child_type) {
            is_dynamic_index.insert(i, true);
            heads.push(vec![0, 0]);
            tails.push(encode(child_type, &values[i])?);
        } else {
            match child_type {
                ABIType::Bool => {
                    let sequence_end_index = find_bool_sequence_end(abi_types, i);
                    let bool_values = &values[i..sequence_end_index];
                    heads.push(compress_bools(bool_values)?.to_be_bytes().to_vec());

                    i = sequence_end_index;
                }
                _ => {
                    heads.push(encode(child_type, &values[i])?);
                }
            }
            is_dynamic_index.insert(i, false);
            tails.push(vec![]);
        }

        i += 1;
    }

    let head_length: usize = heads.iter().map(|e| e.len()).sum();
    let mut tail_length = 0;

    for i in 0..abi_types.len() {
        match is_dynamic_index.get(&i) {
            Some(true) => {
                let head_value = head_length + tail_length;
                let head_value: u16 = u16::try_from(head_length + tail_length).map_err(|_| {
                    ABIError::EncodingError(format!("Value {} cannot fit in u16", head_value))
                })?;
                heads[i] = head_value.to_be_bytes().to_vec();
            }
            _ => {
                tail_length += tails[i].len();
            }
        }
    }

    let results = heads.into_iter().chain(tails).flatten().collect();

    Ok(results)
}

pub fn decode_tuple(abi_type: &ABIType, bytes: &[u8]) -> Result<ABIValue, ABIError> {
    let child_types = match abi_type {
        ABIType::Tuple(child_types) => child_types.iter().map(|b| b.as_ref()).collect::<Vec<_>>(),
        _ => return Err(ABIError::DecodingError("Expected TupleType".to_string())),
    };

    decode_abi_types(&child_types, bytes)
}

// TODO: better name
pub fn decode_abi_types(abi_types: &[&ABIType], bytes: &[u8]) -> Result<ABIValue, ABIError> {
    let value_partitions = extract_values(abi_types, bytes)?;

    let mut values: Vec<ABIValue> = Vec::new();
    for i in 0..abi_types.len() {
        let child_type = &abi_types[i];
        let value_partition = &value_partitions[i];
        let child_type_value = decode(child_type, value_partition)?;
        values.push(child_type_value);
    }

    Ok(ABIValue::Array(values))
}

fn compress_bools(values: &[ABIValue]) -> Result<u8, ABIError> {
    if values.len() > 8 {
        return Err(ABIError::EncodingError(format!(
            "Expected no more than 8 bool values, received {}",
            values.len()
        )));
    }

    let mut result: u8 = 0;
    // TODO: check .iter().enumerate()
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

fn extract_values(abi_types: &[&ABIType], bytes: &[u8]) -> Result<Vec<Vec<u8>>, ABIError> {
    let mut dynamic_segments: Vec<Segment> = Vec::new();
    let mut value_partitions: Vec<Option<Vec<u8>>> = Vec::new();
    let mut bytes_cursor: usize = 0;

    let mut i = 0;
    while i < abi_types.len() {
        let child_type = abi_types[i];

        if is_dynamic(child_type) {
            if bytes[bytes_cursor..].len() < LENGTH_ENCODE_BYTE_SIZE {
                return Err(ABIError::DecodingError(
                    "Dynamic type in tuple is too short to be decoded".to_string(),
                ));
            }

            let dynamic_index = u16::from_be_bytes([bytes[bytes_cursor], bytes[bytes_cursor + 1]]);
            if let Some(last_segment) = dynamic_segments.last_mut() {
                if dynamic_index < last_segment.left {
                    return Err(ABIError::DecodingError(
                        "dynamic index segment miscalculation: left is greater than right index"
                            .to_string(),
                    ));
                }
                last_segment.right = dynamic_index;
            }

            dynamic_segments.push(Segment {
                left: dynamic_index,
                right: 0, // TODO: check this logic, it is -1 in algosdk
            });
            value_partitions.push(None);
            bytes_cursor += LENGTH_ENCODE_BYTE_SIZE;
        } else {
            match child_type {
                ABIType::Bool => {
                    let sequence_end_index = find_bool_sequence_end(abi_types, i);

                    for j in 0..sequence_end_index - i {
                        let bool_mask: u8 = BOOL_TRUE_BYTE >> j;
                        if bytes[bytes_cursor] & bool_mask > 0 {
                            value_partitions.push(Some(vec![BOOL_TRUE_BYTE]));
                        } else {
                            value_partitions.push(Some(vec![BOOL_FALSE_BYTE]));
                        }
                    }

                    i = sequence_end_index;
                    bytes_cursor += 1;
                }
                _ => {
                    let child_type_len = get_size(child_type)?;
                    value_partitions.push(Some(
                        bytes[bytes_cursor..bytes_cursor + child_type_len].to_vec(),
                    ));
                    bytes_cursor += child_type_len;
                }
            }
        }
        i += 1;
    }

    if bytes_cursor < bytes.len() {
        return Err(ABIError::DecodingError(
            "Input bytes not fully consumed".to_string(),
        ));
    }
    if let Some(last_segment) = dynamic_segments.last_mut() {
        last_segment.right = bytes.len() as u16;
    }

    for i in 0..dynamic_segments.len() {
        let segment = &dynamic_segments[i];
        if segment.left > segment.right {
            return Err(ABIError::DecodingError(
                "Dynamic segment should display a [l, r] space with l <= r".to_string(),
            ));
        }
        if i != dynamic_segments.len() - 1 && segment.right != dynamic_segments[i + 1].left {
            return Err(ABIError::DecodingError(
                "Dynamic segments should be consecutive".to_string(),
            ));
        }
    }

    let mut segment_index: usize = 0;
    for i in 0..abi_types.len() {
        let child_type = &abi_types[i];
        if is_dynamic(child_type) {
            value_partitions[i] = Some(
                bytes[dynamic_segments[segment_index].left as usize
                    ..dynamic_segments[segment_index].right as usize]
                    .to_vec(),
            );
            segment_index += 1;
        }
    }

    // Check that all items in value_partitions are Some and convert to Vec<Vec<u8>>
    let value_partitions: Vec<Vec<u8>> = value_partitions
        .into_iter()
        .enumerate()
        .map(|(i, partition)| {
            partition.ok_or_else(|| {
                ABIError::DecodingError(format!("Value partition at index {} is None", i))
            })
        })
        .collect::<Result<Vec<Vec<u8>>, ABIError>>()?;

    Ok(value_partitions)
}

pub fn find_bool_sequence_end<T>(child_types: &[T], current_index: usize) -> usize
where
    T: AsRef<ABIType>,
{
    let mut cursor: usize = current_index;
    loop {
        match child_types[cursor].as_ref() {
            ABIType::Bool => {
                if cursor - current_index == 8 || cursor == child_types.len() - 1 {
                    return cursor;
                }
                cursor += 1;
            }
            _ => {
                return cursor - 1;
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{abi_type::BitSize, abi_value::ABIValue};
//     use num_bigint::BigUint;

//     #[test]
//     fn test_encode_empty_tuple() {
//         let tuple_type = ABIType::Tuple(vec![]);
//         let value = ABIValue::Array(vec![]);
//         let encoded = encode_tuple(&tuple_type, &value).expect("Failed to encode");
//         assert_eq!(encoded, vec![]);
//     }

//     #[test]
//     fn test_encode_simple_tuple() {
//         let uint32_type1 = ABIType::Uint(BitSize::new(32).unwrap());
//         let uint32_type2 = ABIType::Uint(BitSize::new(32).unwrap());
//         let tuple_type = ABIType::Tuple(vec![&uint32_type1, &uint32_type2]);
//         let value = ABIValue::Array(vec![
//             ABIValue::Uint(BigUint::from(1u32)),
//             ABIValue::Uint(BigUint::from(2u32)),
//         ]);
//         let encoded = encode_tuple(&tuple_type, &value).expect("Failed to encode");
//         assert_eq!(encoded, vec![0, 0, 0, 1, 0, 0, 0, 2]);
//     }

//     #[test]
//     fn test_encode_mixed_tuple() {
//         let uint32_type = ABIType::Uint(BitSize::new(32).unwrap());
//         let string_type = ABIType::String;
//         let tuple_type = ABIType::Tuple(vec![&uint32_type, &string_type]);
//         let value = ABIValue::Array(vec![
//             ABIValue::Uint(BigUint::from(42u32)),
//             ABIValue::String("hello".to_string()),
//         ]);
//         let encoded = encode_tuple(&tuple_type, &value).expect("Failed to encode");
//         assert_eq!(
//             encoded,
//             vec![0, 0, 0, 42, 0, 6, 0, 5, 104, 101, 108, 108, 111]
//         );
//     }

//     #[test]
//     fn test_round_trip_simple_tuple() {
//         let uint16_type = ABIType::Uint(BitSize::new(16).unwrap());
//         let bool_type = ABIType::Bool;
//         let tuple_type = ABIType::Tuple(vec![&uint16_type, &bool_type]);
//         let value = ABIValue::Array(vec![
//             ABIValue::Uint(BigUint::from(1234u32)),
//             ABIValue::Bool(true),
//         ]);

//         let encoded = encode_tuple(&tuple_type, &value).expect("Failed to encode");
//         let decoded = decode_tuple(&tuple_type, &encoded).expect("Failed to decode");

//         assert_eq!(decoded, value);
//     }

//     #[test]
//     fn test_round_trip_mixed_tuple() {
//         let uint32_type = ABIType::Uint(BitSize::new(32).unwrap());
//         let string_type = ABIType::String;
//         let bool_type = ABIType::Bool;
//         let tuple_type = ABIType::Tuple(vec![&uint32_type, &string_type, &bool_type]);
//         let value = ABIValue::Array(vec![
//             ABIValue::Uint(BigUint::from(42u32)),
//             ABIValue::String("test".to_string()),
//             ABIValue::Bool(false),
//         ]);

//         let encoded = encode_tuple(&tuple_type, &value).expect("Failed to encode");
//         let decoded = decode_tuple(&tuple_type, &encoded).expect("Failed to decode");

//         assert_eq!(decoded, value);
//     }

//     #[test]
//     fn test_wrong_value_length() {
//         let uint32_type1 = ABIType::Uint(BitSize::new(32).unwrap());
//         let uint32_type2 = ABIType::Uint(BitSize::new(32).unwrap());
//         let tuple_type = ABIType::Tuple(vec![&uint32_type1, &uint32_type2]);
//         let value = ABIValue::Array(vec![ABIValue::Uint(BigUint::from(1u32))]);
//         let result = encode_tuple(&tuple_type, &value);
//         // TODO: Should fail with length mismatch when implemented
//         assert!(result.is_err()); // Currently fails with not implemented
//     }

//     #[test]
//     fn test_wrong_abi_type() {
//         let tuple_type = ABIType::String;
//         let value = ABIValue::Array(vec![]);
//         let result = encode_tuple(&tuple_type, &value);
//         assert!(result.is_err());
//         assert!(result
//             .unwrap_err()
//             .to_string()
//             .contains("Expected TupleType"));
//     }

//     // TODO: Add these decode tests once tuple implementation is complete

//     #[test]
//     fn test_decode_empty_tuple() {
//         let tuple_type = ABIType::Tuple(vec![]);
//         let bytes = vec![];
//         let result = decode_tuple(&tuple_type, &bytes);
//         // TODO: Should succeed when implemented
//         // Expected: Ok(ABIValue::Array(vec![]))
//         assert!(result.is_err()); // Currently fails with not implemented
//     }

//     #[test]
//     fn test_decode_simple_tuple() {
//         let uint8_type = ABIType::Uint(BitSize::new(8).unwrap());
//         let uint16_type = ABIType::Uint(BitSize::new(16).unwrap());
//         let tuple_type = ABIType::Tuple(vec![&uint8_type, &uint16_type]);
//         let bytes = vec![1, 0, 2]; // Based on JS SDK test: [1, 2] -> [1, 0, 2]
//         let result = decode_tuple(&tuple_type, &bytes);
//         // TODO: Should succeed when implemented
//         // Expected: Ok(ABIValue::Array(vec![
//         //     ABIValue::Uint(BigUint::from(1u8)),
//         //     ABIValue::Uint(BigUint::from(2u16)),
//         // ]))
//         assert!(result.is_err()); // Currently fails with not implemented
//     }

//     #[test]
//     fn test_decode_mixed_tuple() {
//         let string_type = ABIType::String;
//         let bool_type1 = ABIType::Bool;
//         let bool_type2 = ABIType::Bool;
//         let bool_type3 = ABIType::Bool;
//         let bool_type4 = ABIType::Bool;
//         let string_type2 = ABIType::String;
//         let tuple_type = ABIType::Tuple(vec![
//             &string_type,
//             &bool_type1,
//             &bool_type2,
//             &bool_type3,
//             &bool_type4,
//             &string_type2,
//         ]);
//         // Based on Python SDK test: ["AB", True, False, True, False, "DE"] -> bytes.fromhex("00 05 A0 00 09 00 02 41 42 00 02 44 45")
//         let bytes = vec![
//             0x00, 0x05, 0xA0, 0x00, 0x09, 0x00, 0x02, 0x41, 0x42, 0x00, 0x02, 0x44, 0x45,
//         ];
//         let result = decode_tuple(&tuple_type, &bytes);
//         // TODO: Should succeed when implemented
//         // Expected: Ok(ABIValue::Array(vec![
//         //     ABIValue::String("AB".to_string()),
//         //     ABIValue::Bool(true),
//         //     ABIValue::Bool(false),
//         //     ABIValue::Bool(true),
//         //     ABIValue::Bool(false),
//         //     ABIValue::String("DE".to_string()),
//         // ]))
//         assert!(result.is_err()); // Currently fails with not implemented
//     }

//     #[test]
//     fn test_decode_dynamic_tuple() {
//         let bool_type1 = ABIType::Bool;
//         let bool_type2 = ABIType::Bool;
//         let inner_tuple1 = ABIType::Tuple(vec![&bool_type1]);
//         let inner_tuple2 = ABIType::Tuple(vec![&bool_type2]);
//         let tuple_type = ABIType::Tuple(vec![
//             &inner_tuple1, // bool[]
//             &inner_tuple2, // bool[]
//         ]);
//         // Based on JS SDK test: [[], []] -> [0, 4, 0, 6, 0, 0, 0, 0]
//         let bytes = vec![0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00];
//         let result = decode_tuple(&tuple_type, &bytes);
//         // TODO: Should succeed when implemented
//         // Expected: Ok(ABIValue::Array(vec![
//         //     ABIValue::Array(vec![]), // Empty bool array
//         //     ABIValue::Array(vec![]), // Empty bool array
//         // ]))
//         assert!(result.is_err()); // Currently fails with not implemented
//     }

//     #[test]
//     fn test_decode_nested_tuple() {
//         let uint16_type = ABIType::Uint(BitSize::new(16).unwrap());
//         let byte_type = ABIType::Byte;
//         let address_type = ABIType::Address;
//         let inner_tuple = ABIType::Tuple(vec![&byte_type, &address_type]);
//         let tuple_type = ABIType::Tuple(vec![&uint16_type, &inner_tuple]);
//         // TODO: Create test bytes based on nested tuple structure
//         let bytes = vec![]; // Placeholder - needs actual encoded bytes
//         let result = decode_tuple(&tuple_type, &bytes);
//         // TODO: Should succeed when implemented
//         // Expected: Ok(ABIValue::Array(vec![
//         //     ABIValue::Uint(BigUint::from(test_value)),
//         //     ABIValue::Array(vec![
//         //         ABIValue::Byte(test_byte),
//         //         ABIValue::Address(test_address),
//         //     ]),
//         // ]))
//         assert!(result.is_err()); // Currently fails with not implemented
//     }

//     #[test]
//     fn test_decode_malformed_tuple_insufficient_bytes() {
//         let uint32_type1 = ABIType::Uint(BitSize::new(32).unwrap());
//         let uint32_type2 = ABIType::Uint(BitSize::new(32).unwrap());
//         let tuple_type = ABIType::Tuple(vec![&uint32_type1, &uint32_type2]);
//         let bytes = vec![0x00, 0x00, 0x00]; // Too few bytes for two uint32s
//         let result = decode_tuple(&tuple_type, &bytes);
//         // TODO: Should fail with appropriate error when implemented
//         // Expected: Err(ABIError::DecodingError("insufficient bytes..."))
//         assert!(result.is_err()); // Currently fails with not implemented
//     }

//     #[test]
//     fn test_decode_malformed_tuple_wrong_abi_type() {
//         let tuple_type = ABIType::String; // Not a tuple type
//         let bytes = vec![0x00, 0x00, 0x00, 0x00];
//         let result = decode_tuple(&tuple_type, &bytes);
//         assert!(result.is_err());
//         assert!(result
//             .unwrap_err()
//             .to_string()
//             .contains("Expected TupleType"));
//     }

//     #[test]
//     fn test_decode_malformed_tuple_extra_bytes() {
//         let uint8_type = ABIType::Uint(BitSize::new(8).unwrap());
//         let tuple_type = ABIType::Tuple(vec![&uint8_type]);
//         let bytes = vec![0x01, 0x02, 0x03]; // Extra bytes after the uint8
//         let result = decode_tuple(&tuple_type, &bytes);
//         // TODO: Should fail with appropriate error when implemented
//         // Expected: Err(ABIError::DecodingError("extra bytes..."))
//         assert!(result.is_err()); // Currently fails with not implemented
//     }
// }
