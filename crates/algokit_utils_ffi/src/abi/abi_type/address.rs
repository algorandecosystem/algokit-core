use algokit_abi::ABIType as RustABIType;

use crate::abi::abi_value::ABIValue;

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
        self.clone().into()
    }
}

#[uniffi::export]
impl ABIType for ABIAddress {
    fn decoode(&self, data: &[u8]) -> ABIValue {
        let rust_abi_type = self.to_rust_abi_type();
        ABIValue::from(rust_abi_type.decode(data).unwrap())
    }

    fn encode(&self, value: ABIValue) -> Vec<u8> {
        let rust_abi_type = self.to_rust_abi_type();
        rust_abi_type.encode(&value.into()).unwrap()
    }
}
