/// This module is to be later integrated into the abi crate to further simplify app client logic
/// For now, it consolidates main functionality not covered by the abi crate and required in app client
use algokit_abi::ABIMethod;
use algokit_abi::{ABIType, ABIValue};
use base64::Engine;
use std::collections::HashMap;
use std::str::FromStr;

use super::AppClient;
use super::error::AppClientError;
use crate::transactions::AppMethodCallArg;

impl AppClient {
    async fn resolve_default_value_for_arg_base(
        &self,
        default: &algokit_abi::arc56_contract::DefaultValue,
        abi_type_str: &str,
        sender: Option<&str>,
    ) -> Result<ABIValue, AppClientError> {
        use algokit_abi::arc56_contract::DefaultValueSource as Src;
        let abi_type = ABIType::from_str(abi_type_str).map_err(|e| {
            AppClientError::AbiError(format!("Invalid ABI type '{}': {}", abi_type_str, e))
        })?;
        match default.source {
            Src::Literal => {
                let raw = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| {
                        AppClientError::ValidationError(format!(
                            "Failed to decode base64 literal: {}",
                            e
                        ))
                    })?;
                if let Some(ref vt) = default.value_type {
                    if vt == algokit_abi::arc56_contract::AVM_STRING {
                        let s = String::from_utf8_lossy(&raw).to_string();
                        return Ok(ABIValue::from(s));
                    }
                    if vt == algokit_abi::arc56_contract::AVM_BYTES {
                        let arr = raw.into_iter().map(ABIValue::from_byte).collect();
                        return Ok(ABIValue::Array(arr));
                    }
                }
                let decode_type = if let Some(ref vt) = default.value_type {
                    ABIType::from_str(vt).map_err(|e| {
                        AppClientError::AbiError(format!(
                            "Invalid default value ABI type '{}': {}",
                            vt, e
                        ))
                    })?
                } else {
                    abi_type.clone()
                };
                decode_type.decode(&raw).map_err(|e| {
                    AppClientError::AbiError(format!("Failed to decode default literal: {}", e))
                })
            }
            Src::Global => {
                let key = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| {
                        AppClientError::ValidationError(format!(
                            "Failed to decode global key: {}",
                            e
                        ))
                    })?;
                let state = self
                    .algorand
                    .app()
                    .get_global_state(self.app_id.ok_or(AppClientError::ValidationError(
                        "Missing app_id".to_string(),
                    ))?)
                    .await
                    .map_err(|e| AppClientError::Network(e.to_string()))?;
                self.get_abi_decoded_value(
                    &key,
                    &state,
                    abi_type_str,
                    default.value_type.as_deref(),
                )
                .await
            }
            Src::Local => {
                let sender_addr = sender.ok_or_else(|| {
                    AppClientError::ValidationError(
                        "Sender is required to resolve local state default".to_string(),
                    )
                })?;
                let key = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| {
                        AppClientError::ValidationError(format!(
                            "Failed to decode local key: {}",
                            e
                        ))
                    })?;
                let state = self
                    .algorand
                    .app()
                    .get_local_state(
                        self.app_id.ok_or(AppClientError::ValidationError(
                            "Missing app_id".to_string(),
                        ))?,
                        sender_addr,
                    )
                    .await
                    .map_err(|e| AppClientError::Network(e.to_string()))?;
                self.get_abi_decoded_value(
                    &key,
                    &state,
                    abi_type_str,
                    default.value_type.as_deref(),
                )
                .await
            }
            Src::Box => {
                let box_key = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| {
                        AppClientError::ValidationError(format!("Failed to decode box key: {}", e))
                    })?;
                let raw = self
                    .algorand
                    .app()
                    .get_box_value(
                        self.app_id.ok_or(AppClientError::ValidationError(
                            "Missing app_id".to_string(),
                        ))?,
                        &box_key,
                    )
                    .await
                    .map_err(|e| AppClientError::Network(e.to_string()))?;
                let effective_type = default.value_type.as_deref().unwrap_or(abi_type_str);
                if effective_type == algokit_abi::arc56_contract::AVM_STRING {
                    return Ok(ABIValue::from(String::from_utf8_lossy(&raw).to_string()));
                }
                if effective_type == algokit_abi::arc56_contract::AVM_BYTES {
                    let arr = raw.into_iter().map(ABIValue::from_byte).collect();
                    return Ok(ABIValue::Array(arr));
                }
                let decode_type = ABIType::from_str(effective_type).map_err(|e| {
                    AppClientError::AbiError(format!(
                        "Invalid ABI type '{}': {}",
                        effective_type, e
                    ))
                })?;
                decode_type.decode(&raw).map_err(|e| {
                    AppClientError::AbiError(format!("Failed to decode box default: {}", e))
                })
            }
            Src::Method => Err(AppClientError::ValidationError(
                "Nested method default values are not supported".to_string(),
            )),
        }
    }
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

    /// Resolve a single ARC-56 default value entry to an ABIValue for a value-type arg
    pub async fn resolve_default_value_for_arg(
        &self,
        default: &algokit_abi::arc56_contract::DefaultValue,
        abi_type_str: &str,
        sender: Option<&str>,
    ) -> Result<ABIValue, AppClientError> {
        use algokit_abi::arc56_contract::DefaultValueSource as Src;
        match default.source {
            Src::Method => {
                let method_signature = default.data.clone();
                let arc56_method = self
                    .app_spec
                    .get_arc56_method(&method_signature)
                    .map_err(|e| AppClientError::MethodNotFound(e.to_string()))?;

                // Resolve all defaults for the method's value-type args
                let mut resolved_args: Vec<AppMethodCallArg> =
                    Vec::with_capacity(arc56_method.args.len());
                for arg in &arc56_method.args {
                    if let Some(def) = &arg.default_value {
                        let val = self
                            .resolve_default_value_for_arg_base(def, &arg.arg_type, sender)
                            .await?;
                        resolved_args.push(AppMethodCallArg::ABIValue(val));
                    } else {
                        return Err(AppClientError::ValidationError(format!(
                            "Method default for '{}' refers to method '{}' which has a required argument without a default",
                            abi_type_str, arc56_method.name
                        )));
                    }
                }

                // Build params via params layer and inject resolved args
                let method_call_params = super::types::AppClientMethodCallParams {
                    method: method_signature.clone(),
                    args: Some(resolved_args),
                    sender: sender.map(|s| s.to_string()),
                    ..Default::default()
                };
                let params = self
                    .params()
                    .method_call_no_defaults(&method_call_params)
                    .map_err(AppClientError::ValidationError)?;

                // Prefer simulate for readonly
                let is_readonly = arc56_method.readonly.unwrap_or(false);
                if is_readonly {
                    let mut composer = self.algorand().new_group();
                    composer
                        .add_app_call_method_call(params)
                        .map_err(|e| AppClientError::TransactionError(e.to_string()))?;
                    let sim = composer
                        .simulate(Some(crate::transactions::composer::SimulateParams {
                            allow_empty_signatures: Some(true),
                            skip_signatures: true,
                            ..Default::default()
                        }))
                        .await
                        .map_err(|e| AppClientError::TransactionError(e.to_string()))?;
                    let ret = sim.returns.last().cloned().ok_or_else(|| {
                        AppClientError::ValidationError(
                            "No ABI return found in simulate result".to_string(),
                        )
                    })?;
                    return Ok(ret.return_value);
                }

                let res = self
                    .algorand()
                    .send()
                    .app_call_method_call(params, None)
                    .await
                    .map_err(|e| AppClientError::TransactionError(e.to_string()))?;
                let ret = res.abi_return.ok_or_else(|| {
                    AppClientError::ValidationError(
                        "Default value method call did not return a value".to_string(),
                    )
                })?;
                Ok(ret.return_value)
            }
            _ => {
                // Non-method sources use shared base resolver
                self.resolve_default_value_for_arg_base(default, abi_type_str, sender)
                    .await
            }
        }
    }
    /// Resolve ARC-56 default arguments for a method. Provided args may be fewer than required.
    pub async fn resolve_default_arguments(
        &self,
        method_name_or_sig: &str,
        provided_args: &Option<Vec<ABIValue>>,
        sender: Option<&str>,
    ) -> Result<Vec<ABIValue>, AppClientError> {
        let method = self
            .app_spec
            .get_arc56_method(method_name_or_sig)
            .map_err(|e| AppClientError::MethodNotFound(e.to_string()))?;

        let mut resolved: Vec<ABIValue> = Vec::with_capacity(method.args.len());

        for (i, m_arg) in method.args.iter().enumerate() {
            if let Some(p) = provided_args.as_ref().and_then(|v| v.get(i)).cloned() {
                resolved.push(p);
                continue;
            }

            if let Some(default) = &m_arg.default_value {
                let value = self
                    .resolve_default_value_for_arg(default, &m_arg.arg_type, sender)
                    .await?;
                resolved.push(value);
            } else {
                return Err(AppClientError::ValidationError(format!(
                    "No value provided and no default for argument {} of method {}",
                    m_arg
                        .name
                        .clone()
                        .unwrap_or_else(|| format!("arg{}", i + 1)),
                    method.name
                )));
            }
        }

        Ok(resolved)
    }

    pub fn is_readonly_method(&self, method: &ABIMethod) -> bool {
        if let Ok(signature) = method.signature() {
            if let Ok(m) = self.app_spec.get_arc56_method(&signature) {
                if let Some(ro) = m.readonly {
                    return ro;
                }
            }
        }
        false
    }

    /// Simulate a read-only method call for cost-free execution.
    pub async fn simulate_readonly_call(
        &self,
        params: super::types::AppClientMethodCallParams,
    ) -> Result<algokit_abi::ABIReturn, AppClientError> {
        // Build full method params (resolve defaults) via params layer
        let method_params = self
            .params()
            .method_call(&params)
            .await
            .map_err(AppClientError::ValidationError)?;

        // If debug enabled, reuse shared debug simulate helper to emit traces
        if crate::config::Config::debug() {
            self.send()
                .simulate_readonly_with_tracing_for_debug(&params, false)
                .await
                .map_err(|e| AppClientError::TransactionError(e.to_string()))?;
        }

        // Always prefer simulate for readonly method calls
        let mut composer = self.algorand().new_group();
        composer
            .add_app_call_method_call(method_params)
            .map_err(|e| AppClientError::TransactionError(e.to_string()))?;
        let sim = composer
            .simulate(Some(crate::transactions::composer::SimulateParams {
                allow_empty_signatures: Some(true),
                skip_signatures: true,
                ..Default::default()
            }))
            .await
            .map_err(|e| AppClientError::TransactionError(e.to_string()))?;
        let ret = sim.returns.last().cloned().ok_or_else(|| {
            AppClientError::ValidationError("No ABI return found in simulate result".to_string())
        })?;
        Ok(ret)
    }
}
