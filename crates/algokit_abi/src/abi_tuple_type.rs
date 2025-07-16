use crate::{error::ABIError, ABIType, ABIValue};

// TODO: Implement tuple encoding/decoding
pub fn encode_tuple(abi_type: ABIType, value: ABIValue) -> Result<Vec<u8>, ABIError> {
    let _child_types = match abi_type {
        ABIType::ABITupleType(child_types) => child_types,
        _ => return Err(ABIError::EncodingError("Expected ABITupleType".to_string())),
    };
    let _values = match value {
        ABIValue::Array(n) => n,
        _ => {
            return Err(ABIError::EncodingError(
                "Cannot encode value as tuple: expected an array".to_string(),
            ));
        }
    };

    // TODO: Implement tuple encoding logic
    Err(ABIError::EncodingError(
        "Tuple encoding not yet implemented".to_string(),
    ))
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
    use crate::abi_value::ABIValue;
    use num_bigint::BigUint;

    #[test]
    fn test_encode_empty_tuple() {
        let tuple_type = ABIType::ABITupleType(vec![]);
        let value = ABIValue::Array(vec![]);
        let encoded = encode_tuple(tuple_type, value);
        // TODO: Should succeed when implemented
        assert!(encoded.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_encode_simple_tuple() {
        let tuple_type =
            ABIType::ABITupleType(vec![ABIType::ABIUintType(32), ABIType::ABIUintType(32)]);
        let value = ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(1u32)),
            ABIValue::Uint(BigUint::from(2u32)),
        ]);
        let encoded = encode_tuple(tuple_type, value);
        // TODO: Should encode to [0, 0, 0, 1, 0, 0, 0, 2] when implemented
        assert!(encoded.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_encode_mixed_tuple() {
        let tuple_type =
            ABIType::ABITupleType(vec![ABIType::ABIUintType(32), ABIType::ABIStringType]);
        let value = ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(42u32)),
            ABIValue::String("hello".to_string()),
        ]);
        let encoded = encode_tuple(tuple_type, value);
        // TODO: Should encode to [0, 0, 0, 42, 0, 6, 0, 5, 104, 101, 108, 108, 111] when implemented
        assert!(encoded.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_round_trip_simple_tuple() {
        let tuple_type =
            ABIType::ABITupleType(vec![ABIType::ABIUintType(16), ABIType::ABIBoolType]);
        let value = ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(1234u32)),
            ABIValue::Bool(true),
        ]);

        let encoded = encode_tuple(tuple_type.clone(), value.clone());
        // TODO: Should succeed when implemented
        assert!(encoded.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_round_trip_mixed_tuple() {
        let tuple_type = ABIType::ABITupleType(vec![
            ABIType::ABIUintType(32),
            ABIType::ABIStringType,
            ABIType::ABIBoolType,
        ]);
        let value = ABIValue::Array(vec![
            ABIValue::Uint(BigUint::from(42u32)),
            ABIValue::String("test".to_string()),
            ABIValue::Bool(false),
        ]);

        let encoded = encode_tuple(tuple_type.clone(), value.clone());
        // TODO: Should succeed when implemented
        assert!(encoded.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_wrong_value_length() {
        let tuple_type =
            ABIType::ABITupleType(vec![ABIType::ABIUintType(32), ABIType::ABIUintType(32)]);
        let value = ABIValue::Array(vec![ABIValue::Uint(BigUint::from(1u32))]);
        let result = encode_tuple(tuple_type, value);
        // TODO: Should fail with length mismatch when implemented
        assert!(result.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_wrong_abi_type() {
        let tuple_type = ABIType::ABIStringType;
        let value = ABIValue::Array(vec![]);
        let result = encode_tuple(tuple_type, value);
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
        let tuple_type =
            ABIType::ABITupleType(vec![ABIType::ABIUintType(8), ABIType::ABIUintType(16)]);
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
        let tuple_type = ABIType::ABITupleType(vec![
            ABIType::ABIStringType,
            ABIType::ABIBoolType,
            ABIType::ABIBoolType,
            ABIType::ABIBoolType,
            ABIType::ABIBoolType,
            ABIType::ABIStringType,
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
        let tuple_type = ABIType::ABITupleType(vec![
            ABIType::ABITupleType(vec![ABIType::ABIBoolType]), // bool[]
            ABIType::ABITupleType(vec![ABIType::ABIBoolType]), // bool[]
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
        let tuple_type = ABIType::ABITupleType(vec![
            ABIType::ABIUintType(16),
            ABIType::ABITupleType(vec![ABIType::ABIByteType, ABIType::ABIAddressType]),
        ]);
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
        let tuple_type =
            ABIType::ABITupleType(vec![ABIType::ABIUintType(32), ABIType::ABIUintType(32)]);
        let bytes = vec![0x00, 0x00, 0x00]; // Too few bytes for two uint32s
        let result = decode_tuple(tuple_type, bytes);
        // TODO: Should fail with appropriate error when implemented
        // Expected: Err(ABIError::DecodingError("insufficient bytes..."))
        assert!(result.is_err()); // Currently fails with not implemented
    }

    #[test]
    fn test_decode_malformed_tuple_wrong_abi_type() {
        let tuple_type = ABIType::ABIStringType; // Not a tuple type
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
        let tuple_type = ABIType::ABITupleType(vec![ABIType::ABIUintType(8)]);
        let bytes = vec![0x01, 0x02, 0x03]; // Extra bytes after the uint8
        let result = decode_tuple(tuple_type, bytes);
        // TODO: Should fail with appropriate error when implemented
        // Expected: Err(ABIError::DecodingError("extra bytes..."))
        assert!(result.is_err()); // Currently fails with not implemented
    }
}
