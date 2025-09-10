use std::sync::Arc;

use super::abi_value::ABIValue;
use algokit_abi::ABIType as RustABIType;

mod address;
mod bool;
mod byte;
mod dynamic_array;
mod static_array;
mod string;
mod tuple;
mod ufixed;
mod uint;

// TODO: remove unwraps in this module

/// This trait is used to convert FFI ABI types to Rust ABI types
/// This is needed because we can't implement From on Arc<dyn ABIType>
pub trait FfiToRustABIType {
    /// Convert the FFI ABI type to a Rust ABI type
    fn to_rust_abi_type(&self) -> RustABIType;
}

/// This trait is used to convert Rust ABI types to FFI ABI types
/// This is needed because we can't implement From on Arc<dyn ABIType>
trait RustToFfiABIType {
    fn to_ffi_abi_type(&self) -> Arc<dyn ABIType>;
}

impl RustToFfiABIType for RustABIType {
    fn to_ffi_abi_type(&self) -> Arc<dyn ABIType> {
        let abi_type: Arc<dyn ABIType> = match self {
            RustABIType::Uint(_) => Arc::new(uint::ABIUint::from(self.clone())),
            RustABIType::Tuple(_) => Arc::new(tuple::ABITuple::from(self.clone())),
            RustABIType::UFixed(_, _) => Arc::new(ufixed::ABIUfixed::from(self.clone())),
            RustABIType::String => Arc::new(string::ABIString::from(self.clone())),
            RustABIType::Byte => Arc::new(byte::ABIByte::from(self.clone())),
            RustABIType::Bool => Arc::new(bool::ABIBool::from(self.clone())),
            RustABIType::DynamicArray(_) => {
                Arc::new(dynamic_array::ABIDynamicArray::from(self.clone()))
            }
            RustABIType::StaticArray(_, _) => {
                Arc::new(static_array::ABIStaticArray::from(self.clone()))
            }
            RustABIType::Address => Arc::new(address::ABIAddress::from(self.clone())),
        };
        abi_type
    }
}

#[uniffi::export]
pub trait ABIType: Send + Sync + FfiToRustABIType {
    fn decoode(&self, data: &[u8]) -> ABIValue {
        let rust_abi_type = self.to_rust_abi_type();
        ABIValue::from(rust_abi_type.decode(data).unwrap())
    }

    fn encode(&self, value: ABIValue) -> Vec<u8> {
        let rust_abi_type = self.to_rust_abi_type();
        rust_abi_type.encode(&value.into()).unwrap()
    }
}
