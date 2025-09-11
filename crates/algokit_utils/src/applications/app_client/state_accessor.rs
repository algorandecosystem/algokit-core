use super::{AppClient, AppClientError};
use crate::clients::app_manager::AppState;
use algokit_abi::arc56_contract::ABIStorageKey;
use algokit_abi::{ABIType, ABIValue, StorageMap};
use base64::Engine;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;

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

    pub fn global_state(&self) -> AppStateAccessor<'_> {
        let provider = GlobalStateProvider {
            client: self.client,
        };
        AppStateAccessor::new("global".to_string(), Box::new(provider))
    }

    pub fn local_state(&self, address: &str) -> AppStateAccessor<'_> {
        let provider = LocalStateProvider {
            client: self.client,
            address: address.to_string(),
        };
        AppStateAccessor::new("local".to_string(), Box::new(provider))
    }

    pub fn box_storage(&self) -> BoxStateAccessor<'a> {
        BoxStateAccessor {
            client: self.client,
        }
    }
}

type GetStateResult = Result<HashMap<Vec<u8>, AppState>, AppClientError>;

pub trait StateProvider {
    fn get_app_state(&self) -> Pin<Box<dyn Future<Output = GetStateResult> + '_>>;
    fn get_storage_keys(&self) -> Result<HashMap<String, ABIStorageKey>, AppClientError>;
    fn get_storage_maps(&self) -> HashMap<String, StorageMap>;
}

struct GlobalStateProvider<'a> {
    client: &'a AppClient,
}

impl<'a> StateProvider for GlobalStateProvider<'a> {
    fn get_app_state(&self) -> Pin<Box<dyn Future<Output = GetStateResult> + '_>> {
        Box::pin(self.client.get_global_state())
    }

    fn get_storage_keys(&self) -> Result<HashMap<String, ABIStorageKey>, AppClientError> {
        self.client
            .app_spec
            .get_global_abi_storage_keys()
            .map_err(|e| AppClientError::ABIError { source: e })
    }

    fn get_storage_maps(&self) -> HashMap<String, StorageMap> {
        self.client.app_spec.state.maps.global_state.clone()
    }
}

struct LocalStateProvider<'a> {
    client: &'a AppClient,
    address: String,
}

impl<'a> StateProvider for LocalStateProvider<'a> {
    fn get_app_state(&self) -> Pin<Box<dyn Future<Output = GetStateResult> + '_>> {
        let addr = self.address.clone();
        let client = self.client;
        Box::pin(async move { client.get_local_state(&addr).await })
    }

    fn get_storage_keys(&self) -> Result<HashMap<String, ABIStorageKey>, AppClientError> {
        self.client
            .app_spec
            .get_local_abi_storage_keys()
            .map_err(|e| AppClientError::ABIError { source: e })
    }

    fn get_storage_maps(&self) -> HashMap<String, StorageMap> {
        self.client.app_spec.state.maps.local_state.clone()
    }
}

pub struct AppStateAccessor<'a> {
    name: String,
    provider: Box<dyn StateProvider + 'a>,
}

impl<'a> AppStateAccessor<'a> {
    pub fn new(name: String, provider: Box<dyn StateProvider + 'a>) -> Self {
        Self { name, provider }
    }

    pub async fn get_all(&self) -> Result<HashMap<String, Option<ABIValue>>, AppClientError> {
        let state = self.provider.get_app_state().await?;
        let storage_key_map = self.provider.get_storage_keys()?;

        let mut result = HashMap::new();
        for (key_name, storage_key) in storage_key_map {
            let abi_value = self.decode_storage_key(&key_name, &storage_key, &state)?;
            result.insert(key_name, abi_value);
        }
        Ok(result)
    }

    pub async fn get_value(&self, key_name: &str) -> Result<Option<ABIValue>, AppClientError> {
        let state = self.provider.get_app_state().await?;
        let storage_key_map = self.provider.get_storage_keys()?;

        let storage_key =
            storage_key_map
                .get(key_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("{} state key '{}' not found", self.name, key_name),
                })?;

        self.decode_storage_key(key_name, storage_key, &state)
    }

    fn decode_storage_key(
        &self,
        key_name: &str,
        storage_key: &ABIStorageKey,
        state: &HashMap<Vec<u8>, AppState>,
    ) -> Result<Option<ABIValue>, AppClientError> {
        let key_bytes = base64::engine::general_purpose::STANDARD
            .decode(&storage_key.key)
            .map_err(|e| AppClientError::AppStateError {
                message: format!("Failed to decode {} key '{}': {}", self.name, key_name, e),
            })?;

        let value = state.get(&key_bytes);

        match value {
            None => Ok(None),
            Some(app_state) => Ok(Some(decode_app_state(&storage_key.value_type, app_state)?)),
        }
    }

    pub async fn get_map(
        &self,
        map_name: &str,
    ) -> Result<HashMap<ABIValue, ABIValue>, AppClientError> {
        let state = self.provider.get_app_state().await?;
        let storage_map_map = self.provider.get_storage_maps();
        let storage_map =
            storage_map_map
                .get(map_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("{} state map '{}' not found", self.name, map_name),
                })?;
        let prefix_bytes = if let Some(prefix_b64) = &storage_map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| AppClientError::AppStateError {
                    message: format!("Failed to decode map prefix: {}", e),
                })?
        } else {
            Vec::new()
        };

        let mut result = HashMap::new();
        for (key, app_state) in state.iter() {
            if !key.starts_with(&prefix_bytes) {
                continue;
            }

            let tail = &key[prefix_bytes.len()..];
            let key_type = ABIType::from_str(&storage_map.key_type)
                .map_err(|e| AppClientError::ABIError { source: e })?;
            let decoded_key = key_type
                .decode(tail)
                .map_err(|e| AppClientError::ABIError { source: e })?;

            let value_type = ABIType::from_str(&storage_map.value_type)
                .map_err(|e| AppClientError::ABIError { source: e })?;
            let decoded_value = decode_app_state(&value_type, &app_state)?;
            result.insert(decoded_key, decoded_value);
        }

        Ok(result)
    }

    pub async fn get_map_value(
        &self,
        map_name: &str,
        key: ABIValue,
    ) -> Result<Option<ABIValue>, AppClientError> {
        let state = self.provider.get_app_state().await?;
        let storage_map_map = self.provider.get_storage_maps();
        let storage_map =
            storage_map_map
                .get(map_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("{} state map '{}' not found", self.name, map_name),
                })?;

        let prefix_bytes = if let Some(prefix_b64) = &storage_map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| AppClientError::AppStateError {
                    message: format!("Failed to decode map prefix: {}", e),
                })?
        } else {
            Vec::new()
        };
        let encoded_key = ABIType::from_str(&storage_map.key_type)
            .map_err(|e| AppClientError::ABIError { source: e })?
            .encode(&key)
            .map_err(|e| AppClientError::ABIError { source: e })?;
        let full_key = [prefix_bytes, encoded_key].concat();

        let value = state.get(&full_key);

        match value {
            None => Ok(None),
            Some(app_state) => {
                let value_type = ABIType::from_str(&storage_map.value_type)
                    .map_err(|e| AppClientError::ABIError { source: e })?;
                Ok(Some(decode_app_state(&value_type, app_state)?))
            }
        }
    }
}

impl<'a> BoxStateAccessor<'a> {
    pub async fn get_all(&self) -> Result<HashMap<String, ABIValue>, AppClientError> {
        let box_keys = &self.client.app_spec.state.keys.box_keys;
        let mut results: HashMap<String, ABIValue> = HashMap::new();

        for (box_name, storage_key) in box_keys {
            let box_name_bytes = base64::engine::general_purpose::STANDARD
                .decode(&storage_key.key)
                .map_err(|e| AppClientError::AppStateError {
                    message: format!("Failed to decode box key '{}': {}", box_name, e),
                })?;

            // TODO: what to do when it failed to fetch the box?
            let box_value = self.client.get_box_value(&box_name_bytes).await?;
            let value_type = ABIType::from_str(&storage_key.value_type)
                .map_err(|e| AppClientError::ABIError { source: e })?;
            let abi_value = value_type
                .decode(&box_value)
                .map_err(|e| AppClientError::ABIError { source: e })?;
            results.insert(box_name.clone(), abi_value);
        }

        return Ok(results);
    }

    pub async fn get_value(&self, name: &str) -> Result<ABIValue, AppClientError> {
        let storage_key = self
            .client
            .app_spec
            .state
            .keys
            .box_keys
            .get(name)
            .ok_or_else(|| AppClientError::AppStateError {
                message: format!("Box key '{}' not found", name),
            })?;

        let box_name_bytes = base64::engine::general_purpose::STANDARD
            .decode(&storage_key.key)
            .map_err(|e| AppClientError::AppStateError {
                message: format!("Failed to decode box key '{}': {}", name, e),
            })?;

        // TODO: what to do when it failed to fetch the box?
        let box_value = self.client.get_box_value(&box_name_bytes).await?;
        let value_type = ABIType::from_str(&storage_key.value_type)
            .map_err(|e| AppClientError::ABIError { source: e })?;
        return value_type
            .decode(&box_value)
            .map_err(|e| AppClientError::ABIError { source: e });
    }

    pub async fn get_map(
        &self,
        map_name: &str,
    ) -> Result<HashMap<ABIValue, ABIValue>, AppClientError> {
        let storage_map_map = &self.client.app_spec.state.maps.box_maps;
        let storage_map =
            storage_map_map
                .get(map_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("Box map '{}' not found", map_name),
                })?;

        let prefix_bytes = if let Some(prefix_b64) = &storage_map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| AppClientError::AppStateError {
                    message: format!("Failed to decode map prefix: {}", e),
                })?
        } else {
            Vec::new()
        };

        let box_names = self.client.get_box_names().await?;
        let box_names = box_names
            .iter()
            .filter(|box_name| box_name.name_raw.starts_with(&prefix_bytes))
            .collect::<Vec<_>>();

        let mut results: HashMap<ABIValue, ABIValue> = HashMap::new();
        for box_name in box_names {
            let tail = &box_name.name_raw[prefix_bytes.len()..];
            let key_type = ABIType::from_str(&storage_map.key_type)
                .map_err(|e| AppClientError::ABIError { source: e })?;
            let decoded_key = key_type
                .decode(tail)
                .map_err(|e| AppClientError::ABIError { source: e })?;

            let box_value = self.client.get_box_value(&box_name.name_raw).await?;
            let value_type = ABIType::from_str(&storage_map.value_type)
                .map_err(|e| AppClientError::ABIError { source: e })?;
            let decoded_value = value_type
                .decode(&box_value)
                .map_err(|e| AppClientError::ABIError { source: e })?;
            results.insert(decoded_key, decoded_value);
        }

        Ok(results)
    }
}

fn decode_app_state(
    value_type: &ABIType,
    app_state: &AppState,
) -> Result<ABIValue, AppClientError> {
    return match &app_state {
        AppState::Uint(uint_app_state) => Ok(ABIValue::Uint(BigUint::from(uint_app_state.value))),
        AppState::Bytes(bytes_app_state) => Ok(value_type
            .decode(&bytes_app_state.value_raw)
            .map_err(|e| AppClientError::ABIError { source: e })?),
    };
}
