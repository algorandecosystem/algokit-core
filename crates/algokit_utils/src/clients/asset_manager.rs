use algod_client::apis::{AlgodClient, Error as AlgodError};
use algokit_transact::Address;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BulkAssetOptInOutResult {
    pub asset_id: u64,
    pub transaction_id: String,
}

#[derive(Debug, Clone)]
pub struct AccountAssetInformation {
    pub asset_id: u64,
    pub balance: u64,
    pub frozen: bool,
    pub round: u64,
}

#[derive(Debug, Clone)]
pub struct AssetInformation {
    pub asset_id: u64,
    pub creator: String,
    pub total: u64,
    pub decimals: u32,
    pub default_frozen: Option<bool>,
    pub manager: Option<String>,
    pub reserve: Option<String>,
    pub freeze: Option<String>,
    pub clawback: Option<String>,
    pub unit_name: Option<String>,
    pub unit_name_as_bytes: Option<Vec<u8>>,
    pub asset_name: Option<String>,
    pub asset_name_as_bytes: Option<Vec<u8>>,
    pub url: Option<String>,
    pub url_as_bytes: Option<Vec<u8>>,
    pub metadata_hash: Option<Vec<u8>>,
}

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

    pub async fn get_by_id(&self, asset_id: u64) -> Result<AssetInformation, AssetManagerError> {
        let asset = self
            .algod_client
            .get_asset_by_id(asset_id)
            .await
            .map_err(AssetManagerError::AlgodClientError)?;

        Ok(AssetInformation {
            asset_id,
            creator: asset.params.creator,
            total: asset.params.total,
            decimals: asset.params.decimals as u32,
            default_frozen: asset.params.default_frozen,
            manager: asset.params.manager,
            reserve: asset.params.reserve,
            freeze: asset.params.freeze,
            clawback: asset.params.clawback,
            unit_name: asset.params.unit_name,
            unit_name_as_bytes: asset.params.unit_name_b64,
            asset_name: asset.params.name,
            asset_name_as_bytes: asset.params.name_b64,
            url: asset.params.url,
            url_as_bytes: asset.params.url_b64,
            metadata_hash: asset.params.metadata_hash,
        })
    }

    pub async fn get_account_information(
        &self,
        sender: &Address,
        asset_id: u64,
    ) -> Result<AccountAssetInformation, AssetManagerError> {
        let account_info = self
            .algod_client
            .account_asset_information(&sender.to_string(), asset_id, None)
            .await
            .map_err(AssetManagerError::AlgodClientError)?;

        Ok(AccountAssetInformation {
            asset_id,
            balance: account_info
                .asset_holding
                .as_ref()
                .map(|h| h.amount)
                .unwrap_or(0),
            frozen: account_info
                .asset_holding
                .as_ref()
                .map(|h| h.is_frozen)
                .unwrap_or(false),
            round: account_info.round,
        })
    }

    pub async fn bulk_opt_in(
        &self,
        _account: &Address,
        _asset_ids: &[u64],
    ) -> Result<Vec<BulkAssetOptInOutResult>, AssetManagerError> {
        todo!("Implement bulk opt-in")
    }

    pub async fn bulk_opt_out(
        &self,
        _account: &Address,
        _asset_ids: &[u64],
        _ensure_zero_balance: Option<bool>,
    ) -> Result<Vec<BulkAssetOptInOutResult>, AssetManagerError> {
        todo!("Implement bulk opt-out")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AssetManagerError {
    #[error("Algod client error: {0}")]
    AlgodClientError(AlgodError),

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
