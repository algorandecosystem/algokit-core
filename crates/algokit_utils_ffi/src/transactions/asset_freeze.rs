use crate::transactions::common::UtilsError;

use super::common::CommonParams;
use algokit_utils::transactions::AssetFreezeParams as RustAssetFreezeParams;

#[derive(uniffi::Record)]
pub struct AssetFreezeParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset being frozen.
    pub asset_id: u64,

    /// The target account whose asset holdings will be frozen.
    pub target_address: String,
}

impl TryFrom<AssetFreezeParams> for RustAssetFreezeParams {
    type Error = UtilsError;

    fn try_from(params: AssetFreezeParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;
        Ok(RustAssetFreezeParams {
            common_params,
            asset_id: params.asset_id,
            target_address: params
                .target_address
                .parse()
                .map_err(|_| UtilsError::UtilsError {
                    message: "Invalid target address".to_string(),
                })?,
        })
    }
}
