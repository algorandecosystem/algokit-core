use super::{AppClient, AppClientError};
use algokit_abi::arc56_contract::{AVM_BYTES, AVM_STRING};
use algokit_abi::{ABIType, ABIValue};
use base64::Engine;
use std::collections::HashMap;
use std::str::FromStr;

pub struct GlobalStateAccessor<'a> {
    client: &'a AppClient,
}

pub struct LocalStateAccessor<'a> {
    client: &'a AppClient,
    address: String,
}

pub struct BoxStateAccessor<'a> {
    client: &'a AppClient,
}

pub struct StateAccessor<'a> {
    pub(crate) client: &'a AppClient,
}

impl<'a> StateAccessor<'a> {
    pub fn new(client: &'a AppClient) -> Self {
        Self { client }
    }

    pub fn global_state(&self) -> GlobalStateAccessor<'a> {
        GlobalStateAccessor {
            client: self.client,
        }
    }
    pub fn local_state(&self, address: &str) -> LocalStateAccessor<'a> {
        LocalStateAccessor {
            client: self.client,
            address: address.to_string(),
        }
    }
    pub fn box_storage(&self) -> BoxStateAccessor<'a> {
        BoxStateAccessor {
            client: self.client,
        }
    }
}

impl GlobalStateAccessor<'_> {
    pub async fn get_all(&self) -> Result<HashMap<String, ABIValue>, AppClientError> {
        let state = self.client.get_global_state().await?;
        let mut result = HashMap::new();
        for (name, metadata) in &self.client.app_spec.state.keys.global_state {
            // decode key and fetch value
            let key_bytes = base64::engine::general_purpose::STANDARD
                .decode(&metadata.key)
                .map_err(|e| {
                    AppClientError::ValidationError(format!(
                        "Failed to decode global key '{}': {}",
                        name, e
                    ))
                })?;
            let app_state = state.get(&key_bytes).ok_or_else(|| {
                AppClientError::ValidationError(format!(
                    "Global state key '{}' not found in app state",
                    name
                ))
            })?;
            let abi_value = decode_app_state_value(&metadata.value_type, app_state)?;
            result.insert(name.clone(), abi_value);
        }
        Ok(result)
    }

    pub async fn get_value(&self, name: &str) -> Result<ABIValue, AppClientError> {
        let metadata = self
            .client
            .app_spec
            .state
            .keys
            .global_state
            .get(name)
            .ok_or_else(|| {
                AppClientError::ValidationError(format!("Unknown global state key: {}", name))
            })?;
        let key_bytes = base64::engine::general_purpose::STANDARD
            .decode(&metadata.key)
            .map_err(|e| {
                AppClientError::ValidationError(format!(
                    "Failed to decode global key '{}': {}",
                    name, e
                ))
            })?;
        let state = self.client.get_global_state().await?;
        let app_state = state.get(&key_bytes).ok_or_else(|| {
            AppClientError::ValidationError(format!(
                "Global state key '{}' not found in app state",
                name
            ))
        })?;
        decode_app_state_value(&metadata.value_type, app_state)
    }

    pub async fn get_map_value(
        &self,
        map_name: &str,
        key: &ABIValue,
    ) -> Result<ABIValue, AppClientError> {
        let map = self
            .client
            .app_spec
            .state
            .maps
            .global_state
            .get(map_name)
            .ok_or_else(|| {
                AppClientError::ValidationError(format!("Unknown global map: {}", map_name))
            })?;
        let key_type = ABIType::from_str(&map.key_type).map_err(|e| {
            AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.key_type, e))
        })?;
        let key_bytes = key_type.encode(key).map_err(|e| {
            AppClientError::ValidationError(format!("Failed to encode map key: {}", e))
        })?;
        let mut full_key = if let Some(prefix_b64) = &map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| {
                    AppClientError::ValidationError(format!("Failed to decode map prefix: {}", e))
                })?
        } else {
            Vec::new()
        };
        full_key.extend_from_slice(&key_bytes);

        let state = self.client.get_global_state().await?;
        let app_state = state.get(&full_key).ok_or_else(|| {
            AppClientError::ValidationError(format!("Global map '{}' key not found", map_name))
        })?;
        decode_app_state_value(&map.value_type, app_state)
    }

    pub async fn get_map(
        &self,
        map_name: &str,
    ) -> Result<HashMap<String, ABIValue>, AppClientError> {
        let map = self
            .client
            .app_spec
            .state
            .maps
            .global_state
            .get(map_name)
            .ok_or_else(|| {
                AppClientError::ValidationError(format!("Unknown global map: {}", map_name))
            })?;
        let prefix_bytes = if let Some(prefix_b64) = &map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| {
                    AppClientError::ValidationError(format!("Failed to decode map prefix: {}", e))
                })?
        } else {
            Vec::new()
        };
        let key_type = ABIType::from_str(&map.key_type).map_err(|e| {
            AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.key_type, e))
        })?;

        let mut result = HashMap::new();
        let state = self.client.get_global_state().await?;
        for (key_raw, app_state) in state.iter() {
            if !key_raw.starts_with(&prefix_bytes) {
                continue;
            }
            let tail = &key_raw[prefix_bytes.len()..];
            // Decode the map key tail according to ABI type, error if invalid
            let decoded_key = key_type.decode(tail).map_err(|e| {
                AppClientError::ABIError(format!(
                    "Failed to decode key for map '{}': {}",
                    map_name, e
                ))
            })?;
            let key_str = abi_value_to_string(&decoded_key);
            let value = decode_app_state_value(&map.value_type, app_state)?;
            result.insert(key_str, value);
        }
        Ok(result)
    }
}

impl LocalStateAccessor<'_> {
    pub async fn get_all(&self) -> Result<HashMap<String, ABIValue>, AppClientError> {
        let state = self.client.get_local_state(&self.address).await?;
        let mut result = HashMap::new();
        for (name, metadata) in &self.client.app_spec.state.keys.local_state {
            let key_bytes = base64::engine::general_purpose::STANDARD
                .decode(&metadata.key)
                .map_err(|e| {
                    AppClientError::ValidationError(format!(
                        "Failed to decode local key '{}': {}",
                        name, e
                    ))
                })?;
            let app_state = state.get(&key_bytes).ok_or_else(|| {
                AppClientError::ValidationError(format!(
                    "Local state key '{}' not found for address {}",
                    name, self.address
                ))
            })?;
            let abi_value = decode_app_state_value(&metadata.value_type, app_state)?;
            result.insert(name.clone(), abi_value);
        }
        Ok(result)
    }

    pub async fn get_value(&self, name: &str) -> Result<ABIValue, AppClientError> {
        let metadata = self
            .client
            .app_spec
            .state
            .keys
            .local_state
            .get(name)
            .ok_or_else(|| {
                AppClientError::ValidationError(format!("Unknown local state key: {}", name))
            })?;
        let key_bytes = base64::engine::general_purpose::STANDARD
            .decode(&metadata.key)
            .map_err(|e| {
                AppClientError::ValidationError(format!(
                    "Failed to decode local key '{}': {}",
                    name, e
                ))
            })?;
        let state = self.client.get_local_state(&self.address).await?;
        let app_state = state.get(&key_bytes).ok_or_else(|| {
            AppClientError::ValidationError(format!(
                "Local state key '{}' not found for address {}",
                name, self.address
            ))
        })?;
        decode_app_state_value(&metadata.value_type, app_state)
    }

    pub async fn get_map_value(
        &self,
        map_name: &str,
        key: &ABIValue,
    ) -> Result<ABIValue, AppClientError> {
        let map = self
            .client
            .app_spec
            .state
            .maps
            .local_state
            .get(map_name)
            .ok_or_else(|| {
                AppClientError::ValidationError(format!("Unknown local map: {}", map_name))
            })?;
        let key_type = ABIType::from_str(&map.key_type).map_err(|e| {
            AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.key_type, e))
        })?;
        let key_bytes = key_type.encode(key).map_err(|e| {
            AppClientError::ValidationError(format!("Failed to encode map key: {}", e))
        })?;
        let mut full_key = if let Some(prefix_b64) = &map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| {
                    AppClientError::ValidationError(format!("Failed to decode map prefix: {}", e))
                })?
        } else {
            Vec::new()
        };
        full_key.extend_from_slice(&key_bytes);

        let state = self.client.get_local_state(&self.address).await?;
        let app_state = state.get(&full_key).ok_or_else(|| {
            AppClientError::ValidationError(format!("Local map '{}' key not found", map_name))
        })?;
        decode_app_state_value(&map.value_type, app_state)
    }

    pub async fn get_map(
        &self,
        map_name: &str,
    ) -> Result<HashMap<String, ABIValue>, AppClientError> {
        let map = self
            .client
            .app_spec
            .state
            .maps
            .local_state
            .get(map_name)
            .ok_or_else(|| {
                AppClientError::ValidationError(format!("Unknown local map: {}", map_name))
            })?;
        let prefix_bytes = if let Some(prefix_b64) = &map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| {
                    AppClientError::ValidationError(format!("Failed to decode map prefix: {}", e))
                })?
        } else {
            Vec::new()
        };
        let key_type = ABIType::from_str(&map.key_type).map_err(|e| {
            AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.key_type, e))
        })?;

        let mut result = HashMap::new();
        let state = self.client.get_local_state(&self.address).await?;
        for (key_raw, app_state) in state.iter() {
            if !key_raw.starts_with(&prefix_bytes) {
                continue;
            }
            let tail = &key_raw[prefix_bytes.len()..];
            let decoded_key = key_type.decode(tail).map_err(|e| {
                AppClientError::ABIError(format!(
                    "Failed to decode key for map '{}': {}",
                    map_name, e
                ))
            })?;
            let key_str = abi_value_to_string(&decoded_key);
            let value = decode_app_state_value(&map.value_type, app_state)?;
            result.insert(key_str, value);
        }
        Ok(result)
    }
}

impl BoxStateAccessor<'_> {
    pub async fn get_value(&self, name: &str) -> Result<ABIValue, AppClientError> {
        let metadata = self
            .client
            .app_spec
            .state
            .keys
            .box_keys
            .get(name)
            .ok_or_else(|| AppClientError::ValidationError(format!("Unknown box key: {}", name)))?;
        let box_name = base64::engine::general_purpose::STANDARD
            .decode(&metadata.key)
            .map_err(|e| {
                AppClientError::ValidationError(format!(
                    "Failed to decode box key '{}': {}",
                    name, e
                ))
            })?;
        let abi_type = ABIType::from_str(&metadata.value_type).map_err(|e| {
            AppClientError::ABIError(format!("Invalid ABI type '{}': {}", metadata.value_type, e))
        })?;
        self.client
            .algorand()
            .app()
            .get_box_value_from_abi_type(
                self.client.app_id().ok_or(AppClientError::ValidationError(
                    "Missing app_id".to_string(),
                ))?,
                &box_name,
                &abi_type,
            )
            .await
            .map_err(|e| AppClientError::AppManagerError(e.to_string()))
    }

    pub async fn get_map_value(
        &self,
        map_name: &str,
        key: &ABIValue,
    ) -> Result<ABIValue, AppClientError> {
        let map = self
            .client
            .app_spec
            .state
            .maps
            .box_maps
            .get(map_name)
            .ok_or_else(|| {
                AppClientError::ValidationError(format!("Unknown box map: {}", map_name))
            })?;
        let key_type = ABIType::from_str(&map.key_type).map_err(|e| {
            AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.key_type, e))
        })?;
        let key_bytes = key_type.encode(key).map_err(|e| {
            AppClientError::ValidationError(format!("Failed to encode map key: {}", e))
        })?;
        let mut full_key = if let Some(prefix_b64) = &map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| {
                    AppClientError::ValidationError(format!("Failed to decode map prefix: {}", e))
                })?
        } else {
            Vec::new()
        };
        full_key.extend_from_slice(&key_bytes);
        let value_type = ABIType::from_str(&map.value_type).map_err(|e| {
            AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.value_type, e))
        })?;
        self.client
            .algorand()
            .app()
            .get_box_value_from_abi_type(
                self.client.app_id().ok_or(AppClientError::ValidationError(
                    "Missing app_id".to_string(),
                ))?,
                &full_key,
                &value_type,
            )
            .await
            .map_err(|e| AppClientError::AppManagerError(e.to_string()))
    }

    pub async fn get_map(
        &self,
        map_name: &str,
    ) -> Result<HashMap<String, ABIValue>, AppClientError> {
        let map = self
            .client
            .app_spec
            .state
            .maps
            .box_maps
            .get(map_name)
            .ok_or_else(|| {
                AppClientError::ValidationError(format!("Unknown box map: {}", map_name))
            })?;
        let prefix_bytes = if let Some(prefix_b64) = &map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| {
                    AppClientError::ValidationError(format!("Failed to decode map prefix: {}", e))
                })?
        } else {
            Vec::new()
        };

        let key_type = ABIType::from_str(&map.key_type).map_err(|e| {
            AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.key_type, e))
        })?;
        let value_type = ABIType::from_str(&map.value_type).map_err(|e| {
            AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.value_type, e))
        })?;

        let mut result = HashMap::new();
        let box_names = self.client.get_box_names().await?;
        for box_name in box_names {
            if !box_name.name_raw.starts_with(&prefix_bytes) {
                continue;
            }
            let tail = &box_name.name_raw[prefix_bytes.len()..];
            let decoded_key = key_type.decode(tail).map_err(|e| {
                AppClientError::ABIError(format!(
                    "Failed to decode key for map '{}': {}",
                    map_name, e
                ))
            })?;
            let key_str = abi_value_to_string(&decoded_key);
            let val = self
                .client
                .algorand()
                .app()
                .get_box_value_from_abi_type(
                    self.client.app_id().ok_or(AppClientError::ValidationError(
                        "Missing app_id".to_string(),
                    ))?,
                    &box_name.name_raw,
                    &value_type,
                )
                .await
                .map_err(|e| AppClientError::AppManagerError(e.to_string()))?;
            result.insert(key_str, val);
        }
        Ok(result)
    }
}

pub(crate) fn decode_app_state_value(
    value_type_str: &str,
    app_state: &crate::clients::app_manager::AppState,
) -> Result<ABIValue, AppClientError> {
    match &app_state.value {
        crate::clients::app_manager::AppStateValue::Uint(u) => {
            // For integer types, convert to ABIValue::Uint directly
            let big = num_bigint::BigUint::from(*u);
            Ok(ABIValue::Uint(big))
        }
        crate::clients::app_manager::AppStateValue::Bytes(_) => {
            // Special-case AVM native types
            let raw = app_state.value_raw.clone().ok_or_else(|| {
                AppClientError::ValidationError(
                    "Missing raw bytes for bytes state value".to_string(),
                )
            })?;

            if value_type_str == AVM_STRING {
                let s = String::from_utf8_lossy(&raw).to_string();
                // Attempt to treat ASCII as base64-encoded string then fall back
                if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(s.trim()) {
                    if let Ok(decoded_str) = String::from_utf8(decoded.clone()) {
                        return Ok(ABIValue::from(decoded_str));
                    }
                }
                return Ok(ABIValue::from(s));
            }
            if value_type_str == AVM_BYTES {
                // Try to interpret raw as base64 string first, then fall back.
                if let Ok(ascii) = String::from_utf8(raw.clone()) {
                    if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(&ascii) {
                        if let Ok(decoded_str) = String::from_utf8(decoded.clone()) {
                            return Ok(ABIValue::from(decoded_str));
                        } else {
                            let arr = decoded.into_iter().map(ABIValue::from_byte).collect();
                            return Ok(ABIValue::Array(arr));
                        }
                    }
                    // Not base64; treat UTF-8 bytes as string
                    return Ok(ABIValue::from(ascii));
                }
                let arr = raw.into_iter().map(ABIValue::from_byte).collect();
                return Ok(ABIValue::Array(arr));
            }

            // Fallback to ABI decoding for declared ARC-4 types (includes structs)
            let abi_type = ABIType::from_str(value_type_str).map_err(|e| {
                AppClientError::ABIError(format!("Invalid ABI type '{}': {}", value_type_str, e))
            })?;
            abi_type.decode(&raw).map_err(|e| {
                AppClientError::ABIError(format!("Failed to decode state value: {}", e))
            })
        }
    }
}

fn abi_value_to_string(value: &ABIValue) -> String {
    match value {
        ABIValue::Bool(b) => b.to_string(),
        ABIValue::Uint(u) => u.to_string(),
        ABIValue::String(s) => s.clone(),
        ABIValue::Byte(b) => b.to_string(),
        ABIValue::Address(addr) => addr.clone(),
        ABIValue::Array(arr) => {
            let inner: Vec<String> = arr.iter().map(abi_value_to_string).collect();
            format!("[{}]", inner.join(","))
        }
        ABIValue::Struct(map) => {
            // Render deterministic order by key for stability
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let inner: Vec<String> = keys
                .into_iter()
                .map(|k| format!("{}:{}", k, abi_value_to_string(&map[k])))
                .collect();
            format!("{{{}}}", inner.join(","))
        }
    }
}
