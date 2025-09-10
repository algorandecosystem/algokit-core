use algokit_abi::ABIType as RustABIType;

use super::{ABIType, FfiToRustABIType};

#[derive(uniffi::Object, Clone)]
pub struct ABIBool {}

impl From<ABIBool> for RustABIType {
    fn from(_: ABIBool) -> Self {
        RustABIType::Bool
    }
}

impl From<RustABIType> for ABIBool {
    fn from(value: RustABIType) -> Self {
        if let RustABIType::Bool = value {
            ABIBool {}
        } else {
            panic!("Expected RustABIType::Bool");
        }
    }
}

impl FfiToRustABIType for ABIBool {
    fn to_rust_abi_type(&self) -> RustABIType {
        (*self).clone().into()
    }
}

impl ABIType for ABIBool {}
