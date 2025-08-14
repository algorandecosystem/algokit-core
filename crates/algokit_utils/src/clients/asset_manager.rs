use algod_client::apis::{AlgodClient, Error as AlgodError};
use algod_client::models::{Asset, AccountAssetInformation as AlgodAccountAssetInformation};
use algokit_transact::Address;
use std::{str::FromStr, sync::Arc};

use crate::transactions::{
    AssetOptInParams, AssetOptOutParams, CommonParams, Composer, ComposerError,
    TransactionSignerGetter,
};

#[derive(Debug, Clone)]
pub struct BulkAssetOptInOutResult {
    pub asset_id: u64,
    pub transaction_id: String,
}

// Note: AssetInformation has been replaced with algod_client::models::Asset
// AccountAssetInformation has been replaced with algod_client::models::AccountAssetInformation

#[derive(Debug, Clone)]
pub struct AssetValidationError {
    pub asset_id: u64,
    pub error: String,
}

/// Manages Algorand Standard Assets.
#[derive(Clone)]
pub struct AssetManager {
    algod_client: Arc<AlgodClient>,
}

impl AssetManager {
    pub fn new(algod_client: Arc<AlgodClient>) -> Self {
        Self { algod_client }
    }

    /// Get asset information by asset ID.
    /// Returns the raw algod Asset type with all asset parameters.
    /// Access asset parameters via `asset.params.*` fields.
    pub async fn get_by_id(&self, asset_id: u64) -> Result<Asset, AssetManagerError> {
        self.algod_client
            .get_asset_by_id(asset_id)
            .await
            .map_err(AssetManagerError::AlgodClientError)
    }

    /// Get account's asset information.
    /// Returns the raw algod AccountAssetInformation type.
    /// Access asset holding via `account_info.asset_holding` and asset params via `account_info.asset_params`.
    pub async fn get_account_information(
        &self,
        sender: &Address,
        asset_id: u64,
    ) -> Result<AlgodAccountAssetInformation, AssetManagerError> {
        self.algod_client
            .account_asset_information(&sender.to_string(), asset_id, None)
            .await
            .map_err(AssetManagerError::AlgodClientError)
    }

    pub async fn bulk_opt_in(
        &self,
        account: &Address,
        asset_ids: &[u64],
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<Vec<BulkAssetOptInOutResult>, AssetManagerError> {
        if asset_ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut composer = Composer::new(self.algod_client.clone(), signer_getter);

        // Add asset opt-in transactions for each asset
        for &asset_id in asset_ids {
            let opt_in_params = AssetOptInParams {
                common_params: CommonParams {
                    sender: account.clone(),
                    ..Default::default()
                },
                asset_id,
            };

            composer
                .add_asset_opt_in(opt_in_params)
                .map_err(AssetManagerError::ComposerError)?;
        }

        // Send the transaction group
        let results = composer
            .send(Default::default())
            .await
            .map_err(AssetManagerError::ComposerError)?;

        // Map transaction IDs back to assets
        let bulk_results: Vec<BulkAssetOptInOutResult> = asset_ids
            .iter()
            .zip(results.transaction_ids.iter())
            .map(|(&asset_id, transaction_id)| BulkAssetOptInOutResult {
                asset_id,
                transaction_id: transaction_id.clone(),
            })
            .collect();

        Ok(bulk_results)
    }

    pub async fn bulk_opt_out(
        &self,
        account: &Address,
        asset_ids: &[u64],
        ensure_zero_balance: Option<bool>,
        signer_getter: Arc<dyn TransactionSignerGetter>,
    ) -> Result<Vec<BulkAssetOptInOutResult>, AssetManagerError> {
        if asset_ids.is_empty() {
            return Ok(Vec::new());
        }

        let should_check_balance = ensure_zero_balance.unwrap_or(false);

        // If we need to check balances, verify they are all zero
        if should_check_balance {
            for &asset_id in asset_ids {
                let account_info = self.get_account_information(account, asset_id).await?;
                let balance = account_info
                    .asset_holding
                    .as_ref()
                    .map(|h| h.amount)
                    .unwrap_or(0);
                if balance > 0 {
                    return Err(AssetManagerError::NonZeroBalance {
                        address: account.to_string(),
                        asset_id,
                        balance,
                    });
                }
            }
        }

        // Fetch asset information to get creators
        let mut asset_creators = Vec::new();
        for &asset_id in asset_ids {
            let asset_info = self.get_by_id(asset_id).await?;
            let creator = Address::from_str(&asset_info.params.creator)
                .map_err(|_| AssetManagerError::AssetNotFound(asset_id))?;
            asset_creators.push(creator);
        }

        let mut composer = Composer::new(self.algod_client.clone(), signer_getter);

        // Add asset opt-out transactions for each asset
        for (i, &asset_id) in asset_ids.iter().enumerate() {
            let opt_out_params = AssetOptOutParams {
                common_params: CommonParams {
                    sender: account.clone(),
                    ..Default::default()
                },
                asset_id,
                close_remainder_to: asset_creators[i].clone(),
            };

            composer
                .add_asset_opt_out(opt_out_params)
                .map_err(AssetManagerError::ComposerError)?;
        }

        // Send the transaction group
        let results = composer
            .send(Default::default())
            .await
            .map_err(AssetManagerError::ComposerError)?;

        // Map transaction IDs back to assets
        let bulk_results: Vec<BulkAssetOptInOutResult> = asset_ids
            .iter()
            .zip(results.transaction_ids.iter())
            .map(|(&asset_id, transaction_id)| BulkAssetOptInOutResult {
                asset_id,
                transaction_id: transaction_id.clone(),
            })
            .collect();

        Ok(bulk_results)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AssetManagerError {
    #[error("Algod client error: {0}")]
    AlgodClientError(AlgodError),

    #[error("Composer error: {0}")]
    ComposerError(ComposerError),

    #[error("Asset not found: {0}")]
    AssetNotFound(u64),

    #[error("Account not found: {0}")]
    AccountNotFound(String),

    #[error("Account {address} is not opted into asset {asset_id}")]
    NotOptedIn { address: String, asset_id: u64 },

    #[error("Account {address} has non-zero balance {balance} for asset {asset_id}")]
    NonZeroBalance {
        address: String,
        asset_id: u64,
        balance: u64,
    },

    #[error("Asset {asset_id} is frozen for account {address}")]
    AssetFrozen { address: String, asset_id: u64 },

    #[error("Method '{method}' not implemented: {reason}")]
    NotImplemented { method: String, reason: String },
}
