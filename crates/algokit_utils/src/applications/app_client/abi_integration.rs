use algokit_abi::ABIMethod;
use algokit_abi::{ABIType, ABIValue};
use base64::Engine;
use std::str::FromStr;

use super::AppClient;
use crate::transactions::{AppCallMethodCallParams, AppMethodCallArg, CommonTransactionParams};

impl AppClient {
    fn build_method_call_params_no_defaults(
        &self,
        method_sig: &str,
        sender: Option<&str>,
    ) -> Result<AppCallMethodCallParams, String> {
        let abi_method = self
            .app_spec
            .get_arc56_method(method_sig)
            .map_err(|e| e.to_string())?
            .to_abi_method()
            .map_err(|e| e.to_string())?;
        let common_params = CommonTransactionParams {
            sender: self.get_sender_address(&sender.map(|s| s.to_string()))?,
            signer: None,
            rekey_to: None,
            note: None,
            lease: None,
            static_fee: None,
            extra_fee: None,
            max_fee: None,
            validity_window: None,
            first_valid_round: None,
            last_valid_round: None,
        };
        Ok(AppCallMethodCallParams {
            common_params,
            app_id: self.app_id.ok_or_else(|| "Missing app_id".to_string())?,
            method: abi_method,
            args: Vec::<AppMethodCallArg>::new(),
            account_references: None,
            app_references: None,
            asset_references: None,
            box_references: None,
            on_complete: algokit_transact::OnApplicationComplete::NoOp,
        })
    }
    /// Resolve a single ARC-56 default value entry to an ABIValue for a value-type arg
    pub async fn resolve_default_value_for_arg(
        &self,
        default: &algokit_abi::arc56_contract::DefaultValue,
        abi_type_str: &str,
        sender: Option<&str>,
    ) -> Result<ABIValue, String> {
        use algokit_abi::arc56_contract::DefaultValueSource as Src;
        let abi_type = ABIType::from_str(abi_type_str)
            .map_err(|e| format!("Invalid ABI type '{}': {}", abi_type_str, e))?;
        match default.source {
            Src::Literal => {
                let raw = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| format!("Failed to decode base64 literal: {}", e))?;
                let decode_type = if let Some(ref vt) = default.value_type {
                    ABIType::from_str(vt)
                        .map_err(|e| format!("Invalid default value ABI type '{}': {}", vt, e))?
                } else {
                    abi_type.clone()
                };
                decode_type
                    .decode(&raw)
                    .map_err(|e| format!("Failed to decode default literal: {}", e))
            }
            Src::Global => {
                let key = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| format!("Failed to decode global key: {}", e))?;
                let state = self
                    .algorand
                    .app()
                    .get_global_state(self.app_id.ok_or("Missing app_id")?)
                    .await
                    .map_err(|e| e.to_string())?;
                let app_state = state.get(&key).ok_or_else(|| {
                    format!("Global state key not found for default: {}", default.data)
                })?;
                if let Some(vt) = &default.value_type {
                    match vt.as_str() {
                        algokit_abi::arc56_contract::AVM_STRING => {
                            let bytes = app_state
                                .value_raw
                                .clone()
                                .ok_or_else(|| "Global state has no raw value".to_string())?;
                            let s = String::from_utf8_lossy(&bytes).to_string();
                            if let Ok(decoded) =
                                base64::engine::general_purpose::STANDARD.decode(&s)
                            {
                                if let Ok(decoded_str) = String::from_utf8(decoded) {
                                    return Ok(ABIValue::from(decoded_str));
                                }
                            }
                            return Ok(ABIValue::from(s));
                        }
                        algokit_abi::arc56_contract::AVM_BYTES => {
                            let bytes = app_state
                                .value_raw
                                .clone()
                                .ok_or_else(|| "Global state has no raw value".to_string())?;
                            let arr = bytes.into_iter().map(ABIValue::from_byte).collect();
                            return Ok(ABIValue::Array(arr));
                        }
                        _ => {}
                    }
                }
                let raw = app_state
                    .value_raw
                    .clone()
                    .ok_or_else(|| "Global state has no raw value".to_string())?;
                abi_type
                    .decode(&raw)
                    .map_err(|e| format!("Failed to decode global default: {}", e))
            }
            Src::Local => {
                let sender_addr = sender.ok_or_else(|| {
                    "Sender is required to resolve local state default".to_string()
                })?;
                let key = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| format!("Failed to decode local key: {}", e))?;
                let state = self
                    .algorand
                    .app()
                    .get_local_state(self.app_id.ok_or("Missing app_id")?, sender_addr)
                    .await
                    .map_err(|e| e.to_string())?;
                let app_state = state.get(&key).ok_or_else(|| {
                    format!("Local state key not found for default: {}", default.data)
                })?;
                if let Some(vt) = &default.value_type {
                    match vt.as_str() {
                        algokit_abi::arc56_contract::AVM_STRING => {
                            let bytes = app_state
                                .value_raw
                                .clone()
                                .ok_or_else(|| "Local state has no raw value".to_string())?;
                            let s = String::from_utf8_lossy(&bytes).to_string();
                            if let Ok(decoded) =
                                base64::engine::general_purpose::STANDARD.decode(&s)
                            {
                                if let Ok(decoded_str) = String::from_utf8(decoded) {
                                    return Ok(ABIValue::from(decoded_str));
                                }
                            }
                            return Ok(ABIValue::from(s));
                        }
                        algokit_abi::arc56_contract::AVM_BYTES => {
                            let bytes = app_state
                                .value_raw
                                .clone()
                                .ok_or_else(|| "Local state has no raw value".to_string())?;
                            let arr = bytes.into_iter().map(ABIValue::from_byte).collect();
                            return Ok(ABIValue::Array(arr));
                        }
                        _ => {}
                    }
                }
                let raw = app_state
                    .value_raw
                    .clone()
                    .ok_or_else(|| "Local state has no raw value".to_string())?;
                abi_type
                    .decode(&raw)
                    .map_err(|e| format!("Failed to decode local default: {}", e))
            }
            Src::Box => {
                let box_key = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| format!("Failed to decode box key: {}", e))?;
                let raw = self
                    .algorand
                    .app()
                    .get_box_value(self.app_id.ok_or("Missing app_id")?, &box_key)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(vt) = &default.value_type {
                    match vt.as_str() {
                        algokit_abi::arc56_contract::AVM_STRING => {
                            let s = String::from_utf8_lossy(&raw).to_string();
                            return Ok(ABIValue::from(s));
                        }
                        algokit_abi::arc56_contract::AVM_BYTES => {
                            let arr = raw.into_iter().map(ABIValue::from_byte).collect();
                            return Ok(ABIValue::Array(arr));
                        }
                        _ => {}
                    }
                }
                abi_type
                    .decode(&raw)
                    .map_err(|e| format!("Failed to decode box default: {}", e))
            }
            Src::Method => {
                let default_sig = default.data.clone();
                let params = super::types::AppClientMethodCallParams {
                    method: default_sig,
                    args: Some(Vec::new()),
                    sender: sender.map(|s| s.to_string()),
                    rekey_to: None,
                    note: None,
                    lease: None,
                    static_fee: None,
                    extra_fee: None,
                    max_fee: None,
                    validity_window: None,
                    first_valid_round: None,
                    last_valid_round: None,
                    account_references: None,
                    app_references: None,
                    asset_references: None,
                    box_references: None,
                    on_complete: Some(algokit_transact::OnApplicationComplete::NoOp),
                };
                let res = self
                    .algorand()
                    .send()
                    .app_call_method_call(
                        self.build_method_call_params_no_defaults(&params.method, sender)?,
                        None,
                    )
                    .await
                    .map_err(|e| e.to_string())?;
                let ret = res.abi_return.ok_or_else(|| {
                    "Default value method call did not return a value".to_string()
                })?;
                Ok(ret.return_value)
            }
        }
    }
    /// Resolve ARC-56 default arguments for a method. Provided args may be fewer than required.
    pub async fn resolve_default_arguments(
        &self,
        method_name_or_sig: &str,
        provided_args: &Option<Vec<ABIValue>>,
        sender: Option<&str>,
    ) -> Result<Vec<ABIValue>, String> {
        let method = self
            .app_spec
            .get_arc56_method(method_name_or_sig)
            .map_err(|e| e.to_string())?;

        let mut resolved: Vec<ABIValue> = Vec::with_capacity(method.args.len());

        for (i, m_arg) in method.args.iter().enumerate() {
            if let Some(p) = provided_args.as_ref().and_then(|v| v.get(i)).cloned() {
                resolved.push(p);
                continue;
            }

            let abi_type = ABIType::from_str(&m_arg.arg_type)
                .map_err(|e| format!("Invalid ABI type '{}': {}", m_arg.arg_type, e))?;

            if let Some(default) = &m_arg.default_value {
                use algokit_abi::arc56_contract::DefaultValueSource as Src;
                match default.source {
                    Src::Literal => {
                        let raw = base64::engine::general_purpose::STANDARD
                            .decode(&default.data)
                            .map_err(|e| format!("Failed to decode base64 literal: {}", e))?;
                        let decode_type = if let Some(ref vt) = default.value_type {
                            ABIType::from_str(vt).map_err(|e| {
                                format!("Invalid default value ABI type '{}': {}", vt, e)
                            })?
                        } else {
                            abi_type.clone()
                        };
                        let value = decode_type
                            .decode(&raw)
                            .map_err(|e| format!("Failed to decode default literal: {}", e))?;
                        resolved.push(value);
                    }
                    Src::Global => {
                        let key = base64::engine::general_purpose::STANDARD
                            .decode(&default.data)
                            .map_err(|e| format!("Failed to decode global key: {}", e))?;
                        let state = self
                            .algorand
                            .app()
                            .get_global_state(self.app_id.ok_or("Missing app_id")?)
                            .await
                            .map_err(|e| e.to_string())?;
                        let app_state = state.get(&key).ok_or_else(|| {
                            format!("Global state key not found for default: {}", default.data)
                        })?;
                        let raw = app_state
                            .value_raw
                            .clone()
                            .ok_or_else(|| "Global state has no raw value".to_string())?;
                        let value = abi_type
                            .decode(&raw)
                            .map_err(|e| format!("Failed to decode global default: {}", e))?;
                        resolved.push(value);
                    }
                    Src::Local => {
                        let sender_addr = sender.ok_or_else(|| {
                            "Sender is required to resolve local state default".to_string()
                        })?;
                        let key = base64::engine::general_purpose::STANDARD
                            .decode(&default.data)
                            .map_err(|e| format!("Failed to decode local key: {}", e))?;
                        let state = self
                            .algorand
                            .app()
                            .get_local_state(self.app_id.ok_or("Missing app_id")?, sender_addr)
                            .await
                            .map_err(|e| e.to_string())?;
                        let app_state = state.get(&key).ok_or_else(|| {
                            format!("Local state key not found for default: {}", default.data)
                        })?;
                        let raw = app_state
                            .value_raw
                            .clone()
                            .ok_or_else(|| "Local state has no raw value".to_string())?;
                        let value = abi_type
                            .decode(&raw)
                            .map_err(|e| format!("Failed to decode local default: {}", e))?;
                        resolved.push(value);
                    }
                    Src::Box => {
                        let box_key = base64::engine::general_purpose::STANDARD
                            .decode(&default.data)
                            .map_err(|e| format!("Failed to decode box key: {}", e))?;
                        let raw = self
                            .algorand
                            .app()
                            .get_box_value(self.app_id.ok_or("Missing app_id")?, &box_key)
                            .await
                            .map_err(|e| e.to_string())?;
                        let value = abi_type
                            .decode(&raw)
                            .map_err(|e| format!("Failed to decode box default: {}", e))?;
                        resolved.push(value);
                    }
                    Src::Method => {
                        // Call the default method with no arguments; extract ABI return value
                        let default_sig = default.data.clone();
                        let call_params = super::types::AppClientMethodCallParams {
                            method: default_sig,
                            args: Some(Vec::new()),
                            sender: sender.map(|s| s.to_string()),
                            rekey_to: None,
                            note: None,
                            lease: None,
                            static_fee: None,
                            extra_fee: None,
                            max_fee: None,
                            validity_window: None,
                            first_valid_round: None,
                            last_valid_round: None,
                            account_references: None,
                            app_references: None,
                            asset_references: None,
                            box_references: None,
                            on_complete: Some(algokit_transact::OnApplicationComplete::NoOp),
                        };
                        let res = self
                            .algorand()
                            .send()
                            .app_call_method_call(
                                self.build_method_call_params_no_defaults(
                                    &call_params.method,
                                    sender,
                                )?,
                                None,
                            )
                            .await
                            .map_err(|e| e.to_string())?;
                        let ret = res.abi_return.ok_or_else(|| {
                            "Default value method call did not return a value".to_string()
                        })?;
                        resolved.push(ret.return_value);
                    }
                }
            } else {
                return Err(format!(
                    "No value provided and no default for argument {} of method {}",
                    m_arg
                        .name
                        .clone()
                        .unwrap_or_else(|| format!("arg{}", i + 1)),
                    method.name
                ));
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
    ) -> Result<algokit_abi::ABIReturn, String> {
        // Prefer simulate when debug is enabled to gather traces; otherwise use send path
        let method_params =
            self.build_method_call_params_no_defaults(&params.method, params.sender.as_deref())?;

        if crate::config::Config::debug() {
            // Build transactions and simulate via TransactionCreator
            let _built = self
                .algorand()
                .create()
                .app_call_method_call(method_params.clone())
                .await
                .map_err(|e| e.to_string())?;

            // Use composer directly to simulate with extra logging when debug
            let mut composer = self.algorand().new_group();
            composer
                .add_app_call_method_call(method_params)
                .map_err(|e| e.to_string())?;

            let params = crate::transactions::composer::SimulateParams {
                allow_more_logging: Some(true),
                allow_empty_signatures: None,
                allow_unnamed_resources: None,
                extra_opcode_budget: None,
                exec_trace_config: Some(algod_client::models::SimulateTraceConfig {
                    enable: Some(true),
                    scratch_change: Some(true),
                    stack_change: Some(true),
                    state_change: Some(true),
                }),
                simulation_round: None,
                skip_signatures: false,
            };

            let sim = composer
                .simulate(Some(params))
                .await
                .map_err(|e| e.to_string())?;

            if crate::config::Config::trace_all() {
                // Emit simulate response as JSON for listeners
                let json = serde_json::to_value(&sim.confirmations)
                    .unwrap_or(serde_json::json!({"error":"failed to serialize confirmations"}));
                let event = crate::config::TxnGroupSimulatedEventData {
                    simulate_response: json,
                };
                crate::config::Config::events()
                    .emit(
                        crate::config::EventType::TxnGroupSimulated,
                        crate::config::EventData::TxnGroupSimulated(event),
                    )
                    .await;
            }

            // Extract last ABI return if available from returns collected during simulate
            let ret = sim
                .returns
                .last()
                .cloned()
                .ok_or_else(|| "No ABI return found in simulate result".to_string())?;
            return Ok(ret);
        }

        let send_res = self
            .algorand
            .send()
            .app_call_method_call(method_params, None)
            .await
            .map_err(|e| e.to_string())?;

        match &send_res.abi_return {
            Some(ret) => Ok(ret.clone()),
            None => Err("No ABI return found in result".to_string()),
        }
    }
}
