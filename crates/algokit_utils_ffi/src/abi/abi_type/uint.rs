use algokit_abi::ABIType as RustABIType;
use algokit_abi::abi_type::BitSize;

use super::{ABIType, FfiToRustABIType};

#[derive(uniffi::Object, Clone)]
pub struct ABIUint {
    bit_size: u16,
}

impl From<ABIUint> for RustABIType {
    fn from(value: ABIUint) -> Self {
        RustABIType::Uint(BitSize::new(value.bit_size).unwrap())
    }
}

impl From<RustABIType> for ABIUint {
    fn from(value: RustABIType) -> Self {
        if let RustABIType::Uint(bit_size) = value {
            ABIUint {
                bit_size: bit_size.value(),
            }
        } else {
            panic!("Expected RustABIType::Uint");
        }
    }
}

impl FfiToRustABIType for ABIUint {
    fn to_rust_abi_type(&self) -> RustABIType {
        (*self).clone().into()
    }
}

impl ABIType for ABIUint {}
