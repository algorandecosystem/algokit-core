use super::{ABIType, FfiToRustABIType};
use algokit_abi::ABIType as RustABIType;

#[derive(uniffi::Object, Clone)]
pub struct ABIByte {}

impl From<ABIByte> for RustABIType {
    fn from(_: ABIByte) -> Self {
        RustABIType::Byte
    }
}

impl From<RustABIType> for ABIByte {
    fn from(value: RustABIType) -> Self {
        if let RustABIType::Byte = value {
            ABIByte {}
        } else {
            panic!("Expected RustABIType::Byte");
        }
    }
}

impl FfiToRustABIType for ABIByte {
    fn to_rust_abi_type(&self) -> RustABIType {
        (*self).clone().into()
    }
}

impl ABIType for ABIByte {}
