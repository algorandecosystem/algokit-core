use algokit_abi::ABIMethod;
use algokit_transact::OnApplicationComplete;

use crate::transactions::{
    AppCallMethodCallParams, AppCallParams, AppMethodCallArg, CommonTransactionParams,
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
    ) -> Result<AppCallMethodCallParams, String> {
        self.method_call_with_on_complete(params, OnApplicationComplete::DeleteApplication)
            .await
    }

    /// Update the application with a method call.
    pub async fn update(
        &self,
        params: AppClientMethodCallParams,
        _compilation_params: Option<CompilationParams>,
    ) -> Result<AppCallMethodCallParams, String> {
        self.method_call_with_on_complete(params, OnApplicationComplete::UpdateApplication)
            .await
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
        use crate::transactions::app_call::AppMethodCallArg as Arg;
        let mut resolved: Vec<Arg> = Vec::with_capacity(method.args.len());
        for (i, m_arg) in method.args.iter().enumerate() {
            if let Some(Some(arg)) = provided.as_ref().map(|v| v.get(i)) {
                resolved.push(arg.clone());
                continue;
            }

            // Fill defaults only for value-type args
            if let Ok(signature) = method.signature() {
                if let Ok(m) = self.client.app_spec().get_arc56_method(&signature) {
                    if let Some(def) = m.args.get(i).and_then(|a| a.default_value.clone()) {
                        let arg_type_string = match &m_arg.arg_type {
                            algokit_abi::ABIMethodArgType::Value(t) => t.to_string(),
                            other => format!("{:?}", other),
                        };
                        let value = self
                            .client
                            .resolve_default_value_for_arg(&def, &arg_type_string, sender)
                            .await?;
                        resolved.push(Arg::ABIValue(value));
                        continue;
                    }
                }
            }

            // No provided value or default
            if let algokit_abi::ABIMethodArgType::Value(_) = &m_arg.arg_type {
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
    pub fn delete(&self, params: AppClientBareCallParams) -> Result<AppCallParams, String> {
        self.build_bare_app_call_params(params, OnApplicationComplete::DeleteApplication)
    }

    /// Call with ClearState.
    pub fn clear_state(&self, params: AppClientBareCallParams) -> Result<AppCallParams, String> {
        self.build_bare_app_call_params(params, OnApplicationComplete::ClearState)
    }

    /// Update with bare call.
    pub fn update(
        &self,
        params: AppClientBareCallParams,
        _compilation_params: Option<CompilationParams>,
    ) -> Result<AppCallParams, String> {
        self.build_bare_app_call_params(params, OnApplicationComplete::UpdateApplication)
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
