use crate::transactions::composer::ComposerError;
use algokit_transact::OnApplicationComplete;

use super::types::{AppClientBareCallParams, AppClientMethodCallParams, CompilationParams};
use super::{AppClient, FundAppAccountParams};

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

    /// Creates an ABI method call with NoOp.
    pub async fn call(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::NoOp)
            .await
    }

    /// Creates an ABI method call with OptIn.
    pub async fn opt_in(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::OptIn)
            .await
    }

    /// Creates an ABI method call with CloseOut.
    pub async fn close_out(
        &self,
        params: AppClientMethodCallParams,
    ) -> Result<algokit_transact::Transaction, ComposerError> {
        self.method_call_with_on_complete(params, OnApplicationComplete::CloseOut)
            .await
    }

    /// Creates an ABI method call with Delete.
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
        // Build update params via params builder (includes compilation)
        let update_params = self
            .client
            .params()
            .update(params, compilation_params)
            .await
            .map_err(|e| ComposerError::TransactionError { message: e })?;

        // Create transactions directly using update params
        let built = self
            .client
            .algorand
            .create()
            .app_update_method_call(update_params)
            .await?;
        Ok(built)
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
        let is_delete = matches!(
            method_params.on_complete,
            OnApplicationComplete::DeleteApplication
        );
        let built = if is_delete {
            // Route delete on-complete to delete-specific API
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
                .create()
                .app_delete_method_call(delete_params)
                .await?
        } else {
            self.client
                .algorand
                .create()
                .app_call_method_call(method_params)
                .await?
        };
        Ok(built.transactions[0].clone())
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
        let delete_params = self
            .client
            .params()
            .bare()
            .delete(params)
            .map_err(|e| ComposerError::TransactionError { message: e })?;
        // Use delete-specific API for bare delete
        self.client
            .algorand
            .create()
            .app_delete(delete_params)
            .await
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
        // Build update params via params builder (includes compilation)
        let update_params = self
            .client
            .params()
            .bare()
            .update(params, compilation_params)
            .await
            .map_err(|e| ComposerError::TransactionError { message: e })?;

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
