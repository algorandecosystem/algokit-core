use algokit_abi::ABIMethod;
use algokit_transact::OnApplicationComplete;

use crate::transactions::{
    AppCallMethodCallParams, AppCallParams, AppDeleteMethodCallParams, AppDeleteParams,
    AppMethodCallArg, AppUpdateMethodCallParams, AppUpdateParams, CommonTransactionParams,
    PaymentParams,
};

use super::AppClient;
use super::types::{
    AppClientBareCallParams, AppClientMethodCallParams, CompilationParams, FundAppAccountParams,
};

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
    ) -> Result<AppCallMethodCallParams, String> {
        self.method_call_with_on_complete(params, OnApplicationComplete::NoOp)
            .await
    }

    /// Call a method with OptIn.
    pub async fn opt_in(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppCallMethodCallParams, String> {
        self.method_call_with_on_complete(params, OnApplicationComplete::OptIn)
            .await
    }

    /// Call a method with CloseOut.
    pub async fn close_out(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppCallMethodCallParams, String> {
        self.method_call_with_on_complete(params, OnApplicationComplete::CloseOut)
            .await
    }

    /// Call a method with Delete.
    pub async fn delete(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<AppDeleteMethodCallParams, String> {
        let method_params = self
            .method_call_with_on_complete(params, OnApplicationComplete::DeleteApplication)
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
    ) -> Result<AppUpdateMethodCallParams, String> {
        // Compile programs (and populate AppManager cache/source maps)
        let cp = compilation_params.unwrap_or_default();
        let (approval_program, clear_state_program) = self
            .client
            .compile_with_params(&cp)
            .await
            .map_err(|e| e.to_string())?;

        // Reuse method_call to resolve method + args + common params
        let method_params = self.method_call(&params).await?;

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
    pub fn fund_app_account(&self, params: &FundAppAccountParams) -> Result<PaymentParams, String> {
        let sender = self.client.get_sender_address(&params.sender)?;
        let receiver = self.client.get_app_address()?;
        let rekey_to = AppClient::get_optional_address(&params.rekey_to)?;

        Ok(PaymentParams {
            common_params: CommonTransactionParams {
                sender,
                signer: None,
                rekey_to,
                note: params.note.clone(),
                lease: params.lease,
                static_fee: params.static_fee,
                extra_fee: params.extra_fee,
                max_fee: params.max_fee,
                validity_window: params.validity_window,
                first_valid_round: params.first_valid_round,
                last_valid_round: params.last_valid_round,
            },
            receiver,
            amount: params.amount,
        })
    }

    async fn method_call_with_on_complete(
        &self,
        mut params: AppClientMethodCallParams,
        on_complete: OnApplicationComplete,
    ) -> Result<AppCallMethodCallParams, String> {
        params.on_complete = Some(on_complete);
        self.method_call(&params).await
    }

    pub async fn method_call(
        &self,
        params: &AppClientMethodCallParams,
    ) -> Result<AppCallMethodCallParams, String> {
        let abimethod = self.to_abimethod(&params.method)?;
        let provided_len = params.args.as_ref().map(|v| v.len()).unwrap_or(0);
        let expected = abimethod.args.len();
        if provided_len > expected {
            return Err(format!(
                "Unexpected arg at position {}. {} only expects {} args",
                expected + 1,
                abimethod.name,
                expected
            ));
        }

        let resolved_args = self
            .resolve_args_with_defaults(&abimethod, &params.args, params.sender.as_deref())
            .await?;

        Ok(AppCallMethodCallParams {
            common_params: self.build_common_params_from_method(params)?,
            app_id: self
                .client
                .app_id
                .ok_or_else(|| "Missing app_id".to_string())?,
            method: abimethod,
            args: resolved_args,
            account_references: super::utils::parse_account_refs_strs(&params.account_references)?,
            app_references: params.app_references.clone(),
            asset_references: params.asset_references.clone(),
            box_references: params.box_references.clone(),
            on_complete: params.on_complete.unwrap_or(OnApplicationComplete::NoOp),
        })
    }

    fn build_common_params_from_method(
        &self,
        params: &AppClientMethodCallParams,
    ) -> Result<CommonTransactionParams, String> {
        Ok(CommonTransactionParams {
            sender: self.client.get_sender_address(&params.sender)?,
            signer: None,
            rekey_to: AppClient::get_optional_address(&params.rekey_to)?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
        })
    }

    fn to_abimethod(&self, method_name_or_sig: &str) -> Result<ABIMethod, String> {
        let m = self
            .client
            .app_spec
            .get_arc56_method(method_name_or_sig)
            .map_err(|e| e.to_string())?;
        m.to_abi_method().map_err(|e| e.to_string())
    }

    async fn resolve_args_with_defaults(
        &self,
        method: &ABIMethod,
        provided: &Option<Vec<AppMethodCallArg>>,
        sender: Option<&str>,
    ) -> Result<Vec<AppMethodCallArg>, String> {
        use algokit_abi::ABIMethodArgType;
        let mut resolved: Vec<AppMethodCallArg> = Vec::with_capacity(method.args.len());

        // Pre-fetch ARC-56 method once if available
        let arc56_method = method
            .signature()
            .ok()
            .and_then(|sig| self.client.app_spec().get_arc56_method(&sig).ok());

        for (i, m_arg) in method.args.iter().enumerate() {
            let provided_arg = provided.as_ref().and_then(|v| v.get(i)).cloned();

            match (&m_arg.arg_type, provided_arg) {
                // Value-type arguments
                (ABIMethodArgType::Value(value_type), Some(AppMethodCallArg::ABIValue(v))) => {
                    // Provided concrete ABI value
                    // (we don't type-check here; encoder will validate)
                    let _ = value_type; // silence unused variable warning if any
                    resolved.push(AppMethodCallArg::ABIValue(v));
                }
                (ABIMethodArgType::Value(value_type), Some(AppMethodCallArg::DefaultValue)) => {
                    // Explicit request to use ARC-56 default
                    let def = arc56_method
                        .as_ref()
                        .and_then(|m| m.args.get(i))
                        .and_then(|a| a.default_value.clone())
                        .ok_or_else(|| {
                            format!(
                                "No default value defined for argument {} in call to method {}",
                                m_arg
                                    .name
                                    .clone()
                                    .unwrap_or_else(|| format!("arg{}", i + 1)),
                                method.name
                            )
                        })?;
                    let abi_type_string = value_type.to_string();
                    let value = self
                        .client
                        .resolve_default_value_for_arg(&def, &abi_type_string, sender)
                        .await?;
                    resolved.push(AppMethodCallArg::ABIValue(value));
                }
                (ABIMethodArgType::Value(_), Some(other)) => {
                    return Err(format!(
                        "Invalid argument type for value argument {} in call to method {}: {:?}",
                        m_arg
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("arg{}", i + 1)),
                        method.name,
                        other
                    ));
                }
                (ABIMethodArgType::Value(value_type), None) => {
                    // No provided value; try default, else error
                    if let Some(def) = arc56_method
                        .as_ref()
                        .and_then(|m| m.args.get(i))
                        .and_then(|a| a.default_value.clone())
                    {
                        let abi_type_string = value_type.to_string();
                        let value = self
                            .client
                            .resolve_default_value_for_arg(&def, &abi_type_string, sender)
                            .await?;
                        resolved.push(AppMethodCallArg::ABIValue(value));
                    } else {
                        return Err(format!(
                            "No value provided for required argument {} in call to method {}",
                            m_arg
                                .name
                                .clone()
                                .unwrap_or_else(|| format!("arg{}", i + 1)),
                            method.name
                        ));
                    }
                }

                // Reference-type arguments must be provided explicitly as ABIReference
                (ABIMethodArgType::Reference(_), Some(AppMethodCallArg::ABIReference(r))) => {
                    resolved.push(AppMethodCallArg::ABIReference(r));
                }
                (ABIMethodArgType::Reference(_), Some(AppMethodCallArg::DefaultValue)) => {
                    return Err(format!(
                        "DefaultValue sentinel not supported for reference argument {} in call to method {}",
                        m_arg
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("arg{}", i + 1)),
                        method.name
                    ));
                }
                (ABIMethodArgType::Reference(_), Some(other)) => {
                    return Err(format!(
                        "Invalid argument type for reference argument {} in call to method {}: {:?}",
                        m_arg
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("arg{}", i + 1)),
                        method.name,
                        other
                    ));
                }
                (ABIMethodArgType::Reference(_), None) => {
                    return Err(format!(
                        "No value provided for required reference argument {} in call to method {}",
                        m_arg
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("arg{}", i + 1)),
                        method.name
                    ));
                }

                // Transaction-type arguments: allow omission or DefaultValue -> placeholder
                (ABIMethodArgType::Transaction(_), Some(AppMethodCallArg::DefaultValue)) => {
                    resolved.push(AppMethodCallArg::TransactionPlaceholder);
                }
                (ABIMethodArgType::Transaction(_), Some(arg)) => {
                    // Any transaction-bearing variant or explicit placeholder is accepted
                    resolved.push(arg);
                }
                (ABIMethodArgType::Transaction(_), None) => {
                    resolved.push(AppMethodCallArg::TransactionPlaceholder);
                }
            }
        }

        Ok(resolved)
    }
}

impl BareParamsBuilder<'_> {
    /// Call with NoOp.
    pub fn call(&self, params: AppClientBareCallParams) -> Result<AppCallParams, String> {
        self.build_bare_app_call_params(params, OnApplicationComplete::NoOp)
    }

    /// Call with OptIn.
    pub fn opt_in(&self, params: AppClientBareCallParams) -> Result<AppCallParams, String> {
        self.build_bare_app_call_params(params, OnApplicationComplete::OptIn)
    }

    /// Call with CloseOut.
    pub fn close_out(&self, params: AppClientBareCallParams) -> Result<AppCallParams, String> {
        self.build_bare_app_call_params(params, OnApplicationComplete::CloseOut)
    }

    /// Call with Delete.
    pub fn delete(&self, params: AppClientBareCallParams) -> Result<AppDeleteParams, String> {
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
    pub fn clear_state(&self, params: AppClientBareCallParams) -> Result<AppCallParams, String> {
        self.build_bare_app_call_params(params, OnApplicationComplete::ClearState)
    }

    /// Update with bare call.
    pub async fn update(
        &self,
        params: AppClientBareCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<AppUpdateParams, String> {
        // Compile programs (and populate AppManager cache/source maps)
        let cp = compilation_params.unwrap_or_default();
        let (approval_program, clear_state_program) = self
            .client
            .compile_with_params(&cp)
            .await
            .map_err(|e| e.to_string())?;

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
        default_on_complete: OnApplicationComplete,
    ) -> Result<AppCallParams, String> {
        Ok(AppCallParams {
            common_params: self.build_common_params_from_bare(&params)?,
            app_id: self
                .client
                .app_id
                .ok_or_else(|| "Missing app_id".to_string())?,
            on_complete: params.on_complete.unwrap_or(default_on_complete),
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
    ) -> Result<CommonTransactionParams, String> {
        Ok(CommonTransactionParams {
            sender: self.client.get_sender_address(&params.sender)?,
            signer: None,
            rekey_to: AppClient::get_optional_address(&params.rekey_to)?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
        })
    }
}
