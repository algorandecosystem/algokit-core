use std::collections::BTreeMap;

use crate::constants::{
    ALGORAND_CHECKSUM_BYTE_LENGTH, ALGORAND_PUBLIC_KEY_BYTE_LENGTH, Byte32, HASH_BYTES_LENGTH,
};
use crate::traits::MsgPackEmpty;
use crate::{
    Address, AlgoKitTransactError, AlgorandMsgpack, MAX_TX_GROUP_SIZE, Transaction, TransactionId,
};
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as, skip_serializing_none};
use sha2::{Digest, Sha512_256};

const INTEGER_KEY_FIELDS: &[&str] = &["r"];

fn should_use_integer_keys(field_name: &str) -> bool {
    INTEGER_KEY_FIELDS.contains(&field_name)
}

pub fn sort_msgpack_value(value: rmpv::Value) -> rmpv::Value {
    sort_msgpack_value_internal(value, None)
}

fn sort_msgpack_value_internal(value: rmpv::Value, parent_field: Option<&str>) -> rmpv::Value {
    match value {
        rmpv::Value::Map(m) => {
            // Check if this map should have integer keys sorted
            let use_integer_keys = parent_field.map(should_use_integer_keys).unwrap_or(false);

            if use_integer_keys {
                // Sort map with integer keys numerically
                sort_map_with_integer_keys(m)
            } else {
                // Default: sort map with string keys alphabetically
                sort_map_with_string_keys(m)
            }
        }
        rmpv::Value::Array(arr) => rmpv::Value::Array(
            arr.into_iter()
                .map(|v| sort_msgpack_value_internal(v, None))
                .collect(),
        ),
        v => v,
    }
}
/// Sorts a map with string keys alphabetically.
fn sort_map_with_string_keys(map: Vec<(rmpv::Value, rmpv::Value)>) -> rmpv::Value {
    let mut sorted_map: BTreeMap<String, (rmpv::Value, Option<String>)> = BTreeMap::new();

    // Convert and sort all key-value pairs
    for (k, v) in map {
        if let rmpv::Value::String(key) = k {
            let key_str = key.into_str().unwrap_or_default();
            let field_name = key_str.clone();
            // Recursively sort the value, passing the field name as parent
            let sorted_v = sort_msgpack_value_internal(v, Some(&field_name));
            sorted_map.insert(key_str, (sorted_v, Some(field_name)));
        }
    }

    // Convert back to rmpv::Value::Map
    rmpv::Value::Map(
        sorted_map
            .into_iter()
            .map(|(k, (v, _))| (rmpv::Value::String(k.into()), v))
            .collect(),
    )
}

/// Sorts a map with integer keys numerically.
fn sort_map_with_integer_keys(map: Vec<(rmpv::Value, rmpv::Value)>) -> rmpv::Value {
    let mut int_entries: Vec<(u64, rmpv::Value)> = Vec::new();

    // Extract integer keys and sort values recursively
    for (k, v) in map {
        if let rmpv::Value::Integer(int_key) = k {
            // Convert to u64 for sorting
            let key_u64 = if let Some(u) = int_key.as_u64() {
                u
            } else if let Some(i) = int_key.as_i64() {
                i as u64 // Handle negative as well, though unlikely for this use case
            } else {
                0
            };

            // Recursively sort the value (no parent field since we're in an integer map)
            let sorted_v = sort_msgpack_value_internal(v, None);
            int_entries.push((key_u64, sorted_v));
        }
    }

    // Sort by integer key numerically
    int_entries.sort_by_key(|(k, _)| *k);

    // Convert back to rmpv::Value::Map with integer keys
    rmpv::Value::Map(
        int_entries
            .into_iter()
            .map(|(k, v)| (rmpv::Value::Integer(k.into()), v))
            .collect(),
    )
}

pub fn is_zero<T>(n: &T) -> bool
where
    T: PartialEq + From<u8>,
{
    *n == T::from(0u8)
}

pub fn is_zero_opt<T>(n: &Option<T>) -> bool
where
    T: PartialEq + From<u8>,
{
    n.as_ref().is_none_or(is_zero)
}

pub fn is_zero_addr(addr: &Address) -> bool {
    addr.as_bytes() == &[0u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
}

pub fn is_zero_addr_opt(addr: &Option<Address>) -> bool {
    addr.as_ref().is_none_or(is_zero_addr)
}

pub fn is_empty_bytes32(bytes: &Byte32) -> bool {
    bytes == &[0u8; 32]
}

pub fn is_empty_bytes32_opt(bytes: &Option<Byte32>) -> bool {
    bytes.as_ref().is_none_or(is_empty_bytes32)
}

pub fn is_empty_string_opt(string: &Option<String>) -> bool {
    string.as_ref().is_none_or(String::is_empty)
}

pub fn is_empty_vec_opt<T>(vec: &Option<Vec<T>>) -> bool {
    vec.as_ref().is_none_or(Vec::is_empty)
}

pub fn is_empty_struct_opt<T>(val: &Option<T>) -> bool
where
    T: MsgPackEmpty,
{
    val.as_ref().is_none_or(|v| v.is_empty())
}

pub fn pub_key_to_checksum(pub_key: &Byte32) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
    let mut hasher = Sha512_256::new();
    hasher.update(pub_key);

    let mut checksum = [0u8; ALGORAND_CHECKSUM_BYTE_LENGTH];
    checksum
        .copy_from_slice(&hasher.finalize()[(HASH_BYTES_LENGTH - ALGORAND_CHECKSUM_BYTE_LENGTH)..]);
    checksum
}

pub fn hash(bytes: &Vec<u8>) -> Byte32 {
    let mut hasher = Sha512_256::new();
    hasher.update(bytes);

    let mut hash_bytes = [0u8; HASH_BYTES_LENGTH];
    hash_bytes.copy_from_slice(&hasher.finalize()[..HASH_BYTES_LENGTH]);
    hash_bytes
}

pub fn compute_group(txs: &[Transaction]) -> Result<Byte32, AlgoKitTransactError> {
    if txs.is_empty() {
        return Err(AlgoKitTransactError::InputError {
            message: String::from("Transaction group size cannot be 0"),
        });
    }

    if txs.len() > MAX_TX_GROUP_SIZE {
        return Err(AlgoKitTransactError::InputError {
            message: format!(
                "Transaction group size exceeds the max limit of {}",
                MAX_TX_GROUP_SIZE
            ),
        });
    }

    let tx_hashes: Result<Vec<Byte32>, AlgoKitTransactError> = txs
        .iter()
        .map(|tx| {
            if tx.header().group.is_some() {
                return Err(AlgoKitTransactError::InputError {
                    message: "Transactions must not already be grouped".to_string(),
                });
            }
            tx.id_raw()
        })
        .collect();
    let grouped = (GroupedTransactions {
        tx_hashes: tx_hashes?,
    })
    .encode()
    .unwrap();

    Ok(hash(&grouped))
}

pub fn is_false_opt(bool: &Option<bool>) -> bool {
    bool.as_ref().is_none_or(|b| !b)
}

// This struct is only used internally for generating the group id
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct GroupedTransactions {
    #[serde(rename = "txlist")]
    #[serde_as(as = "Vec<Bytes>")]
    pub tx_hashes: Vec<Byte32>,
}

impl AlgorandMsgpack for GroupedTransactions {
    const PREFIX: &'static [u8] = b"TG";
}
