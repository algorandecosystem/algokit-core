use std::fmt::Display;

use super::value::Value;
use crate::AlgoKitTransactError;

// TODO: do we need PartialEq? Is to_string equal enough?
pub trait Type: Display + PartialEq {
    fn is_dynamic(&self) -> Result<bool, AlgoKitTransactError>;
    fn byte_len(&self) -> Result<u16, AlgoKitTransactError>;
    fn encode(&self, value: Value) -> Result<Vec<u8>, AlgoKitTransactError>;
    fn decode(&self, bytes: Vec<u8>) -> Result<Value, AlgoKitTransactError>;
}
