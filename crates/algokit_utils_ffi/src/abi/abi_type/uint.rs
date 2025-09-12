use algokit_abi::ABIType as RustABIType;
use algokit_abi::abi_type::BitSize;

use crate::abi::abi_value::ABIValue;

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
        self.clone().into()
    }
}

#[uniffi::export]
impl ABIType for ABIUint {
    fn decoode(&self, data: &[u8]) -> ABIValue {
        let rust_abi_type = self.to_rust_abi_type();
        ABIValue::from(rust_abi_type.decode(data).unwrap())
    }

    fn encode(&self, value: ABIValue) -> Vec<u8> {
        let rust_abi_type = self.to_rust_abi_type();
        rust_abi_type.encode(&value.into()).unwrap()
    }
}
