use crate::{
    abi_address_type::encode_address, abi_bool_type::encode_bool, abi_byte_type::encode_byte,
    abi_string_type::encode_string, abi_ufixed_type::encode_ufixed, abi_uint_type::encode_uint,
    error::ABIError,
};

use super::abi_value::ABIValue;

// "Uint16" -> ABIType
// 123 -> ABIValue

// let uint16 = ABIUintType::new(16);
// uint16::encode(123);

// let tuple = ABITuple::new(ABIUintType::new(16), ABIUintType::new(16));

// let str = ABIString::new();
// str::encode("abc") | ABIString::encode("abc");

pub enum ABIType {
    // TODO: validation on creation
    ABIUintType(u16),
    ABIUFixedType(u16, u8),
    ABIAddressType,
    // ABITupleType(Vec<ABIType>), // blocked
    ABIStringType,
    ABIBoolType,
    ABIByteType,
}

pub fn encode(abi_type: ABIType, value: ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIUintType(_) => Ok(encode_uint(abi_type, value)?),
        ABIType::ABIUFixedType(_, __) => Ok(encode_ufixed(abi_type, value)?),
        ABIType::ABIAddressType => Ok(encode_address(abi_type, value)?),
        ABIType::ABIStringType => encode_string(abi_type, value),
        ABIType::ABIBoolType => encode_bool(abi_type, value),
        ABIType::ABIByteType => encode_byte(abi_type, value),
    }
}

pub fn is_dynamic(abi_type: &ABIType) -> bool {
    match abi_type {
        ABIType::ABIStaticArray(child_type, _) => is_dynamic(child_type),
        ABIType::ABIDynamicArray(child_type) => is_dynamic(child_type),
        ABIType::ABITupleType(child_types) => child_types.iter().all(|t| is_dynamic(t)),
        _ => false,
    }
}

pub fn get_name(abi_type: ABIType) -> String {
    "Not implemented".to_string()
}
