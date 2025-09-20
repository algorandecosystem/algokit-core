use std::sync::Arc;

use derive_more::Display;

#[derive(uniffi::Object, Display)]
pub struct ABIType {
    pub abi_type: algokit_abi::ABIType,
}

#[uniffi::export]
impl ABIType {
    #[uniffi::constructor]
    pub fn bool() -> Self {
        Self {
            abi_type: algokit_abi::ABIType::Bool,
        }
    }

    #[uniffi::constructor]
    pub fn array(element_type: Arc<ABIType>) -> Self {
        Self {
            abi_type: algokit_abi::ABIType::DynamicArray(Box::new(element_type.abi_type.clone())),
        }
    }
}
