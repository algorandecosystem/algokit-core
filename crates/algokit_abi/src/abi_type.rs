use crate::{
    abi_address_type::{decode_address, encode_address},
    abi_bool_type::{decode_bool, encode_bool},
    abi_byte_type::{decode_byte, encode_byte},
    abi_string_type::{decode_string, encode_string},
    abi_tuple_type::{decode_tuple, encode_tuple},
    abi_ufixed_type::{decode_ufixed, encode_ufixed},
    abi_uint_type::{decode_uint, encode_uint},
    error::ABIError,
};

use super::abi_value::ABIValue;

#[derive(Clone, Debug)]
pub enum ABIType {
    // TODO: validation on creation
    ABIUintType(u16),
    ABIUFixedType(u16, u8),
    ABIAddressType,
    ABITupleType(Vec<ABIType>),
    ABIStringType,
    ABIBoolType,
    ABIByteType,
}

pub fn encode(abi_type: ABIType, value: ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIUintType(_) => Ok(encode_uint(abi_type, value)?),
        ABIType::ABIUFixedType(_, _) => Ok(encode_ufixed(abi_type, value)?),
        ABIType::ABIAddressType => Ok(encode_address(abi_type, value)?),
        ABIType::ABITupleType(_) => encode_tuple(abi_type, value),
        ABIType::ABIStringType => encode_string(abi_type, value),
        ABIType::ABIBoolType => encode_bool(abi_type, value),
        ABIType::ABIByteType => encode_byte(abi_type, value),
    }
}

pub fn decode(abi_type: ABIType, bytes: Vec<u8>) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::ABIUintType(_) => decode_uint(abi_type, bytes),
        ABIType::ABIUFixedType(_, _) => decode_ufixed(abi_type, bytes),
        ABIType::ABIAddressType => decode_address(abi_type, bytes),
        ABIType::ABITupleType(_) => decode_tuple(abi_type, bytes),
        ABIType::ABIStringType => decode_string(abi_type, bytes),
        ABIType::ABIBoolType => decode_bool(abi_type, bytes),
        ABIType::ABIByteType => decode_byte(abi_type, bytes),
    }
}

pub fn is_dynamic(abi_type: &ABIType) -> bool {
    match abi_type {
        ABIType::ABIStringType => true,
        ABIType::ABITupleType(child_types) => child_types.iter().any(is_dynamic),
        ABIType::ABIUintType(_) => false,
        ABIType::ABIUFixedType(_, _) => false,
        ABIType::ABIAddressType => false,
        ABIType::ABIBoolType => false,
        ABIType::ABIByteType => false,
    }
}

pub fn get_name(_abi_type: ABIType) -> String {
    "Not implemented".to_string()
}
