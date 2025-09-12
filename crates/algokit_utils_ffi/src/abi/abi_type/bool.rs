use algokit_abi::ABIType as RustABIType;

use crate::abi::abi_value::ABIValue;

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
        self.clone().into()
    }
}

#[uniffi::export]
impl ABIBool {
    #[allow(clippy::new_without_default)]
    #[uniffi::constructor]
    pub fn new() -> Self {
        ABIBool {}
    }
}

#[uniffi::export]
impl ABIType for ABIBool {
    fn decoode(&self, data: &[u8]) -> ABIValue {
        let rust_abi_type = self.to_rust_abi_type();
        ABIValue::from(rust_abi_type.decode(data).unwrap())
    }

    fn encode(&self, value: ABIValue) -> Vec<u8> {
        let rust_abi_type = self.to_rust_abi_type();
        rust_abi_type.encode(&value.into()).unwrap()
    }
}
