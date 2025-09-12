use algokit_abi::ABIType as RustABIType;

use crate::abi::abi_value::ABIValue;

use super::{ABIType, FfiToRustABIType};

#[derive(uniffi::Object, Clone)]
pub struct ABIString {}

impl From<ABIString> for RustABIType {
    fn from(_: ABIString) -> Self {
        RustABIType::String
    }
}

impl From<RustABIType> for ABIString {
    fn from(value: RustABIType) -> Self {
        if let RustABIType::String = value {
            ABIString {}
        } else {
            panic!("Expected RustABIType::String");
        }
    }
}

impl FfiToRustABIType for ABIString {
    fn to_rust_abi_type(&self) -> RustABIType {
        self.clone().into()
    }
}

#[uniffi::export]
impl ABIType for ABIString {
    fn decoode(&self, data: &[u8]) -> ABIValue {
        let rust_abi_type = self.to_rust_abi_type();
        ABIValue::from(rust_abi_type.decode(data).unwrap())
    }

    fn encode(&self, value: ABIValue) -> Vec<u8> {
        let rust_abi_type = self.to_rust_abi_type();
        rust_abi_type.encode(&value.into()).unwrap()
    }
}
