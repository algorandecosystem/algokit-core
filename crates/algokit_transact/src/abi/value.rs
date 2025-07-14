use crate::address::Address;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Number(u64),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<Value>),
    Address(Address),
}
