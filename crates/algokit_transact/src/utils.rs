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
use std::collections::BTreeMap;

pub fn sort_msgpack_value(value: rmpv::Value) -> Result<rmpv::Value, AlgoKitTransactError> {
    match value {
        rmpv::Value::Map(m) => {
            if m.is_empty() {
                return Ok(rmpv::Value::Map(Vec::new()));
            }

            // First, determine the key type by checking all keys
            let mut has_string_keys = false;
            let mut has_integer_keys = false;
            let mut has_other_keys = false;

            for (k, _) in &m {
                match k {
                    rmpv::Value::String(_) => has_string_keys = true,
                    rmpv::Value::Integer(_) => has_integer_keys = true,
                    _ => has_other_keys = true,
                }
            }

            // Check for mixed key types or unsupported key types
            let key_type_count = [has_string_keys, has_integer_keys, has_other_keys]
                .iter()
                .filter(|&&x| x)
                .count();

            if key_type_count > 1 || has_other_keys {
                return Err(AlgoKitTransactError::InputError {
                    message: "MessagePack map must have all keys of the same type (either all strings or all integers)".to_string(),
                });
            }

            if has_string_keys {
                // Use BTreeMap for string keys (sorted)
                let mut sorted_map: BTreeMap<String, rmpv::Value> = BTreeMap::new();
                for (k, v) in m {
                    if let rmpv::Value::String(key) = k {
                        sorted_map.insert(
                            key.as_str().unwrap_or_default().to_string(),
                            sort_msgpack_value(v)?,
                        );
                    }
                }

                let result: Vec<(rmpv::Value, rmpv::Value)> = sorted_map
                    .into_iter()
                    .map(|(k, v)| (rmpv::Value::String(k.into()), v))
                    .collect();

                Ok(rmpv::Value::Map(result))
            } else if has_integer_keys {
                // Use Vec for integer keys (preserve order)
                let result: Result<Vec<(rmpv::Value, rmpv::Value)>, AlgoKitTransactError> = m
                    .into_iter()
                    .map(|(k, v)| Ok((k, sort_msgpack_value(v)?)))
                    .collect();

                Ok(rmpv::Value::Map(result?))
            } else {
                // Empty map was already handled above, this shouldn't be reached
                Ok(rmpv::Value::Map(Vec::new()))
            }
        }
        rmpv::Value::Array(arr) => {
            let result: Result<Vec<rmpv::Value>, AlgoKitTransactError> =
                arr.into_iter().map(sort_msgpack_value).collect();
            Ok(rmpv::Value::Array(result?))
        }
        // For all other types, return as-is
        v => Ok(v),
    }
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
