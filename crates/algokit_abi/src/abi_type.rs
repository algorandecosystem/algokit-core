use crate::{
    common::{ADDR_BYTE_SIZE, BITS_PER_BYTE},
    error::ABIError,
    types::{
        collections::{
            array_dynamic::{decode_dynamic_array, encode_dynamic_array},
            array_static::{decode_static_array, encode_static_array},
            tuple::{decode_tuple, encode_tuple, find_bool_sequence_end},
        },
        primitives::{
            address::{decode_address, encode_address},
            bool::{decode_bool, encode_bool},
            byte::{decode_byte, encode_byte},
            string::{decode_string, encode_string},
            ufixed::{decode_ufixed, encode_ufixed},
            uint::{decode_uint, encode_uint},
        },
    },
};

use super::abi_value::ABIValue;

#[derive(Debug, Clone, Copy)]
pub struct BitSize(u16);

impl BitSize {
    pub fn new(bits: u16) -> Result<Self, ABIError> {
        if bits < BITS_PER_BYTE as u16 || bits > 512 || bits % BITS_PER_BYTE as u16 != 0 {
            return Err(ABIError::ValidationError(format!(
                "Bit size must be between {} and 512 and divisible by {}, got {}",
                BITS_PER_BYTE, BITS_PER_BYTE, bits
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
    Uint(BitSize),
    UFixed(BitSize, Precision),
    Address,
    Tuple(Vec<&'a ABIType<'a>>),
    String,
    Byte,
    Bool,
    StaticArray(&'a ABIType<'a>, usize),
    DynamicArray(&'a ABIType<'a>),
}

pub fn encode(abi_type: &ABIType, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
    match abi_type {
        ABIType::Uint(_) => Ok(encode_uint(abi_type, value)?),
        ABIType::UFixed(_, _) => Ok(encode_ufixed(abi_type, value)?),
        ABIType::Address => Ok(encode_address(abi_type, value)?),
        ABIType::Tuple(_) => Ok(encode_tuple(abi_type, value)?),
        ABIType::StaticArray(_, _size) => Ok(encode_static_array(abi_type, value)?),
        ABIType::DynamicArray(_) => Ok(encode_dynamic_array(abi_type, value)?),
        ABIType::String => Ok(encode_string(abi_type, value)?),
        ABIType::Byte => Ok(encode_byte(abi_type, value)?),
        ABIType::Bool => Ok(encode_bool(abi_type, value)?),
    }
}

pub fn decode(abi_type: &ABIType, bytes: &[u8]) -> Result<ABIValue, ABIError> {
    match abi_type {
        ABIType::Uint(_) => decode_uint(abi_type, bytes),
        ABIType::UFixed(_, _) => decode_ufixed(abi_type, bytes),
        ABIType::Address => decode_address(abi_type, bytes),
        ABIType::String => decode_string(abi_type, bytes),
        ABIType::Bool => decode_bool(abi_type, bytes),
        ABIType::Byte => decode_byte(abi_type, bytes),
        ABIType::Tuple(_) => decode_tuple(abi_type, bytes),
        ABIType::StaticArray(_, _size) => decode_static_array(abi_type, bytes),
        ABIType::DynamicArray(_) => decode_dynamic_array(abi_type, bytes),
    }
}

pub fn is_dynamic(abi_type: &ABIType) -> bool {
    match abi_type {
        ABIType::StaticArray(child_type, _) => is_dynamic(child_type),
        ABIType::Tuple(child_types) => child_types.iter().all(|t| is_dynamic(t)),
        ABIType::DynamicArray(_) => true,
        ABIType::String => true,
        _ => false,
    }
}

pub fn get_name(abi_type: &ABIType) -> String {
    match abi_type {
        ABIType::Uint(bit_size) => format!("uint{}", bit_size.value()),
        ABIType::UFixed(bit_size, precision) => {
            format!("ufixed{}x{}", bit_size.value(), precision.value())
        }
        ABIType::Address => "address".to_string(),
        ABIType::Tuple(child_types) => {
            let type_names: Vec<String> = child_types.iter().map(|t| get_name(t)).collect();
            format!("({})", type_names.join(","))
        }
        ABIType::String => "string".to_string(),
        ABIType::Byte => "byte".to_string(),
        ABIType::Bool => "bool".to_string(),
        ABIType::StaticArray(child_type, length) => {
            format!("{}[{}]", get_name(child_type), length)
        }
        ABIType::DynamicArray(child_type) => {
            format!("{}[]", get_name(child_type))
        }
    }
}

// TODO: check the return type
pub fn get_size(abi_type: &ABIType) -> Result<usize, ABIError> {
    match abi_type {
        ABIType::Uint(bit_size) => Ok((bit_size.value() / BITS_PER_BYTE as u16) as usize),
        ABIType::UFixed(bit_size, _) => Ok((bit_size.value() / BITS_PER_BYTE as u16) as usize),
        ABIType::Address => Ok(ADDR_BYTE_SIZE),
        ABIType::Bool => Ok(1),
        ABIType::Byte => Ok(1),
        ABIType::StaticArray(child_type, size) => match child_type {
            ABIType::Bool => Ok((*size).div_ceil(BITS_PER_BYTE as usize)),
            _ => Ok(get_size(child_type)? * *size),
        },
        ABIType::Tuple(child_types) => {
            let mut size = 0;
            let mut i = 0;
            while i < child_types.len() {
                let child_type = child_types[i];
                match child_type {
                    ABIType::Bool => {
                        let sequence_end_index = find_bool_sequence_end(child_types, i);
                        let bool_count = sequence_end_index - i + 1;

                        size += bool_count.div_ceil(BITS_PER_BYTE as usize);
                        i = sequence_end_index + 1;
                    }
                    _ => {
                        size += get_size(child_type)?;
                        i += 1;
                    }
                }
            }
            Ok(size)
        }
        ABIType::String => Err(ABIError::DecodingError(format!(
            "{} is a dynamic type",
            get_name(abi_type)
        ))),
        ABIType::DynamicArray(_) => Err(ABIError::DecodingError(format!(
            "{} is a dynamic type",
            get_name(abi_type)
        ))),
    }
}
