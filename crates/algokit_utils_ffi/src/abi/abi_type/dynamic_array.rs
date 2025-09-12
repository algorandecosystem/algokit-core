use std::sync::Arc;

use algokit_abi::ABIType as RustABIType;

use crate::abi::abi_value::ABIValue;

use super::{ABIType, FfiToRustABIType, RustToFfiABIType};

#[derive(uniffi::Object, Clone)]
pub struct ABIDynamicArray {
    element_type: Arc<dyn ABIType>,
}

impl From<ABIDynamicArray> for RustABIType {
    fn from(value: ABIDynamicArray) -> Self {
        RustABIType::DynamicArray(Box::new(value.element_type.to_rust_abi_type()))
    }
}

impl From<RustABIType> for ABIDynamicArray {
    fn from(value: RustABIType) -> Self {
        if let RustABIType::DynamicArray(rs_element_type) = value {
            ABIDynamicArray {
                element_type: rs_element_type.to_ffi_abi_type(),
            }
        } else {
            panic!("Expected RustABIType::DynamicArray");
        }
    }
}

impl FfiToRustABIType for ABIDynamicArray {
    fn to_rust_abi_type(&self) -> RustABIType {
        eprintln!("DEBUGPRINT[43]: dynamic_array.rs:34 (before let cloned = self.clone();)");
        let cloned = self.clone();
        eprintln!("DEBUGPRINT[44]: dynamic_array.rs:35 (after let cloned = self.clone();)");

        eprintln!("DEBUGPRINT[46]: dynamic_array.rs:38 (before cloned.into())");
        let res = cloned.into();
        eprintln!("DEBUGPRINT[47]: dynamic_array.rs:39 (after cloned.into())");
        res
    }
}

#[uniffi::export]
impl ABIType for ABIDynamicArray {
    fn decoode(&self, data: &[u8]) -> ABIValue {
        let rust_abi_type = self.to_rust_abi_type();
        ABIValue::from(rust_abi_type.decode(data).unwrap())
    }

    fn encode(&self, value: ABIValue) -> Vec<u8> {
        let rust_abi_type = self.to_rust_abi_type();
        rust_abi_type.encode(&value.into()).unwrap()
    }
}

#[uniffi::export]
impl ABIDynamicArray {
    #[uniffi::constructor]
    pub fn new(element_type: Arc<dyn ABIType>) -> Self {
        ABIDynamicArray { element_type }
    }
}
