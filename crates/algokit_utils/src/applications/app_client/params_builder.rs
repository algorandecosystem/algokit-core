use super::AppClient;
use super::types::{
    AppClientBareCallParams, AppClientMethodCallParams, CompilationParams, FundAppAccountParams,
};
use crate::AppClientError;
use crate::applications::app_client::utils::get_abi_decoded_value;
use crate::transactions::{
    AppCallMethodCallParams, AppCallParams, AppDeleteMethodCallParams, AppDeleteParams,
    AppMethodCallArg, AppUpdateMethodCallParams, AppUpdateParams, CommonTransactionParams,
    PaymentParams,
};
use algokit_abi::{
    ABIMethod, ABIMethodArgType, ABIType, ABIValue, DefaultValue, DefaultValueSource,
};
use algokit_transact::{Address, OnApplicationComplete};
use base64::Engine;
use std::str::FromStr;

pub struct ParamsBuilder<'a> {
    pub(crate) client: &'a AppClient,
}

pub struct BareParamsBuilder<'a> {
    pub(crate) client: &'a AppClient,
}

impl<'a> ParamsBuilder<'a> {
    /// Get the bare call params builder.
    pub fn bare(&self) -> BareParamsBuilder<'a> {
        BareParamsBuilder {
            client: self.client,
        }
    }

    /// Call a method with NoOp.
    pub async fn call(
        &self,
        params: AppClientMethodCallParams,
        on_complete: Option<OnApplicationComplete>,
    ) -> Result<AppCallMethodCallParams, AppClientError> {
        self.get_method_call_params(&params, on_complete.unwrap_or(OnApplicationComplete::NoOp))
            .await
    }

    /// Call a method with OptIn.
    pub async fn opt_in(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppCallMethodCallParams, AppClientError> {
        self.get_method_call_params(&params, OnApplicationComplete::OptIn)
            .await
    }

    /// Call a method with CloseOut.
    pub async fn close_out(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppCallMethodCallParams, AppClientError> {
        self.get_method_call_params(&params, OnApplicationComplete::CloseOut)
            .await
    }

    /// Call a method with ClearState.
    pub async fn clear_state(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppCallMethodCallParams, AppClientError> {
        self.get_method_call_params(&params, OnApplicationComplete::ClearState)
            .await
    }

    /// Call a method with Delete.
    pub async fn delete(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppDeleteMethodCallParams, AppClientError> {
        let method_params = self
            .get_method_call_params(&params, OnApplicationComplete::DeleteApplication)
            .await?;

        Ok(AppDeleteMethodCallParams {
            common_params: method_params.common_params,
            app_id: method_params.app_id,
            method: method_params.method,
            args: method_params.args,
            account_references: method_params.account_references,
            app_references: method_params.app_references,
            asset_references: method_params.asset_references,
            box_references: method_params.box_references,
        })
    }

    /// Update the application with a method call.
    pub async fn update(
        &self,
        params: AppClientMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<AppUpdateMethodCallParams, AppClientError> {
        // Compile programs (and populate AppManager cache/source maps)
        let compilation_params = compilation_params.unwrap_or_default();
        let (approval_program, clear_state_program) =
            self.client.compile_with_params(&compilation_params).await?;

        // Reuse method_call to resolve method + args + common params
        let method_params = self
            .get_method_call_params(&params, OnApplicationComplete::UpdateApplication)
            .await?;

        Ok(AppUpdateMethodCallParams {
            common_params: method_params.common_params,
            app_id: method_params.app_id,
            approval_program,
            clear_state_program,
            method: method_params.method,
            args: method_params.args,
            account_references: method_params.account_references,
            app_references: method_params.app_references,
            asset_references: method_params.asset_references,
            box_references: method_params.box_references,
        })
    }

    /// Fund the application account.
    pub fn fund_app_account(
        &self,
        params: &FundAppAccountParams,
    ) -> Result<PaymentParams, AppClientError> {
        let sender = self.client.get_sender_address(&params.sender)?;
        let receiver = self.client.app_address();
        let rekey_to = get_optional_address(&params.rekey_to)?;

        Ok(PaymentParams {
            common_params: CommonTransactionParams {
                sender,
                rekey_to,
                note: params.note.clone(),
                lease: params.lease,
                static_fee: params.static_fee,
                extra_fee: params.extra_fee,
                max_fee: params.max_fee,
                validity_window: params.validity_window,
                first_valid_round: params.first_valid_round,
                last_valid_round: params.last_valid_round,
                ..Default::default()
            },
            receiver,
            amount: params.amount,
        })
    }

    async fn get_method_call_params(
        &self,
        params: &AppClientMethodCallParams,
        on_complete: OnApplicationComplete,
    ) -> Result<AppCallMethodCallParams, AppClientError> {
        let abi_method = self.get_abi_method(&params.method)?;
        let sender = self.client.get_sender_address(&params.sender)?.as_str();

        let resolved_args = self
            .resolve_args_with_defaults(&abi_method, &params.args, &sender)
            .await?;

        Ok(AppCallMethodCallParams {
            common_params: self.build_common_params_from_method(params)?,
            app_id: self.client.app_id,
            method: abi_method,
            args: resolved_args,
            account_references: super::utils::parse_account_refs_strs(&params.account_references)?,
            app_references: params.app_references.clone(),
            asset_references: params.asset_references.clone(),
            box_references: params.box_references.clone(),
            on_complete: on_complete,
        })
    }

    fn build_common_params_from_method(
        &self,
        params: &AppClientMethodCallParams,
    ) -> Result<CommonTransactionParams, AppClientError> {
        Ok(CommonTransactionParams {
            sender: self.client.get_sender_address(&params.sender)?,
            rekey_to: get_optional_address(&params.rekey_to)?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            ..Default::default()
        })
    }

    fn get_abi_method(&self, method_name_or_signature: &str) -> Result<ABIMethod, AppClientError> {
        let m = self
            .client
            .app_spec
            .get_arc56_method(method_name_or_signature)
            .map_err(|e| AppClientError::ABIError { source: e })?;
        self.client
            .app_spec
            .to_abi_method(m)
            .map_err(|e| AppClientError::ABIError { source: e })
    }

    // TODO: rethink the positioning of this method
    async fn resolve_args_with_defaults(
        &self,
        method: &ABIMethod,
        provided: &Vec<AppMethodCallArg>,
        sender: &str,
    ) -> Result<Vec<AppMethodCallArg>, AppClientError> {
        let mut resolved: Vec<AppMethodCallArg> = Vec::with_capacity(method.args.len());

        // Pre-fetch ARC-56 method once if available
        let arc56_method = method
            .signature()
            .and_then(|sig| self.client.app_spec().get_arc56_method(&sig))
            .map_err(|e| AppClientError::ABIError { source: e })?;

        if method.args.len() != provided.len() {
            return Err(AppClientError::ValidationError {
                message: format!(
                    "The number of provided arguments is {} while the method expects {} arguments",
                    provided.len(),
                    method.args.len()
                ),
            });
        }

        for (index, (method_arg, provided_arg)) in method.args.iter().zip(provided).enumerate() {
            let method_arg_name = method_arg
                .name
                .clone()
                .unwrap_or_else(|| format!("arg{}", index + 1));
            match (&method_arg.arg_type, provided_arg) {
                // Value-type arguments
                (ABIMethodArgType::Value(value_type), AppMethodCallArg::DefaultValue) => {
                    let default_value = arc56_method
                        .args
                        .get(index)
                        .and_then(|a| a.default_value.clone())
                        .ok_or_else(|| AppClientError::ParamsBuilderError {
                            message: format!(
                                "No default value defined for argument {} in call to method {}",
                                method_arg_name, method.name
                            ),
                        })?;
                    let value = self
                        .resolve_default_value_for_arg(&default_value, &value_type, sender)
                        .await?;
                    resolved.push(AppMethodCallArg::ABIValue(value));
                }
                (_, AppMethodCallArg::DefaultValue) => {
                    return Err(AppClientError::ParamsBuilderError {
                        message: format!(
                            "Default value is not supported by argument {} in call to method {}",
                            method_arg_name, method.name
                        ),
                    });
                }
                // TODO: can we ignore other validations, they will be handled at encoding?
                (_, value) => {
                    resolved.push(value.clone());
                }
            }
        }

        Ok(resolved)
    }

    /// Resolve a single ARC-56 default value entry to an ABIValue for a value-type arg
    pub async fn resolve_default_value_for_arg(
        &self,
        default: &DefaultValue,
        abi_type: &ABIType,
        sender: &str,
    ) -> Result<ABIValue, AppClientError> {
        match default.source {
            DefaultValueSource::Method => {
                let method_signature = default.data.clone();
                let arc56_method = self
                    .client
                    .app_spec
                    .get_arc56_method(&method_signature)
                    .map_err(|e| AppClientError::ABIError { source: e })?;

                // Build params via params layer
                let method_call_params = AppClientMethodCallParams {
                    method: method_signature.clone(),
                    args: vec![AppMethodCallArg::DefaultValue; arc56_method.args.len()],
                    sender: Some(sender.to_string()),
                    ..Default::default()
                };

                let app_call_result =
                    Box::pin(self.client.send().call(method_call_params, None)).await?;
                let abi_return = app_call_result.abi_return.ok_or_else(|| {
                    AppClientError::ParamsBuilderError {
                        message: "Default value method call did not return a value".to_string(),
                    }
                })?;
                Ok(abi_return.return_value)
            }
            // TODO: I haven't checked this logic
            DefaultValueSource::Literal => {
                let raw = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| AppClientError::ParamsBuilderError {
                        message: format!("Failed to decode base64 literal: {}", e),
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
                let decode_type = if let Some(ref value_type) = default.value_type {
                    ABIType::from_str(value_type)
                        .map_err(|e| AppClientError::ABIError { source: e })?
                } else {
                    abi_type.clone()
                };
                decode_type
                    .decode(&raw)
                    .map_err(|e| AppClientError::ParamsBuilderError {
                        message: format!("Failed to decode base64 literal: {}", e),
                    })
            }
            DefaultValueSource::Global => {
                let key = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| AppClientError::ParamsBuilderError {
                        message: format!("Failed to decode global key: {}", e),
                    })?;

                let state = self
                    .client
                    .algorand
                    .app()
                    .get_global_state(self.client.app_id)
                    .await
                    .map_err(|e| AppClientError::AppManagerError { source: e })?;

                get_abi_decoded_value(
                    &key,
                    &state,
                    &abi_type.to_string(), // TODO: fix this
                    default.value_type.as_deref(),
                )
                .await
            }
            DefaultValueSource::Local => {
                let key = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| AppClientError::ParamsBuilderError {
                        message: format!("Failed to decode local key: {}", e),
                    })?;
                let state = self
                    .client
                    .algorand
                    .app()
                    .get_local_state(self.client.app_id, sender)
                    .await
                    .map_err(|e| AppClientError::AppManagerError { source: e })?;
                get_abi_decoded_value(
                    &key,
                    &state,
                    &abi_type.to_string(), // TODO: fix this
                    default.value_type.as_deref(),
                )
                .await
            }
            DefaultValueSource::Box => {
                let box_key = base64::engine::general_purpose::STANDARD
                    .decode(&default.data)
                    .map_err(|e| AppClientError::ParamsBuilderError {
                        message: format!("Failed to decode box key: {}", e),
                    })?;
                let raw = self
                    .client
                    .algorand
                    .app()
                    .get_box_value(self.client.app_id, &box_key)
                    .await
                    .map_err(|e| AppClientError::AppManagerError { source: e })?;
                let foo = &abi_type.to_string(); // TODO: fix this
                let effective_type = default.value_type.as_deref().unwrap_or(foo);
                if effective_type == algokit_abi::arc56_contract::AVM_STRING {
                    return Ok(ABIValue::from(String::from_utf8_lossy(&raw).to_string()));
                }
                if effective_type == algokit_abi::arc56_contract::AVM_BYTES {
                    let arr = raw.into_iter().map(ABIValue::from_byte).collect();
                    return Ok(ABIValue::Array(arr));
                }
                let decode_type = ABIType::from_str(effective_type)
                    .map_err(|e| AppClientError::ABIError { source: e })?;
                decode_type
                    .decode(&raw)
                    .map_err(|e| AppClientError::ABIError { source: e })
            }
        }
    }
}

impl BareParamsBuilder<'_> {
    /// Call with NoOp.
    pub fn call(
        &self,
        params: AppClientBareCallParams,
        on_complete: Option<OnApplicationComplete>,
    ) -> Result<AppCallParams, AppClientError> {
        self.build_bare_app_call_params(params, on_complete.unwrap_or(OnApplicationComplete::NoOp))
    }

    /// Call with OptIn.
    pub fn opt_in(&self, params: AppClientBareCallParams) -> Result<AppCallParams, AppClientError> {
        self.build_bare_app_call_params(params, OnApplicationComplete::OptIn)
    }

    /// Call with CloseOut.
    pub fn close_out(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<AppCallParams, AppClientError> {
        self.build_bare_app_call_params(params, OnApplicationComplete::CloseOut)
    }

    /// Call with Delete.
    pub fn delete(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<AppDeleteParams, AppClientError> {
        let app_call =
            self.build_bare_app_call_params(params, OnApplicationComplete::DeleteApplication)?;
        Ok(AppDeleteParams {
            common_params: app_call.common_params,
            app_id: app_call.app_id,
            args: app_call.args,
            account_references: app_call.account_references,
            app_references: app_call.app_references,
            asset_references: app_call.asset_references,
            box_references: app_call.box_references,
        })
    }

    /// Call with ClearState.
    pub fn clear_state(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<AppCallParams, AppClientError> {
        self.build_bare_app_call_params(params, OnApplicationComplete::ClearState)
    }

    /// Update with bare call.
    pub async fn update(
        &self,
        params: AppClientBareCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<AppUpdateParams, AppClientError> {
        // Compile programs (and populate AppManager cache/source maps)
        let compilation_params = compilation_params.unwrap_or_default();
        let (approval_program, clear_state_program) =
            self.client.compile_with_params(&compilation_params).await?;

        // Resolve common/bare fields
        let app_call =
            self.build_bare_app_call_params(params, OnApplicationComplete::UpdateApplication)?;

        Ok(AppUpdateParams {
            common_params: app_call.common_params,
            app_id: app_call.app_id,
            approval_program,
            clear_state_program,
            args: app_call.args,
            account_references: app_call.account_references,
            app_references: app_call.app_references,
            asset_references: app_call.asset_references,
            box_references: app_call.box_references,
        })
    }

    fn build_bare_app_call_params(
        &self,
        params: AppClientBareCallParams,
        on_complete: OnApplicationComplete,
    ) -> Result<AppCallParams, AppClientError> {
        Ok(AppCallParams {
            common_params: self.build_common_params_from_bare(&params)?,
            app_id: self.client.app_id,
            on_complete: on_complete,
            args: params.args,
            account_references: super::utils::parse_account_refs_strs(&params.account_references)?,
            app_references: params.app_references,
            asset_references: params.asset_references,
            box_references: params.box_references,
        })
    }

    fn build_common_params_from_bare(
        &self,
        params: &AppClientBareCallParams,
    ) -> Result<CommonTransactionParams, AppClientError> {
        Ok(CommonTransactionParams {
            sender: self.client.get_sender_address(&params.sender)?,
            rekey_to: get_optional_address(&params.rekey_to)?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
            ..Default::default()
        })
    }
}

fn get_optional_address(value: &Option<String>) -> Result<Option<Address>, AppClientError> {
    match value {
        Some(s) => {
            Ok(Some(Address::from_str(s).map_err(|e| {
                AppClientError::TransactError { source: e }
            })?))
        }
        None => Ok(None),
    }
}
