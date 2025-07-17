use num_bigint::BigUint;

use crate::common::ADDR_BYTE_SIZE;

#[derive(Debug, Clone, PartialEq)]
pub enum ABIValue {
    Bool(bool),
    Uint(BigUint),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<ABIValue>),
    Address([u8; ADDR_BYTE_SIZE]),
}
