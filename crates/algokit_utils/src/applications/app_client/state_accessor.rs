use crate::clients::app_manager::{AppState, BytesAppState};

use super::{AppClient, AppClientError};
use algokit_abi::arc56_contract::{AVM_BYTES, AVM_STRING};
use algokit_abi::{ABIType, ABIValue, AVM_UINT64, StorageKey, StorageMap};
use base64::Engine;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
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

    pub fn global_state(&self) -> AppStateAccessor<'_> {
        let client = self.client;
        AppStateAccessor::new(
            "global".to_string(),
            Box::new(move || Box::pin(client.get_global_state())),
            Box::new(move || client.app_spec.state.keys.global_state.clone()),
            Box::new(move || client.app_spec.state.maps.global_state.clone()),
        )
    }

    pub fn local_state(&self, address: &str) -> AppStateAccessor<'_> {
        let client = self.client;
        let address = address.to_string();
        AppStateAccessor::new(
            "local".to_string(),
            Box::new(move || {
                let addr = address.clone();
                Box::pin(async move { client.get_local_state(&addr).await })
            }),
            Box::new(move || client.app_spec.state.keys.local_state.clone()),
            Box::new(move || client.app_spec.state.maps.local_state.clone()),
        )
    }

    pub fn box_storage(&self) -> BoxStateAccessor<'a> {
        BoxStateAccessor {
            client: self.client,
        }
    }
}

// TODO: simplify this
pub struct AppStateAccessor<'a> {
    name: String,
    get_app_state: Box<
        dyn Fn() -> Pin<
                Box<dyn Future<Output = Result<HashMap<Vec<u8>, AppState>, AppClientError>> + 'a>,
            > + 'a,
    >,
    get_storage_key: Box<dyn Fn() -> HashMap<String, StorageKey> + 'a>,
    get_storage_map: Box<dyn Fn() -> HashMap<String, StorageMap> + 'a>,
}

impl<'a> AppStateAccessor<'a> {
    pub fn new(
        name: String,
        get_app_state: Box<
            dyn Fn() -> Pin<
                    Box<
                        dyn Future<Output = Result<HashMap<Vec<u8>, AppState>, AppClientError>>
                            + 'a,
                    >,
                > + 'a,
        >,
        get_storage_key: Box<dyn Fn() -> HashMap<String, StorageKey> + 'a>,
        get_storage_map: Box<dyn Fn() -> HashMap<String, StorageMap> + 'a>,
    ) -> Self {
        Self {
            name,
            get_app_state,
            get_storage_key,
            get_storage_map,
        }
    }

    pub async fn get_all(&self) -> Result<HashMap<String, Option<ABIValue>>, AppClientError> {
        let state = (self.get_app_state)().await?;
        let storage_key_map = (self.get_storage_key)();

        let mut result = HashMap::new();
        for (key_name, storage_key) in storage_key_map {
            let abi_value = self.decode_storage_key(&key_name, &storage_key, &state)?;
            result.insert(key_name, abi_value);
        }
        Ok(result)
    }

    pub async fn get_value(&self, key_name: &str) -> Result<Option<ABIValue>, AppClientError> {
        let state = (self.get_app_state)().await?;
        let storage_key_map = (self.get_storage_key)();

        let storage_key =
            storage_key_map
                .get(key_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("{} state key {} not found", self.name, key_name),
                })?;

        self.decode_storage_key(key_name, storage_key, &state)
    }

    fn decode_storage_key(
        &self,
        key_name: &str,
        storage_key: &StorageKey,
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
            Some(app_state) => Ok(Some(decode_app_state(&storage_key.key_type, app_state)?)),
        }
    }

    pub async fn get_map(
        &self,
        map_name: &str,
    ) -> Result<HashMap<ABIValue, ABIValue>, AppClientError> {
        let state = (self.get_app_state)().await?;
        let storage_map_map = (self.get_storage_map)();
        let storage_map =
            storage_map_map
                .get(map_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("{} state map {} not found", self.name, map_name),
                })?;
        return self.decode_storage_map(storage_map, &state);
    }

    pub async fn get_map_value(
        &self,
        map_name: &str,
        key: ABIValue,
    ) -> Result<Option<ABIValue>, AppClientError> {
        let state = (self.get_app_state)().await?;
        let storage_map_map = (self.get_storage_map)();
        let storage_map =
            storage_map_map
                .get(map_name)
                .ok_or_else(|| AppClientError::AppStateError {
                    message: format!("{} state map {} not found", self.name, map_name),
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
            Some(app_state) => Ok(Some(decode_app_state(&storage_map.value_type, app_state)?)),
        }
    }

    fn decode_storage_map(
        &self,
        storage_map: &StorageMap,
        state: &HashMap<Vec<u8>, AppState>,
    ) -> Result<HashMap<ABIValue, ABIValue>, AppClientError> {
        let prefix_bytes = if let Some(prefix_b64) = &storage_map.prefix {
            base64::engine::general_purpose::STANDARD
                .decode(prefix_b64)
                .map_err(|e| AppClientError::AppStateError {
                    message: format!("Failed to decode map prefix: {}", e),
                })?
        } else {
            Vec::new()
        };

        let key_type = ABIType::from_str(&storage_map.key_type)
            .map_err(|e| AppClientError::ABIError { source: e })?;

        let mut result = HashMap::new();
        for (key, app_state) in state.iter() {
            if !key.starts_with(&prefix_bytes) {
                continue;
            }

            let tail = &key[prefix_bytes.len()..];
            let decoded_key: ABIValue = key_type
                .decode(tail)
                .map_err(|e| AppClientError::ABIError { source: e })?;

            let value = decode_app_state(&storage_map.value_type, &app_state)?;
            result.insert(decoded_key, value);
        }

        Ok(result)
    }
}

// impl BoxStateAccessor<'_> {
//     pub async fn get_value(&self, name: &str) -> Result<ABIValue, AppClientError> {
//         let metadata = self
//             .client
//             .app_spec
//             .state
//             .keys
//             .box_keys
//             .get(name)
//             .ok_or_else(|| AppClientError::ValidationError(format!("Unknown box key: {}", name)))?;
//         let box_name = base64::engine::general_purpose::STANDARD
//             .decode(&metadata.key)
//             .map_err(|e| {
//                 AppClientError::ValidationError(format!(
//                     "Failed to decode box key '{}': {}",
//                     name, e
//                 ))
//             })?;
//         let abi_type = ABIType::from_str(&metadata.value_type).map_err(|e| {
//             AppClientError::ABIError(format!("Invalid ABI type '{}': {}", metadata.value_type, e))
//         })?;
//         self.client
//             .algorand()
//             .app()
//             .get_box_value_from_abi_type(
//                 self.client.app_id().ok_or(AppClientError::ValidationError(
//                     "Missing app_id".to_string(),
//                 ))?,
//                 &box_name,
//                 &abi_type,
//             )
//             .await
//             .map_err(|e| AppClientError::AppManagerError(e.to_string()))
//     }

//     pub async fn get_map_value(
//         &self,
//         map_name: &str,
//         key: &ABIValue,
//     ) -> Result<ABIValue, AppClientError> {
//         let map = self
//             .client
//             .app_spec
//             .state
//             .maps
//             .box_maps
//             .get(map_name)
//             .ok_or_else(|| {
//                 AppClientError::ValidationError(format!("Unknown box map: {}", map_name))
//             })?;
//         let key_type = ABIType::from_str(&map.key_type).map_err(|e| {
//             AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.key_type, e))
//         })?;
//         let key_bytes = key_type.encode(key).map_err(|e| {
//             AppClientError::ValidationError(format!("Failed to encode map key: {}", e))
//         })?;
//         let mut full_key = if let Some(prefix_b64) = &map.prefix {
//             base64::engine::general_purpose::STANDARD
//                 .decode(prefix_b64)
//                 .map_err(|e| {
//                     AppClientError::ValidationError(format!("Failed to decode map prefix: {}", e))
//                 })?
//         } else {
//             Vec::new()
//         };
//         full_key.extend_from_slice(&key_bytes);
//         let value_type = ABIType::from_str(&map.value_type).map_err(|e| {
//             AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.value_type, e))
//         })?;
//         self.client
//             .algorand()
//             .app()
//             .get_box_value_from_abi_type(
//                 self.client.app_id().ok_or(AppClientError::ValidationError(
//                     "Missing app_id".to_string(),
//                 ))?,
//                 &full_key,
//                 &value_type,
//             )
//             .await
//             .map_err(|e| AppClientError::AppManagerError(e.to_string()))
//     }

//     pub async fn get_map(
//         &self,
//         map_name: &str,
//     ) -> Result<HashMap<String, ABIValue>, AppClientError> {
//         let map = self
//             .client
//             .app_spec
//             .state
//             .maps
//             .box_maps
//             .get(map_name)
//             .ok_or_else(|| {
//                 AppClientError::ValidationError(format!("Unknown box map: {}", map_name))
//             })?;
//         let prefix_bytes = if let Some(prefix_b64) = &map.prefix {
//             base64::engine::general_purpose::STANDARD
//                 .decode(prefix_b64)
//                 .map_err(|e| {
//                     AppClientError::ValidationError(format!("Failed to decode map prefix: {}", e))
//                 })?
//         } else {
//             Vec::new()
//         };

//         let key_type = ABIType::from_str(&map.key_type).map_err(|e| {
//             AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.key_type, e))
//         })?;
//         let value_type = ABIType::from_str(&map.value_type).map_err(|e| {
//             AppClientError::ABIError(format!("Invalid ABI type '{}': {}", map.value_type, e))
//         })?;

//         let mut result = HashMap::new();
//         let box_names = self.client.get_box_names().await?;
//         for box_name in box_names {
//             if !box_name.name_raw.starts_with(&prefix_bytes) {
//                 continue;
//             }
//             let tail = &box_name.name_raw[prefix_bytes.len()..];
//             let decoded_key = key_type.decode(tail).map_err(|e| {
//                 AppClientError::ABIError(format!(
//                     "Failed to decode key for map '{}': {}",
//                     map_name, e
//                 ))
//             })?;
//             let key_str = abi_value_to_string(&decoded_key);
//             let val = self
//                 .client
//                 .algorand()
//                 .app()
//                 .get_box_value_from_abi_type(
//                     self.client.app_id().ok_or(AppClientError::ValidationError(
//                         "Missing app_id".to_string(),
//                     ))?,
//                     &box_name.name_raw,
//                     &value_type,
//                 )
//                 .await
//                 .map_err(|e| AppClientError::AppManagerError(e.to_string()))?;
//             result.insert(key_str, val);
//         }
//         Ok(result)
//     }
// }

fn decode_app_state(value_type: &str, app_state: &AppState) -> Result<ABIValue, AppClientError> {
    return match &app_state {
        AppState::Uint(uint_app_state) => Ok(ABIValue::Uint(BigUint::from(uint_app_state.value))),
        AppState::Bytes(bytes_app_state) => {
            Ok(decode_app_state_bytes_value(&value_type, bytes_app_state)?)
        }
    };
}

fn decode_app_state_bytes_value(
    r#type: &str,
    value: &BytesAppState,
) -> Result<ABIValue, AppClientError> {
    if r#type == AVM_STRING {
        return Ok(ABIValue::from(value.value.clone()));
    }
    if r#type == AVM_BYTES {
        return Ok(ABIValue::Bytes(value.value_raw.clone()));
    }
    if r#type == AVM_UINT64 {
        return Ok(ABIType::from_str("uint64")
            .map_err(|e| AppClientError::ABIError { source: e })?
            .decode(&value.value_raw)
            .map_err(|e| AppClientError::ABIError { source: e })?);
    }

    // TODO: structs will be handled in another PR
    let abi_type = ABIType::from_str(r#type).map_err(|e| AppClientError::ABIError { source: e })?;

    abi_type
        .decode(&value.value_raw)
        .map_err(|e| AppClientError::ABIError { source: e })
}
