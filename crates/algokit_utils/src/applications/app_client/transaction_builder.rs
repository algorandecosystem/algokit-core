use crate::transactions::composer::ComposerError;
use algokit_transact::OnApplicationComplete;

use super::types::{AppClientBareCallParams, AppClientMethodCallParams, CompilationParams};
use super::{AppClient, FundAppAccountParams};
use std::str::FromStr;

pub struct TransactionBuilder<'a> {
    pub(crate) client: &'a AppClient,
}

pub struct BareTransactionBuilder<'a> {
    pub(crate) client: &'a AppClient,
}

impl TransactionBuilder<'_> {
    /// Get the bare transaction builder.
    pub fn bare(&self) -> BareTransactionBuilder<'_> {
        BareTransactionBuilder {
            client: self.client,
        }
    }

    /// Call a method with NoOp.
    pub async fn call(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::NoOp)
            .await
    }

    /// Call a method with OptIn.
    pub async fn opt_in(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::OptIn)
            .await
    }

    /// Call a method with CloseOut.
    pub async fn close_out(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::CloseOut)
            .await
    }

    /// Call a method with Delete.
    pub async fn delete(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::DeleteApplication)
            .await
    }

    /// Update the application with method call.
    pub async fn update(
        &self,
        params: AppClientMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<crate::transactions::BuiltTransactions, ComposerError> {
        self.update_method(params, compilation_params).await
    }

    /// Fund the application account.
    pub async fn fund_app_account(
        &self,
        params: FundAppAccountParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        let payment = self
            .client
            .params()
            .fund_app_account(&params)
            .map_err(|e| ComposerError::TransactionError { message: e })?;
        self.client.algorand.create().payment(payment).await
    }

    async fn method_call_with_on_complete(
        &self,
        mut params: AppClientMethodCallParams,
        on_complete: OnApplicationComplete,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        params.on_complete = Some(on_complete);
        let method_params = self
            .client
            .params()
            .method_call(&params)
            .await
            .map_err(|e| ComposerError::TransactionError { message: e })?;
        let built = self
            .client
            .algorand
            .create()
            .app_call_method_call(method_params)
            .await?;
        Ok(built.transactions[0].clone())
    }

    async fn update_method(
        &self,
        params: AppClientMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<crate::transactions::BuiltTransactions, ComposerError> {
        // Compile TEAL and populate AppManager cache
        let (approval_teal_bytes, clear_teal_bytes) = if let Some(ref cp) = compilation_params {
            self.client.compile_with_params(cp).await.map_err(|e| {
                ComposerError::TransactionError {
                    message: e.to_string(),
                }
            })?
        } else {
            // Fallback: decode source and compile with defaults
            let default_cp = CompilationParams::default();
            self.client
                .compile_with_params(&default_cp)
                .await
                .map_err(|e| ComposerError::TransactionError {
                    message: e.to_string(),
                })?
        };

        // Build AppUpdateMethodCallParams
        let common_params = crate::transactions::CommonParams {
            sender: self
                .client
                .get_sender_address(&params.sender)
                .map_err(|e| ComposerError::TransactionError { message: e })?,
            signer: None,
            rekey_to: AppClient::get_optional_address(&params.rekey_to)
                .map_err(|e| ComposerError::TransactionError { message: e })?,
            note: params.note.clone(),
            lease: params.lease,
            static_fee: params.static_fee,
            extra_fee: params.extra_fee,
            max_fee: params.max_fee,
            validity_window: params.validity_window,
            first_valid_round: params.first_valid_round,
            last_valid_round: params.last_valid_round,
        };

        let to_abimethod =
            |method_name_or_sig: &str| -> Result<algokit_abi::ABIMethod, ComposerError> {
                let m = self
                    .client
                    .app_spec
                    .get_arc56_method(method_name_or_sig)
                    .map_err(|e| ComposerError::TransactionError {
                        message: e.to_string(),
                    })?;
                m.to_abi_method()
                    .map_err(|e| ComposerError::TransactionError {
                        message: e.to_string(),
                    })
            };

        let parse_account_refs = |account_refs: &Option<Vec<String>>| -> Result<
            Option<Vec<algokit_transact::Address>>,
            ComposerError,
        > {
            match account_refs {
                None => Ok(None),
                Some(refs) => {
                    let mut result = Vec::with_capacity(refs.len());
                    for s in refs {
                        result.push(algokit_transact::Address::from_str(s).map_err(|e| {
                            ComposerError::TransactionError {
                                message: e.to_string(),
                            }
                        })?);
                    }
                    Ok(Some(result))
                }
            }
        };

        let encode_args = |args: &Option<Vec<crate::transactions::app_call::AppMethodCallArg>>| -> Vec<crate::transactions::AppMethodCallArg> {
            args.as_ref()
                .cloned()
                .unwrap_or_default()
        };

        let update_params = crate::transactions::AppUpdateMethodCallParams {
            common_params,
            app_id: self
                .client
                .app_id()
                .ok_or(ComposerError::TransactionError {
                    message: "Missing app_id".to_string(),
                })?,
            approval_program: approval_teal_bytes,
            clear_state_program: clear_teal_bytes,
            method: to_abimethod(&params.method)?,
            args: encode_args(&params.args),
            account_references: parse_account_refs(&params.account_references)?,
            app_references: params.app_references.clone(),
            asset_references: params.asset_references.clone(),
            box_references: params.box_references.clone(),
        };

        self.client
            .algorand
            .create()
            .app_update_method_call(update_params)
            .await
    }
}

impl BareTransactionBuilder<'_> {
    /// Call with NoOp.
    pub async fn call(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        let app_call = self
            .client
            .params()
            .bare()
            .call(params)
            .map_err(|e| ComposerError::TransactionError { message: e })?;
        self.client.algorand.create().app_call(app_call).await
    }

    /// Call with OptIn.
    pub async fn opt_in(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        let app_call = self
            .client
            .params()
            .bare()
            .opt_in(params)
            .map_err(|e| ComposerError::TransactionError { message: e })?;
        self.client.algorand.create().app_call(app_call).await
    }

    /// Call with CloseOut.
    pub async fn close_out(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        let app_call = self
            .client
            .params()
            .bare()
            .close_out(params)
            .map_err(|e| ComposerError::TransactionError { message: e })?;
        self.client.algorand.create().app_call(app_call).await
    }

    /// Call with Delete.
    pub async fn delete(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        let app_call = self
            .client
            .params()
            .bare()
            .delete(params)
            .map_err(|e| ComposerError::TransactionError { message: e })?;
        self.client.algorand.create().app_call(app_call).await
    }

    /// Call with ClearState.
    pub async fn clear_state(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        let app_call = self
            .client
            .params()
            .bare()
            .clear_state(params)
            .map_err(|e| ComposerError::TransactionError { message: e })?;
        self.client.algorand.create().app_call(app_call).await
    }

    /// Update with bare call.
    pub async fn update(
        &self,
        params: AppClientBareCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<crate::transactions::BuiltTransactions, ComposerError> {
        // Compile TEAL and populate AppManager cache
        let (approval_teal_bytes, clear_teal_bytes) = if let Some(ref cp) = compilation_params {
            self.client.compile_with_params(cp).await.map_err(|e| {
                ComposerError::TransactionError {
                    message: e.to_string(),
                }
            })?
        } else {
            let default_cp = CompilationParams::default();
            self.client
                .compile_with_params(&default_cp)
                .await
                .map_err(|e| ComposerError::TransactionError {
                    message: e.to_string(),
                })?
        };

        let app_call = self
            .client
            .params()
            .bare()
            .update(params, compilation_params)
            .map_err(|e| ComposerError::TransactionError { message: e })?;

        // Build update params with compiled programs
        let update_params = crate::transactions::AppUpdateParams {
            common_params: app_call.common_params,
            app_id: app_call.app_id,
            approval_program: approval_teal_bytes,
            clear_state_program: clear_teal_bytes,
            args: app_call.args,
            account_references: app_call.account_references,
            app_references: app_call.app_references,
            asset_references: app_call.asset_references,
            box_references: app_call.box_references,
        };

        let built = self
            .client
            .algorand
            .create()
            .app_update(update_params)
            .await?;
        Ok(crate::transactions::BuiltTransactions {
            transactions: vec![built],
            method_calls: std::collections::HashMap::new(),
            signers: std::collections::HashMap::new(),
        })
    }
}
