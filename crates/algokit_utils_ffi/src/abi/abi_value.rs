use std::sync::Arc;

use algokit_abi::ABIValue as RustABIValue;

use crate::transactions::common::UtilsError;

// NOTE: Once we get a release that enables custom types with Python (it's on main), we can use them to provide a better ux: https://github.com/mozilla/uniffi-rs/issues/2652#issuecomment-3307297845

#[derive(uniffi::Object, Debug, Clone, PartialEq)]
#[uniffi::export(Eq)]
pub struct ABIValue {
    pub rust_value: RustABIValue,
}

#[uniffi::export]
impl ABIValue {
    #[uniffi::constructor]
    pub fn bool(value: bool) -> Self {
        ABIValue {
            rust_value: RustABIValue::Bool(value),
        }
    }

    pub fn get_bool(&self) -> Result<bool, UtilsError> {
        if let RustABIValue::Bool(b) = &self.rust_value {
            Ok(*b)
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not a bool".to_string(),
            })
        }
    }

    #[uniffi::constructor]
    pub fn array(values: Vec<Arc<ABIValue>>) -> Self {
        ABIValue {
            rust_value: RustABIValue::Array(
                values.into_iter().map(|v| v.rust_value.clone()).collect(),
            ),
        }
    }

    pub fn get_array(&self) -> Result<Vec<Arc<ABIValue>>, UtilsError> {
        if let RustABIValue::Array(arr) = &self.rust_value {
            Ok(arr
                .iter()
                .cloned()
                .map(|v| Arc::new(ABIValue { rust_value: v }))
                .collect())
        } else {
            Err(UtilsError::UtilsError {
                message: "ABI value is not an array".to_string(),
            })
        }
    }
}
