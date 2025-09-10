use crate::transactions::composer::SimulateParams;
use crate::transactions::{SendTransactionResult, TransactionComposerConfig};
use crate::{AppClientError, ResourcePopulation, SendAppCallResult};
use algod_client::models::SimulateTraceConfig;
use algokit_transact::{MAX_SIMULATE_OPCODE_BUDGET, OnApplicationComplete};

use super::types::{AppClientBareCallParams, AppClientMethodCallParams, CompilationParams};
use super::{AppClient, FundAppAccountParams};

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

    pub async fn call(
        &self,
        params: AppClientMethodCallParams,
        on_complete: Option<OnApplicationComplete>,
    ) -> Result<SendAppCallResult, AppClientError> {
        let arc56_method = self
            .client
            .app_spec
            .get_arc56_method(&params.method)
            .map_err(|e| AppClientError::ABIError { source: e })?;

        let method_params = self.client.params().call(params, on_complete).await?;

        if method_params.on_complete == OnApplicationComplete::NoOp
            && arc56_method.readonly == Some(true)
        {
            // TODO: send params
            let mut composer = self
                .client
                .algorand()
                .new_group(Some(TransactionComposerConfig {
                    populate_app_call_resources: ResourcePopulation::Disabled,
                    cover_app_call_inner_transaction_fees: false,
                }));

            composer
                .add_app_call_method_call(method_params)
                .map_err(|e| AppClientError::ComposerError { source: e });

            let simulate_params = SimulateParams {
                allow_more_logging: Some(true),
                allow_empty_signatures: Some(true),
                exec_trace_config: Some(SimulateTraceConfig {
                    enable: Some(true),
                    scratch_change: Some(true),
                    stack_change: Some(true),
                    state_change: Some(true),
                }),
                skip_signatures: true,
                extra_opcode_budget: Some(MAX_SIMULATE_OPCODE_BUDGET),
                ..Default::default()
            };

            let simulate_results = composer
                .simulate(Some(simulate_params))
                .await
                .map_err(|e| AppClientError::ComposerError { source: e });

            Err(AppClientError::ValidationError {
                message: "Not implemented".to_string(),
            })
            // // Convert Transaction objects to transaction IDs (strings)
            // let tx_ids: Vec<String> = simulate_results
            //     .transactions
            //     .iter()
            //     .map(|tx| tx.id().unwrap_or_default())
            //     .collect();

            // let send_transaction_result = SendTransactionResult::new(
            //     "",
            //     tx_ids,
            //     Some(simulate_results.transactions),
            //     Some(simulate_results.confirmations),
            //     simulate_results.returns,
            // )?;

            // Ok(SendAppCallResult {
            //     common_params: send_transaction_result,
            //     abi_return: simulate_results.returns.first().cloned(),
            // })
            // TODO: convert SimulateComposerResults to SendAppCallResult
        } else {
            self.client
                .algorand
                .send()
                .app_call_method_call(method_params, None)
                .await
                .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
        }
    }

    /// Call a method with OptIn.
    pub async fn opt_in(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<crate::transactions::SendAppCallResult, AppClientError> {
        let method_params = self.client.params().opt_in(params).await?;

        self.client
            .algorand
            .send()
            .app_call_method_call(method_params, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }

    /// Call a method with CloseOut.
    pub async fn close_out(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<crate::transactions::SendAppCallResult, AppClientError> {
        let method_params = self.client.params().close_out(params).await?;

        self.client
            .algorand
            .send()
            .app_call_method_call(method_params, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }

    /// Call a method with Delete.
    pub async fn delete(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<crate::transactions::SendAppCallResult, AppClientError> {
        let delete_params = self.client.params().delete(params).await?;

        self.client
            .algorand
            .send()
            .app_delete_method_call(delete_params, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }

    /// Update the application with a method call.
    pub async fn update(
        &self,
        params: AppClientMethodCallParams,
        compilation_params: Option<CompilationParams>, // TODO: consider creating MethodCallParams struct for each of these method so on_complete and compilation_params are handled better
    ) -> Result<crate::transactions::SendAppUpdateResult, AppClientError> {
        let update_params = self
            .client
            .params()
            .update(params, compilation_params)
            .await?;

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
    ) -> Result<SendTransactionResult, AppClientError> {
        let payment = self.client.params().fund_app_account(&params)?;

        self.client
            .algorand
            .send()
            .payment(payment, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }
}

impl BareTransactionSender<'_> {
    /// Call with NoOp.
    pub async fn call(
        &self,
        params: AppClientBareCallParams,
        on_complete: Option<OnApplicationComplete>,
    ) -> Result<SendTransactionResult, AppClientError> {
        let params = self.client.params().bare().call(params, on_complete)?;
        self.client
            .algorand
            .send()
            .app_call(params, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }

    /// Call with OptIn.
    pub async fn opt_in(
        &self,
        params: AppClientBareCallParams,
    ) -> Result<SendTransactionResult, AppClientError> {
        let app_call = self.client.params().bare().opt_in(params)?;
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
    ) -> Result<SendTransactionResult, AppClientError> {
        let app_call = self.client.params().bare().close_out(params)?;
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
    ) -> Result<SendTransactionResult, AppClientError> {
        let delete_params = self.client.params().bare().delete(params)?;
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
    ) -> Result<SendTransactionResult, AppClientError> {
        let app_call = self.client.params().bare().clear_state(params)?;
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
    ) -> Result<crate::transactions::SendAppUpdateResult, AppClientError> {
        let update_params = self
            .client
            .params()
            .bare()
            .update(params, compilation_params)
            .await?;

        self.client
            .algorand
            .send()
            .app_update(update_params, None)
            .await
            .map_err(|e| super::utils::transform_transaction_error(self.client, e, false))
    }
}
