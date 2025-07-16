use std::collections::HashMap;

use crate::{
    abi_type::{encode, is_dynamic},
    error::ABIError,
    utils::extend_bytes_to_length,
    ABIType, ABIValue,
};

pub fn encode_tuple(abi_type: ABIType, value: ABIValue) -> Result<Vec<u8>, ABIError> {
    let child_types = match abi_type {
        ABIType::ABITupleType(child_types) => child_types,
        _ => return Err(ABIError::EncodingError("Expected ABITupleType".to_string())),
    };
    let value = match value {
        ABIValue::Array(n) => n,
        _ => {
            return Err(ABIError::EncodingError(format!(
                "Cannot encode value",
                // TODO: error including tuple type name (to_string)
            )));
        }
    };

    let mut heads: Vec<Vec<u8>> = Vec::new();
    let mut tails: Vec<Vec<u8>> = Vec::new();
    let mut is_dynamic_index: HashMap<usize, bool> = HashMap::new();

    for i in 0..child_types.len() {
        let child_type = child_types[i];

        if is_dynamic(&child_type) {
            is_dynamic_index.insert(i, true);
            heads.push(vec![0, 0]);
            tails.push(encode(child_type, value[i])?);
        } else {
            match child_type {
                ABIType::ABIBool => {}
                _ => {
                    heads.push(encode(child_type, value[i])?);
                }
            }
            is_dynamic_index.insert(i, false);
            tails.push(vec![]);
        }
    }

    let head_length: usize = heads.iter().map(|e| e.len()).sum();
    let tail_length = 0;

    for i in 0..child_types.len() {
        match is_dynamic_index[&i] {
            true => {
                let head_value = head_length + tail_length;
                // TODO: throw error if head_value > u16 MAX
                heads[i] = extend_bytes_to_length(head_value.to_be_bytes().to_vec(), 2);
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
