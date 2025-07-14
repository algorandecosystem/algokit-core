use num_bigint::BigUint;

use crate::address::Address;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Uint(BigUint),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<Value>),
    Address(Address),
}
