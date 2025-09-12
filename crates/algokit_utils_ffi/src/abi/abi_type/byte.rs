use crate::abi::abi_value::ABIValue;

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
        self.clone().into()
    }
}

#[uniffi::export]
impl ABIType for ABIByte {
    fn decoode(&self, data: &[u8]) -> ABIValue {
        let rust_abi_type = self.to_rust_abi_type();
        ABIValue::from(rust_abi_type.decode(data).unwrap())
    }

    fn encode(&self, value: ABIValue) -> Vec<u8> {
        let rust_abi_type = self.to_rust_abi_type();
        rust_abi_type.encode(&value.into()).unwrap()
    }
}
