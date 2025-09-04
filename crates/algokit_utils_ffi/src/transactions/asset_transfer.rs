use crate::transactions::common::UtilsError;

use super::common::CommonParams;
use algokit_utils::transactions::{
    AssetClawbackParams as RustAssetClawbackParams, AssetOptInParams as RustAssetOptInParams,
    AssetOptOutParams as RustAssetOptOutParams, AssetTransferParams as RustAssetTransferParams,
};

#[derive(uniffi::Record)]
pub struct AssetTransferParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset being transferred.
    pub asset_id: u64,

    /// The amount of the asset to transfer.
    pub amount: u64,

    /// The address that will receive the asset.
    pub receiver: String,
}

#[derive(uniffi::Record)]
pub struct AssetOptInParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset to opt into.
    pub asset_id: u64,
}

#[derive(uniffi::Record)]
pub struct AssetOptOutParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset to opt out of.
    pub asset_id: u64,

    /// The address to close the remainder to. If None, defaults to the asset creator.
    pub close_remainder_to: Option<String>,
}

#[derive(uniffi::Record)]
pub struct AssetClawbackParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset being clawed back.
    pub asset_id: u64,

    /// The amount of the asset to clawback.
    pub amount: u64,

    /// The address that will receive the clawed back asset.
    pub receiver: String,

    /// The address from which assets are taken.
    pub clawback_target: String,
}

impl TryFrom<AssetTransferParams> for RustAssetTransferParams {
    type Error = UtilsError;

    fn try_from(params: AssetTransferParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;
        Ok(RustAssetTransferParams {
            common_params,
            asset_id: params.asset_id,
            amount: params.amount,
            receiver: params
                .receiver
                .parse()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Failed to parse receiver address: {}", e),
                })?,
        })
    }
}

impl TryFrom<AssetOptInParams> for RustAssetOptInParams {
    type Error = UtilsError;

    fn try_from(params: AssetOptInParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;
        Ok(RustAssetOptInParams {
            common_params,
            asset_id: params.asset_id,
        })
    }
}

impl TryFrom<AssetOptOutParams> for RustAssetOptOutParams {
    type Error = UtilsError;

    fn try_from(params: AssetOptOutParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;
        let close_remainder_to = match params.close_remainder_to {
            Some(addr) => Some(addr.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Failed to parse close_remainder_to address: {}", e),
            })?),
            None => None,
        };
        Ok(RustAssetOptOutParams {
            common_params,
            asset_id: params.asset_id,
            close_remainder_to,
        })
    }
}

impl TryFrom<AssetClawbackParams> for RustAssetClawbackParams {
    type Error = UtilsError;

    fn try_from(params: AssetClawbackParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;
        Ok(RustAssetClawbackParams {
            common_params,
            asset_id: params.asset_id,
            amount: params.amount,
            receiver: params
                .receiver
                .parse()
                .map_err(|e| UtilsError::UtilsError {
                    message: format!("Failed to parse receiver address: {}", e),
                })?,
            clawback_target: params.clawback_target.parse().map_err(|e| {
                UtilsError::UtilsError {
                    message: format!("Failed to parse clawback_target address: {}", e),
                }
            })?,
        })
    }
}
