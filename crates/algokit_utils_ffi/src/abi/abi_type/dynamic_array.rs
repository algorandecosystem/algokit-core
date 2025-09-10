use std::sync::Arc;

use algokit_abi::ABIType as RustABIType;

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
        (*self).clone().into()
    }
}

impl ABIType for ABIDynamicArray {}
