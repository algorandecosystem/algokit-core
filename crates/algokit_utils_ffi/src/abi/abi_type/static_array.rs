use std::sync::Arc;

use algokit_abi::ABIType as RustABIType;

use super::{ABIType, FfiToRustABIType, RustToFfiABIType};

#[derive(uniffi::Object, Clone)]
pub struct ABIStaticArray {
    element_type: Arc<dyn ABIType>,
    length: u32,
}

impl From<ABIStaticArray> for RustABIType {
    fn from(value: ABIStaticArray) -> Self {
        RustABIType::StaticArray(
            Box::new(value.element_type.to_rust_abi_type()),
            value.length.try_into().unwrap(),
        )
    }
}

impl From<RustABIType> for ABIStaticArray {
    fn from(value: RustABIType) -> Self {
        if let RustABIType::StaticArray(rs_element_type, rs_length) = value {
            ABIStaticArray {
                element_type: rs_element_type.to_ffi_abi_type(),
                length: rs_length.try_into().unwrap(),
            }
        } else {
            panic!("Expected RustABIType::StaticArray");
        }
    }
}

impl FfiToRustABIType for ABIStaticArray {
    fn to_rust_abi_type(&self) -> RustABIType {
        (*self).clone().into()
    }
}

impl ABIType for ABIStaticArray {}
