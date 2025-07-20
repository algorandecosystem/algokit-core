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
use regex::Regex;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
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

#[derive(Clone)]
pub enum ABIType {
    Uint(BitSize),
    UFixed(BitSize, Precision),
    Address,
    Tuple(Vec<ABIType>),
    String,
    Byte,
    Bool,
    StaticArray(Box<ABIType>, usize),
    DynamicArray(Box<ABIType>),
}

impl AsRef<ABIType> for ABIType {
    fn as_ref(&self) -> &ABIType {
        self
    }
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

// TODO: check the return type
pub fn get_size(abi_type: &ABIType) -> Result<usize, ABIError> {
    match abi_type {
        ABIType::Uint(bit_size) => Ok((bit_size.value() / BITS_PER_BYTE as u16) as usize),
        ABIType::UFixed(bit_size, _) => Ok((bit_size.value() / BITS_PER_BYTE as u16) as usize),
        ABIType::Address => Ok(ADDR_BYTE_SIZE),
        ABIType::Bool => Ok(1),
        ABIType::Byte => Ok(1),
        ABIType::StaticArray(child_type, size) => match child_type.as_ref() {
            ABIType::Bool => Ok((*size).div_ceil(BITS_PER_BYTE as usize)),
            _ => Ok(get_size(child_type)? * *size),
        },
        ABIType::Tuple(child_types) => {
            let mut size = 0;
            let mut i = 0;
            while i < child_types.len() {
                let child_type = &child_types[i];
                match child_type.as_ref() {
                    ABIType::Bool => {
                        let sequence_end_index = find_bool_sequence_end(&child_types, i);
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
            abi_type
        ))),
        ABIType::DynamicArray(_) => Err(ABIError::DecodingError(format!(
            "{} is a dynamic type",
            abi_type
        ))),
    }
}

impl Display for ABIType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ABIType::Uint(bit_size) => write!(f, "uint{}", bit_size.value()),
            ABIType::UFixed(bit_size, precision) => {
                write!(f, "ufixed{}x{}", bit_size.value(), precision.value())
            }
            ABIType::Address => write!(f, "address"),
            ABIType::Tuple(child_types) => {
                let type_strings: Vec<String> = child_types.iter().map(|t| t.to_string()).collect();
                write!(f, "({})", type_strings.join(","))
            }
            ABIType::String => write!(f, "string"),
            ABIType::Byte => write!(f, "byte"),
            ABIType::Bool => write!(f, "bool"),
            ABIType::StaticArray(child_type, length) => {
                write!(f, "{}[{}]", child_type, length)
            }
            ABIType::DynamicArray(child_type) => {
                write!(f, "{}[]", child_type)
            }
        }
    }
}

impl FromStr for ABIType {
    type Err = ABIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Dynamic array
        if s.ends_with("[]") {
            let element_type_str = &s[..s.len() - 2];
            let element_type = ABIType::from_str(element_type_str)?;
            return Ok(ABIType::DynamicArray(Box::new(element_type)));
        }

        // Static array
        if s.ends_with(']') {
            let regex = Regex::new(r"^([a-z\d[\](),]+)\[(0|[1-9][\d]*)]$").expect("Invalid regex");
            if let Some(captures) = regex.captures(s) {
                let element_type_str = &captures[1];
                let length_str = &captures[2];

                let length = length_str.parse::<usize>().map_err(|_| {
                    ABIError::ValidationError(format!("Invalid array length: {}", length_str))
                })?;

                let element_type = ABIType::from_str(element_type_str)?;
                return Ok(ABIType::StaticArray(Box::new(element_type), length));
            } else {
                return Err(ABIError::ValidationError(format!(
                    "Malformed static array string: {}",
                    s
                )));
            }
        }

        // Uint type
        if s.starts_with("uint") {
            let size_str = &s[4..];
            if size_str.chars().all(|c| c.is_ascii_digit()) {
                let size = size_str.parse::<u16>().map_err(|_| {
                    ABIError::ValidationError(format!("Invalid uint size: {}", size_str))
                })?;
                let bit_size = BitSize::new(size)?;
                return Ok(ABIType::Uint(bit_size));
            } else {
                return Err(ABIError::ValidationError(format!(
                    "Malformed uint string: {}",
                    size_str
                )));
            }
        }

        // UFixed type
        if s.starts_with("ufixed") {
            let regex = Regex::new(r"^ufixed([1-9][\d]*)x([1-9][\d]*)$").expect("Invalid regex");
            if let Some(captures) = regex.captures(s) {
                let size_str = &captures[1];
                let precision_str = &captures[2];

                let size = size_str.parse::<u16>().map_err(|_| {
                    ABIError::ValidationError(format!("Invalid ufixed size: {}", size_str))
                })?;
                let precision = precision_str.parse::<u8>().map_err(|_| {
                    ABIError::ValidationError(format!(
                        "Invalid ufixed precision: {}",
                        precision_str
                    ))
                })?;

                let bit_size = BitSize::new(size)?;
                let precision = Precision::new(precision)?;
                return Ok(ABIType::UFixed(bit_size, precision));
            } else {
                return Err(ABIError::ValidationError(format!(
                    "Malformed ufixed type: {}",
                    s
                )));
            }
        }

        // Tuple type
        if s.len() >= 2 && s.starts_with('(') && s.ends_with(')') {
            return Err(ABIError::FormatError("Not supported".to_string()));
            // let tuple_content = &s[1..s.len() - 1];
            // let child_types = parse_tuple_content(tuple_content)?;
            // return Ok(ABIType::Tuple(child_types));
        }

        // Simple types
        match s {
            "byte" => Ok(ABIType::Byte),
            "bool" => Ok(ABIType::Bool),
            "address" => Ok(ABIType::Address),
            "string" => Ok(ABIType::String),
            _ => Err(ABIError::ValidationError(format!(
                "Cannot convert string '{}' to an ABI type",
                s
            ))),
        }
    }
}

// fn parse_tuple_content(content: String) -> Result<Vec<&ABIType>, ABIError> {
//     if content.is_empty() {
//         return Ok(Vec::new());
//     }

//     // TODO: can we regex this?
//     if content.starts_with(",") {
//         return Err(ABIError::FormatError(
//             "Tuple name should not start with comma".to_string(),
//         ));
//     }
//     if content.ends_with(",") {
//         return Err(ABIError::FormatError(
//             "Tuple name should not start with comma".to_string(),
//         ));
//     }
//     if content.contains(",,") {
//         return Err(ABIError::FormatError(
//             "tuple string should not have consecutive commas".to_string(),
//         ));
//     }

//     let mut result = Vec::new();
//     let mut current_start = 0;
//     let mut paren_depth = 0;
//     let mut bracket_depth = 0;

//     for (i, ch) in content.char_indices() {
//         match ch {
//             '(' => paren_depth += 1,
//             ')' => paren_depth -= 1,
//             '[' => bracket_depth += 1,
//             ']' => bracket_depth -= 1,
//             ',' if paren_depth == 0 && bracket_depth == 0 => {
//                 let type_str = content[current_start..i].trim();
//                 if !type_str.is_empty() {
//                     let abi_type = Box::new(from_string(type_str)?);
//                     result.push(Box::leak(abi_type));
//                 }
//                 current_start = i + 1;
//             }
//             _ => {}
//         }
//     }

//     // Handle the last element
//     let type_str = content[current_start..].trim();
//     if !type_str.is_empty() {
//         let abi_type = Box::new(from_string(type_str)?);
//         result.push(Box::leak(abi_type));
//     }

//     Ok(result)
// }
