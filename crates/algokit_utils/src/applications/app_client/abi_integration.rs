use algokit_abi::{ABIType, ABIValue};
use base64::Engine;
use std::collections::HashMap;

use super::AppClient;
use super::error::AppClientError;

impl AppClient {
    async fn get_abi_decoded_value(
        &self,
        key: &[u8],
        state: &HashMap<Vec<u8>, crate::clients::app_manager::AppState>,
        abi_type_str: &str,
        default_value_type: Option<&str>,
    ) -> Result<ABIValue, AppClientError> {
        let app_state = state.get(key).ok_or_else(|| {
            AppClientError::ValidationError(format!("State key not found: {:?}", key))
        })?;
        let effective_type = default_value_type.unwrap_or(abi_type_str);
        super::state_accessor::decode_app_state_value(effective_type, app_state)
    }
}
