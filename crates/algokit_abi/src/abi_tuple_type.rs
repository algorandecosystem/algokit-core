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
