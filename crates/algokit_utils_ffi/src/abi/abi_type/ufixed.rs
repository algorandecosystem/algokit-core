use algokit_abi::abi_type::BitSize;
use algokit_abi::{ABIType as RustABIType, abi_type::Precision};

use crate::abi::abi_value::ABIValue;

use super::{ABIType, FfiToRustABIType};

#[derive(uniffi::Object, Clone)]
pub struct ABIUfixed {
    bit_size: u16,
    precision: u8,
}

impl From<ABIUfixed> for RustABIType {
    fn from(value: ABIUfixed) -> Self {
        RustABIType::UFixed(
            BitSize::new(value.bit_size).unwrap(),
            Precision::new(value.precision).unwrap(),
        )
    }
}

impl From<RustABIType> for ABIUfixed {
    fn from(value: RustABIType) -> Self {
        if let RustABIType::UFixed(bit_size, precision) = value {
            ABIUfixed {
                bit_size: bit_size.value(),
                precision: precision.value(),
            }
        } else {
            panic!("Expected RustABIType::Uint");
        }
    }
}

impl FfiToRustABIType for ABIUfixed {
    fn to_rust_abi_type(&self) -> RustABIType {
        self.clone().into()
    }
}

#[uniffi::export]
impl ABIType for ABIUfixed {
    fn decoode(&self, data: &[u8]) -> ABIValue {
        let rust_abi_type = self.to_rust_abi_type();
        ABIValue::from(rust_abi_type.decode(data).unwrap())
    }

    fn encode(&self, value: ABIValue) -> Vec<u8> {
        let rust_abi_type = self.to_rust_abi_type();
        rust_abi_type.encode(&value.into()).unwrap()
    }
}
