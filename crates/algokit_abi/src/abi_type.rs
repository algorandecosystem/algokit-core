use crate::{
    abi_address_type::encode_address, abi_tuple_type::encode_tuple, abi_ufixed_type::encode_ufixed,
    abi_uint_type::encode_uint, dynamic_array::encode_dynamic_array, error::ABIError,
    static_array::encode_static_array,
};

use super::abi_value::ABIValue;

pub enum ABIType<'a> {
    // TODO: validation on creation
    ABIUintType(u16),
    ABIUFixedType(u16, u8),
    ABIAddressType,
    ABITupleType(Vec<&'a ABIType<'a>>),
    ABIString,
    ABIByte,
    ABIBool,
    ABIStaticArray(&'a ABIType<'a>, usize),
    ABIDynamicArray(&'a ABIType<'a>), // blocked
}

pub fn encode(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIUintType(_) => Ok(encode_uint(abi_type, value)?),
        ABIType::ABIUFixedType(_, __) => Ok(encode_ufixed(abi_type, value)?),
        ABIType::ABIAddressType => Ok(encode_address(abi_type, value)?),
        ABIType::ABITupleType(_) => Ok(encode_tuple(abi_type, value)?),
        ABIType::ABIStaticArray(_, __) => Ok(encode_static_array(abi_type, value)?),
        ABIType::ABIDynamicArray(_) => Ok(encode_dynamic_array(abi_type, value)?),
        _ => return Err(ABIError::EncodingError("Not implemented".to_string())),
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

pub fn get_name(abi_type: &ABIType) -> String {
    "Not implemented".to_string()
}
