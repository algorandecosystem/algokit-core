use algokit_transact::Address;

use super::common::CommonParams;

/// Parameters to freeze an asset for a target account.
#[derive(Debug, Clone)]
pub struct AssetFreezeParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset being frozen.
    pub asset_id: u64,

    /// The address of the account whose asset is being frozen.
    pub target_account: Address,
}

/// Parameters to unfreeze an asset for a target account.
#[derive(Debug, Clone)]
pub struct AssetUnfreezeParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The ID of the asset being unfrozen.
    pub asset_id: u64,

    /// The address of the account whose asset is being unfrozen.
    pub target_account: Address,
}
