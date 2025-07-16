use num_bigint::BigUint;

#[derive(Debug, Clone, PartialEq)]
pub enum ABIValue {
    Bool(bool),
    Uint(BigUint),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<ABIValue>),
    Address([u8; 32]),
}
