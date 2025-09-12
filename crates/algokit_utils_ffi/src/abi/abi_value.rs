use algokit_abi::ABIValue as RustABIValue;
use num_bigint::BigUint;

// #[derive(uniffi::Enum)]
// pub enum ABIValue {
//     Bool(bool),
//     Uint(u64),
//     BigUint(Vec<u8>),
//     String(String),
//     Byte(u8),
//     Array(Vec<ABIValue>),
// }
//

#[derive(uniffi::Record)]
pub struct ABIValue {
    #[uniffi(default = None)]
    bool: Option<bool>,
    #[uniffi(default = None)]
    uint: Option<u64>,
    #[uniffi(default = None)]
    big_uint: Option<Vec<u8>>,
    #[uniffi(default = None)]
    string: Option<String>,
    #[uniffi(default = None)]
    byte: Option<u8>,
    #[uniffi(default = None)]
    array: Option<Vec<ABIValue>>,
    #[uniffi(default = None)]
    address: Option<String>,
}

impl From<ABIValue> for RustABIValue {
    fn from(value: ABIValue) -> Self {
        let is_some_count = [
            value.bool.is_some(),
            value.uint.is_some(),
            value.big_uint.is_some(),
            value.string.is_some(),
            value.byte.is_some(),
            value.array.is_some(),
            value.address.is_some(),
        ]
        .iter()
        .filter_map(|&b| if b { Some(()) } else { None })
        .count();

        if is_some_count == 0 {
            panic!("No fields set in ABIValue");
        }

        if is_some_count > 1 {
            panic!("Multiple fields set in ABIValue");
        }

        // check that only one field is set
        if let Some(b) = value.bool {
            return RustABIValue::Bool(b);
        }

        if let Some(u) = value.uint {
            return RustABIValue::Uint(u.into());
        }

        if let Some(bu) = value.big_uint {
            return RustABIValue::Uint(BigUint::from_bytes_be(&bu));
        }

        if let Some(s) = value.string {
            return RustABIValue::String(s);
        }

        if let Some(b) = value.byte {
            return RustABIValue::Byte(b);
        }

        if let Some(a) = value.array {
            return RustABIValue::Array(a.into_iter().map(|v| v.into()).collect());
        }

        if let Some(addr) = value.address {
            return RustABIValue::Address(addr.parse().unwrap());
        }

        unreachable!()
    }
}

impl From<RustABIValue> for ABIValue {
    fn from(value: RustABIValue) -> Self {
        match value {
            RustABIValue::Bool(b) => ABIValue {
                bool: Some(b),
                uint: None,
                big_uint: None,
                string: None,
                byte: None,
                array: None,
                address: None,
            },
            RustABIValue::Uint(u) => {
                let digits = u.to_u64_digits();
                if digits.len() == 1 {
                    ABIValue {
                        bool: None,
                        uint: Some(*digits.first().unwrap_or(&0u64)),
                        big_uint: None,
                        string: None,
                        byte: None,
                        array: None,
                        address: None,
                    }
                } else {
                    ABIValue {
                        bool: None,
                        uint: None,
                        big_uint: Some(u.to_bytes_be()),
                        string: None,
                        byte: None,
                        array: None,
                        address: None,
                    }
                }
            }
            RustABIValue::String(s) => ABIValue {
                bool: None,
                uint: None,
                big_uint: None,
                string: Some(s),
                byte: None,
                array: None,
                address: None,
            },
            RustABIValue::Byte(b) => ABIValue {
                bool: None,
                uint: None,
                big_uint: None,
                string: None,
                byte: Some(b),
                array: None,
                address: None,
            },
            RustABIValue::Array(a) => ABIValue {
                bool: None,
                uint: None,
                big_uint: None,
                string: None,
                byte: None,
                array: Some(a.into_iter().map(|v| v.into()).collect()),
                address: None,
            },
            RustABIValue::Address(addr) => ABIValue {
                bool: None,
                uint: None,
                big_uint: None,
                string: None,
                byte: None,
                array: None,
                address: Some(addr.to_string()),
            },
        }
    }
}
