use algokit_abi::ABIType as RustABIType;

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
        (*self).clone().into()
    }
}

impl ABIType for ABIString {}
