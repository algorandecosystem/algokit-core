use std::sync::Arc;

use algokit_abi::ABIType as RustABIType;

use crate::abi::abi_value::ABIValue;

use super::RustToFfiABIType;
use super::{ABIType, FfiToRustABIType};

#[derive(uniffi::Object, Clone)]
pub struct ABITuple {
    components: Vec<Arc<dyn ABIType>>,
}

impl FfiToRustABIType for ABITuple {
    fn to_rust_abi_type(&self) -> RustABIType {
        self.clone().into()
    }
}

impl From<ABITuple> for RustABIType {
    fn from(value: ABITuple) -> Self {
        let rust_components = value
            .components
            .into_iter()
            .map(|comp| comp.to_rust_abi_type())
            .collect();
        RustABIType::Tuple(rust_components)
    }
}

impl From<RustABIType> for ABITuple {
    fn from(value: RustABIType) -> Self {
        if let RustABIType::Tuple(rust_components) = value {
            let components = rust_components
                .into_iter()
                .map(|comp| comp.to_ffi_abi_type())
                .collect();
            ABITuple { components }
        } else {
            panic!("Expected RustABIType::Tuple");
        }
    }
}

#[uniffi::export]
impl ABIType for ABITuple {
    fn decoode(&self, data: &[u8]) -> ABIValue {
        let rust_abi_type = self.to_rust_abi_type();
        ABIValue::from(rust_abi_type.decode(data).unwrap())
    }

    fn encode(&self, value: ABIValue) -> Vec<u8> {
        let rust_abi_type = self.to_rust_abi_type();
        rust_abi_type.encode(&value.into()).unwrap()
    }
}
