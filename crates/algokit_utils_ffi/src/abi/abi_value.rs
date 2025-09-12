use std::{str::FromStr, sync::Arc};

use algokit_abi::ABIType as RustABIType;

#[derive(uniffi::Object, Debug, Clone, PartialEq)]
pub struct ABIValue {
    bytes: Vec<u8>,
    abi_type: String,
}

#[uniffi::export]
impl ABIValue {
    pub fn encoded_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn abi_type(&self) -> String {
        self.abi_type.clone()
    }

    // TODO: support > u64
    #[uniffi::constructor]
    pub fn uint(value: u64, width: u16) -> Self {
        let abi_type = format!("uint{}", width);
        let rust_abi_type = RustABIType::from_str(&abi_type).unwrap();
        let bytes = rust_abi_type
            .encode(&algokit_abi::ABIValue::Uint(value.into()))
            .unwrap();
        ABIValue { bytes, abi_type }
    }

    #[uniffi::constructor]
    pub fn bool(value: bool) -> Self {
        let abi_type = "bool".to_string();
        let rust_abi_type = RustABIType::from_str(&abi_type).unwrap();
        let bytes = rust_abi_type
            .encode(&algokit_abi::ABIValue::Bool(value))
            .unwrap();
        ABIValue { bytes, abi_type }
    }

    #[uniffi::constructor]
    pub fn string(value: String) -> Self {
        let abi_type = "string".to_string();
        let rust_abi_type = RustABIType::from_str(&abi_type).unwrap();
        let bytes = rust_abi_type
            .encode(&algokit_abi::ABIValue::String(value))
            .unwrap();
        ABIValue { bytes, abi_type }
    }

    #[uniffi::constructor]
    pub fn byte(value: u8) -> Self {
        let abi_type = "byte".to_string();
        let rust_abi_type = RustABIType::from_str(&abi_type).unwrap();
        let bytes = rust_abi_type
            .encode(&algokit_abi::ABIValue::Byte(value))
            .unwrap();
        ABIValue { bytes, abi_type }
    }

    #[uniffi::constructor]
    pub fn address(value: String) -> Self {
        let abi_type = "address".to_string();
        let rust_abi_type = RustABIType::from_str(&abi_type).unwrap();
        let bytes = rust_abi_type
            .encode(&algokit_abi::ABIValue::Address(value))
            .unwrap();
        ABIValue { bytes, abi_type }
    }

    #[uniffi::constructor]
    pub fn array(element_type: String, values: Vec<Arc<ABIValue>>) -> Self {
        let abi_type = format!("{}[]", element_type);
        let rust_abi_type = RustABIType::from_str(&abi_type).unwrap();
        let rust_values: Vec<algokit_abi::ABIValue> = values
            .into_iter()
            .map(|v| {
                let rust_abi_type = RustABIType::from_str(&element_type).unwrap();
                rust_abi_type.decode(&v.encoded_bytes()).unwrap()
            })
            .collect();
        let bytes = rust_abi_type
            .encode(&algokit_abi::ABIValue::Array(rust_values))
            .unwrap();
        ABIValue { bytes, abi_type }
    }

    #[uniffi::constructor]
    pub fn static_array(element_type: String, size: u64, values: Vec<Arc<ABIValue>>) -> Self {
        let abi_type = format!("{}[{}]", element_type, size);
        let rust_abi_type = RustABIType::from_str(&abi_type).unwrap();
        let rust_values: Vec<algokit_abi::ABIValue> = values
            .into_iter()
            .map(|v| {
                let rust_abi_type = RustABIType::from_str(&element_type).unwrap();
                rust_abi_type.decode(&v.encoded_bytes()).unwrap()
            })
            .collect();
        let bytes = rust_abi_type
            .encode(&algokit_abi::ABIValue::Array(rust_values))
            .unwrap();
        ABIValue { bytes, abi_type }
    }
}
