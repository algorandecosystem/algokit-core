use crate::{
    abi_address_type::{decode_address, encode_address},
    abi_bool_type::{decode_bool, encode_bool},
    abi_byte_type::{decode_byte, encode_byte},
    abi_string_type::{decode_string, encode_string},
    abi_tuple_type::{decode_tuple, encode_tuple, find_bool_sequence_end},
    abi_ufixed_type::{decode_ufixed, encode_ufixed},
    abi_uint_type::{decode_uint, encode_uint},
    common::ADDR_BYTE_SIZE,
    dynamic_array::decode_dynamic_array,
    error::ABIError,
    static_array::{decode_static_array, encode_static_array},
};

use super::abi_value::ABIValue;

#[derive(Debug, Clone, Copy)]
pub struct BitSize(u16);

impl BitSize {
    pub fn new(bits: u16) -> Result<Self, ABIError> {
        if bits < 8 || bits > 512 || bits % 8 != 0 {
            return Err(ABIError::ValidationError(format!(
                "Bit size must be between 8 and 512 and divisible by 8, got {}",
                bits
            )));
        }
        Ok(BitSize(bits))
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Precision(u8);

impl Precision {
    pub fn new(precision: u8) -> Result<Self, ABIError> {
        if precision > 160 {
            return Err(ABIError::ValidationError(format!(
                "Precision must be between 0 and 160, got {}",
                precision
            )));
        }
        Ok(Precision(precision))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

pub enum ABIType<'a> {
    ABIUintType(BitSize),
    ABIUFixedType(BitSize, Precision),
    ABIAddressType,
    ABITupleType(Vec<&'a ABIType<'a>>),
    ABIString,
    ABIByte,
    ABIBool,
    ABIStaticArray(&'a ABIType<'a>, usize),
    ABIDynamicArray(&'a ABIType<'a>),
}

pub fn encode(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::ABIUintType(_) => Ok(encode_uint(abi_type, value)?),
        ABIType::ABIUFixedType(_, _) => Ok(encode_ufixed(abi_type, value)?),
        ABIType::ABIAddressType => Ok(encode_address(abi_type, value)?),
        ABIType::ABITupleType(_) => Ok(encode_tuple(abi_type, value)?),
        ABIType::ABIStaticArray(_, __) => Ok(encode_static_array(abi_type, value)?),
        ABIType::ABIDynamicArray(_) => Ok(encode_static_array(abi_type, value)?),
        ABIType::ABIString => Ok(encode_string(abi_type, value)?),
        ABIType::ABIByte => Ok(encode_bool(abi_type, value)?),
        ABIType::ABIBool => Ok(encode_byte(abi_type, value)?),
    }
}

pub fn decode(abi_type: &ABIType, bytes: &[u8]) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::ABIUintType(_) => decode_uint(abi_type, bytes),
        ABIType::ABIUFixedType(_, _) => decode_ufixed(abi_type, bytes),
        ABIType::ABIAddressType => decode_address(abi_type, bytes),
        ABIType::ABIString => decode_string(abi_type, bytes),
        ABIType::ABIBool => decode_bool(abi_type, bytes),
        ABIType::ABIByte => decode_byte(abi_type, bytes),
        ABIType::ABITupleType(_) => decode_tuple(abi_type, bytes),
        ABIType::ABIStaticArray(_, __) => decode_static_array(abi_type, bytes),
        ABIType::ABIDynamicArray(_) => decode_dynamic_array(abi_type, bytes),
    }
}

pub fn is_dynamic(abi_type: &ABIType) -> bool {
    match abi_type {
        ABIType::ABIStaticArray(child_type, _) => is_dynamic(child_type),
        ABIType::ABITupleType(child_types) => child_types.iter().all(|t| is_dynamic(t)),
        ABIType::ABIDynamicArray(_) => true,
        ABIType::ABIString => true,
        _ => false,
    }
}

pub fn get_name(abi_type: &ABIType) -> String {
    match abi_type {
        ABIType::ABIUintType(bit_size) => format!("uint{}", bit_size.value()),
        ABIType::ABIUFixedType(bit_size, precision) => {
            format!("ufixed{}x{}", bit_size.value(), precision.value())
        }
        ABIType::ABIAddressType => "address".to_string(),
        ABIType::ABITupleType(child_types) => {
            let type_names: Vec<String> = child_types.iter().map(|t| get_name(t)).collect();
            format!("({})", type_names.join(","))
        }
        ABIType::ABIString => "string".to_string(),
        ABIType::ABIByte => "byte".to_string(),
        ABIType::ABIBool => "bool".to_string(),
        ABIType::ABIStaticArray(child_type, length) => {
            format!("{}[{}]", get_name(child_type), length)
        }
        ABIType::ABIDynamicArray(child_type) => {
            format!("{}[]", get_name(child_type))
        }
    }
}

// TODO: check the return type
pub fn get_size(abi_type: &ABIType) -> Result<usize, ABIError> {
    match abi_type {
        ABIType::ABIUintType(bit_size) => Ok((bit_size.value() / 8) as usize),
        ABIType::ABIUFixedType(bit_size, _) => Ok((bit_size.value() / 8) as usize),
        ABIType::ABIAddressType => Ok(ADDR_BYTE_SIZE),
        ABIType::ABIBool => Ok(1),
        ABIType::ABIByte => Ok(1),
        ABIType::ABIStaticArray(child_type, size) => match child_type {
            ABIType::ABIBool => Ok((*size).div_ceil(8)),
            _ => Ok(get_size(&child_type)? * *size),
        },
        ABIType::ABITupleType(child_types) => {
            let mut size = 0;
            for mut i in 0..child_types.len() {
                let child_type = child_types[i];
                match child_type {
                    ABIType::ABIBool => {
                        let sequence_end_index = find_bool_sequence_end(child_types, i);
                        let bool_count = sequence_end_index - i + 1;

                        size += bool_count.div_ceil(8);
                        i = sequence_end_index;
                    }
                    _ => {
                        size += get_size(child_type)?;
                    }
                }
            }
            Ok(size)
        }
        ABIType::ABIString => Err(ABIError::DecodingError(format!(
            "{} is a dynamic type",
            get_name(abi_type)
        ))),
        ABIType::ABIDynamicArray(_) => Err(ABIError::DecodingError(format!(
            "{} is a dynamic type",
            get_name(abi_type)
        ))),
    }
}
