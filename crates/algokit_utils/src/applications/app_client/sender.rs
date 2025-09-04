use crate::transactions::{SendTransactionResult, TransactionSenderError};
use algokit_transact::OnApplicationComplete;

use super::types::{AppClientBareCallParams, AppClientMethodCallParams, CompilationParams};
use super::{AppClient, FundAppAccountParams};
// use std::str::FromStr; // no longer needed after refactor

pub struct TransactionSender<'a> {
    pub(crate) client: &'a AppClient,
}

pub struct BareTransactionSender<'a> {
    pub(crate) client: &'a AppClient,
}

impl<'a> TransactionSender<'a> {
    /// Get the bare transaction sender.
    pub fn bare(&self) -> BareTransactionSender<'a> {
        BareTransactionSender {
            client: self.client,
        }
    }

    /// Call a method with NoOp.
    pub async fn call(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<crate::transactions::SendAppCallResult, TransactionSenderError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::NoOp)
            .await
    }

    /// Call a method with OptIn.
    pub async fn opt_in(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<crate::transactions::SendAppCallResult, TransactionSenderError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::OptIn)
            .await
    }

    /// Call a method with CloseOut.
    pub async fn close_out(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<crate::transactions::SendAppCallResult, TransactionSenderError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::CloseOut)
            .await
    }

    /// Call a method with Delete.
    pub async fn delete(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<crate::transactions::SendAppCallResult, TransactionSenderError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::DeleteApplication)
            .await
    }

    /// Update the application with a method call.
    pub async fn update(
        &self,
        params: AppClientMethodCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<crate::transactions::SendAppUpdateResult, TransactionSenderError> {
        let update_params = self
            .client
            .params()
            .update(params, compilation_params)
            .await
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;

        self.client
            .algorand
            .send()
            .app_update_method_call(update_params, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }

    /// Fund the application account.
    pub async fn fund_app_account(
        &self,
        params: FundAppAccountParams,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        let payment = self
            .client
            .params()
            .fund_app_account(&params)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;
        self.client
            .algorand
            .send()
            .payment(payment, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }

    async fn method_call_with_on_complete(
        &self,
        mut params: AppClientMethodCallParams,
        on_complete: OnApplicationComplete,
    ) -> Result<crate::transactions::SendAppCallResult, TransactionSenderError> {
        params.on_complete = Some(on_complete);
        let method_params = self
            .client
            .params()
            .method_call(&params)
            .await
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;
        let is_delete = matches!(
            method_params.on_complete,
            OnApplicationComplete::DeleteApplication
        );
        // If debug enabled and readonly method, simulate with tracing
        let is_readonly = self.client.is_readonly_method(&method_params.method);
        if crate::config::Config::debug() && is_readonly {
            self.simulate_readonly_with_tracing_for_debug(&params, is_delete)
                .await?;
        }

        let result = if is_delete {
            let delete_params = crate::transactions::AppDeleteMethodCallParams {
                common_params: method_params.common_params.clone(),
                app_id: method_params.app_id,
                method: method_params.method.clone(),
                args: method_params.args.clone(),
                account_references: method_params.account_references.clone(),
                app_references: method_params.app_references.clone(),
                asset_references: method_params.asset_references.clone(),
                box_references: method_params.box_references.clone(),
            };
            self.client
                .algorand
                .send()
                .app_delete_method_call(delete_params, None)
                .await
                .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))?
        } else {
            self.client
                .algorand
                .send()
                .app_call_method_call(method_params, None)
                .await
                .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))?
        };

        // Returns are already ABI-decoded; expose as-is
        Ok(result)
    }

    // Simulate a readonly call when debug is enabled, emitting traces if configured.
    pub(crate) async fn simulate_readonly_with_tracing_for_debug(
        &self,
        params: &AppClientMethodCallParams,
        is_delete: bool,
    ) -> Result<(), TransactionSenderError> {
        let mut composer = self.client.algorand().new_group();
        if is_delete {
            let method_params_for_composer = self
                .client
                .params()
                .method_call(params)
                .await
                .map_err(|e| TransactionSenderError::ValidationError { message: e })?;
            let delete_params = crate::transactions::AppDeleteMethodCallParams {
                common_params: method_params_for_composer.common_params.clone(),
                app_id: method_params_for_composer.app_id,
                method: method_params_for_composer.method.clone(),
                args: method_params_for_composer.args.clone(),
                account_references: method_params_for_composer.account_references.clone(),
                app_references: method_params_for_composer.app_references.clone(),
                asset_references: method_params_for_composer.asset_references.clone(),
                box_references: method_params_for_composer.box_references.clone(),
            };
            composer
                .add_app_delete_method_call(delete_params)
                .map_err(|e| TransactionSenderError::ValidationError {
                    message: e.to_string(),
                })?;
        } else {
            let method_params_for_composer = self
                .client
                .params()
                .method_call(params)
                .await
                .map_err(|e| TransactionSenderError::ValidationError { message: e })?;
            composer
                .add_app_call_method_call(method_params_for_composer)
                .map_err(|e| TransactionSenderError::ValidationError {
                    message: e.to_string(),
                })?;
        }

        let sim_params = crate::transactions::composer::SimulateParams {
            allow_more_logging: Some(true),
            allow_empty_signatures: Some(true),
            exec_trace_config: Some(algod_client::models::SimulateTraceConfig {
                enable: Some(true),
                scratch_change: Some(true),
                stack_change: Some(true),
                state_change: Some(true),
            }),
            skip_signatures: true,
            ..Default::default()
        };

        let sim = composer.simulate(Some(sim_params)).await.map_err(|e| {
            TransactionSenderError::ValidationError {
                message: e.to_string(),
            }
        })?;

        if crate::config::Config::trace_all() {
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

        Ok(())
    }
}

impl BareTransactionSender<'_> {
    /// Call with NoOp.
    pub async fn call(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        let app_call = self
            .client
            .params()
            .bare()
            .call(params)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;
        self.client
            .algorand
            .send()
            .app_call(app_call, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }

    /// Call with OptIn.
    pub async fn opt_in(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        let app_call = self
            .client
            .params()
            .bare()
            .opt_in(params)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;
        self.client
            .algorand
            .send()
            .app_call(app_call, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }

    /// Call with CloseOut.
    pub async fn close_out(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        let app_call = self
            .client
            .params()
            .bare()
            .close_out(params)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;
        self.client
            .algorand
            .send()
            .app_call(app_call, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }

    /// Call with Delete.
    pub async fn delete(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        let delete_params = self
            .client
            .params()
            .bare()
            .delete(params)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;
        self.client
            .algorand
            .send()
            .app_delete(delete_params, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }

    /// Call with ClearState.
    pub async fn clear_state(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<SendTransactionResult, TransactionSenderError> {
        let app_call = self
            .client
            .params()
            .bare()
            .clear_state(params)
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;
        self.client
            .algorand
            .send()
            .app_call(app_call, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, true))
    }

    /// Update with bare call.
    pub async fn update(
        &self,
        params: AppClientBareCallParams,
        compilation_params: Option<CompilationParams>,
    ) -> Result<crate::transactions::SendAppUpdateResult, TransactionSenderError> {
        let update_params = self
            .client
            .params()
            .bare()
            .update(params, compilation_params)
            .await
            .map_err(|e| TransactionSenderError::ValidationError { message: e })?;

        self.client
            .algorand
            .send()
            .app_update(update_params, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }
}
