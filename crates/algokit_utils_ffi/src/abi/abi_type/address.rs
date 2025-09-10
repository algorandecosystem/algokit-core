use algokit_abi::ABIType as RustABIType;

use super::{ABIType, FfiToRustABIType};

#[derive(uniffi::Object, Clone)]
pub struct ABIAddress {}

impl From<ABIAddress> for RustABIType {
    fn from(_: ABIAddress) -> Self {
        RustABIType::Address
    }
}

impl From<RustABIType> for ABIAddress {
    fn from(value: RustABIType) -> Self {
        if let RustABIType::Address = value {
            ABIAddress {}
        } else {
            panic!("Expected RustABIType::Address");
        }
    }
}

impl FfiToRustABIType for ABIAddress {
    fn to_rust_abi_type(&self) -> RustABIType {
        (*self).clone().into()
    }
}

impl ABIType for ABIAddress {}
