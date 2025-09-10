use algokit_abi::ABIValue as RustABIValue;

#[derive(uniffi::Enum)]
pub enum ABIValue {
    Bool(bool),
    Uint(u64),
    BigUint(Vec<u8>),
    String(String),
    Byte(u8),
    Array(Vec<ABIValue>),
}

impl From<RustABIValue> for ABIValue {
    fn from(value: RustABIValue) -> Self {
        match value {
            RustABIValue::Bool(b) => ABIValue::Bool(b),
            RustABIValue::Uint(u) => {
                let digits = u.to_u64_digits();
                if digits.len() == 1 {
                    ABIValue::Uint(digits[0])
                } else {
                    ABIValue::BigUint(u.to_bytes_be())
                }
            }
            RustABIValue::String(s) => ABIValue::String(s),
            RustABIValue::Byte(b) => ABIValue::Byte(b),
            RustABIValue::Array(arr) => {
                let vec = arr.into_iter().map(ABIValue::from).collect();
                ABIValue::Array(vec)
            }
            RustABIValue::Address(addr) => ABIValue::String(addr), // Addresses are represented as strings
        }
    }
}

impl From<ABIValue> for RustABIValue {
    fn from(value: ABIValue) -> Self {
        match value {
            ABIValue::Bool(b) => RustABIValue::Bool(b),
            ABIValue::Uint(u) => RustABIValue::Uint(u.into()),
            ABIValue::BigUint(bytes) => {
                let big_uint = num_bigint::BigUint::from_bytes_be(&bytes);
                RustABIValue::Uint(big_uint)
            }
            ABIValue::String(s) => RustABIValue::String(s),
            ABIValue::Byte(b) => RustABIValue::Byte(b),
            ABIValue::Array(arr) => {
                let vec = arr.into_iter().map(RustABIValue::from).collect();
                RustABIValue::Array(vec)
            }
        }
    }
}
