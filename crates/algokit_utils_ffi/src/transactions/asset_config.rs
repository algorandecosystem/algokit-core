use crate::transactions::common::UtilsError;

use super::common::CommonParams;
use algokit_utils::transactions::{
    AssetCreateParams as RustAssetCreateParams, AssetDestroyParams as RustAssetDestroyParams,
    AssetReconfigureParams as RustAssetReconfigureParams,
};

// Helper function to parse optional address strings
fn parse_optional_address(
    addr_opt: Option<String>,
    field_name: &str,
) -> Result<Option<algokit_transact::Address>, UtilsError> {
    match addr_opt {
        Some(addr_str) => {
            let addr = addr_str.parse().map_err(|e| UtilsError::UtilsError {
                message: format!("Failed to parse {} address: {}", field_name, e),
            })?;
            Ok(Some(addr))
        }
        None => Ok(None),
    }
}

#[derive(uniffi::Record)]
pub struct AssetCreateParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The total amount of the smallest divisible (decimal) unit to create.
    ///
    /// For example, if creating an asset with 2 decimals and wanting a total supply of 100 units, this value should be 10000.
    pub total: u64,

    /// The amount of decimal places the asset should have.
    ///
    /// If unspecified then the asset will be in whole units (i.e. `0`).
    /// * If 0, the asset is not divisible;
    /// * If 1, the base unit of the asset is in tenths;
    /// * If 2, the base unit of the asset is in hundredths;
    /// * If 3, the base unit of the asset is in thousandths;
    ///
    /// and so on up to 19 decimal places.
    pub decimals: Option<u32>,

    /// Whether the asset is frozen by default for all accounts.
    /// Defaults to `false`.
    ///
    /// If `true` then for anyone apart from the creator to hold the
    /// asset it needs to be unfrozen per account using an asset freeze
    /// transaction from the `freeze` account, which must be set on creation.
    pub default_frozen: Option<bool>,

    /// The optional name of the asset.
    ///
    /// Max size is 32 bytes.
    pub asset_name: Option<String>,

    /// The optional name of the unit of this asset (e.g. ticker name).
    ///
    /// Max size is 8 bytes.
    pub unit_name: Option<String>,

    /// Specifies an optional URL where more information about the asset can be retrieved (e.g. metadata).
    ///
    /// Max size is 96 bytes.
    pub url: Option<String>,

    /// 32-byte hash of some metadata that is relevant to your asset and/or asset holders.
    ///
    /// The format of this metadata is up to the application.
    pub metadata_hash: Option<Vec<u8>>,

    /// The address of the optional account that can manage the configuration of the asset and destroy it.
    ///
    /// The configuration fields it can change are `manager`, `reserve`, `clawback`, and `freeze`.
    ///
    /// If not set or set to the Zero address the asset becomes permanently immutable.
    pub manager: Option<String>,

    /// The address of the optional account that holds the reserve (uncirculated supply) units of the asset.
    ///
    /// This address has no specific authority in the protocol itself and is informational only.
    ///
    /// Some standards like [ARC-19](https://github.com/algorandfoundation/ARCs/blob/main/ARCs/arc-0019.md)
    /// rely on this field to hold meaningful data.
    ///
    /// It can be used in the case where you want to signal to holders of your asset that the uncirculated units
    /// of the asset reside in an account that is different from the default creator account.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub reserve: Option<String>,

    /// The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
    ///
    /// If empty, freezing is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub freeze: Option<String>,

    /// The address of the optional account that can clawback holdings of this asset from any account.
    ///
    /// **This field should be used with caution** as the clawback account has the ability to **unconditionally take assets from any account**.
    ///
    /// If empty, clawback is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub clawback: Option<String>,
}

#[derive(uniffi::Record)]
pub struct AssetReconfigureParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// ID of the existing asset to be reconfigured.
    pub asset_id: u64,

    /// The address of the optional account that can manage the configuration of the asset and destroy it.
    ///
    /// The configuration fields it can change are `manager`, `reserve`, `clawback`, and `freeze`.
    ///
    /// If not set or set to the Zero address the asset becomes permanently immutable.
    pub manager: Option<String>,

    /// The address of the optional account that holds the reserve (uncirculated supply) units of the asset.
    ///
    /// This address has no specific authority in the protocol itself and is informational only.
    ///
    /// Some standards like [ARC-19](https://github.com/algorandfoundation/ARCs/blob/main/ARCs/arc-0019.md)
    /// rely on this field to hold meaningful data.
    ///
    /// It can be used in the case where you want to signal to holders of your asset that the uncirculated units
    /// of the asset reside in an account that is different from the default creator account.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub reserve: Option<String>,

    /// The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
    ///
    /// If empty, freezing is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub freeze: Option<String>,

    /// The address of the optional account that can clawback holdings of this asset from any account.
    ///
    /// **This field should be used with caution** as the clawback account has the ability to **unconditionally take assets from any account**.
    ///
    /// If empty, clawback is not permitted.
    ///
    /// If not set or set to the Zero address is permanently empty.
    pub clawback: Option<String>,
}

#[derive(uniffi::Record)]
pub struct AssetDestroyParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// ID of the existing asset to be destroyed.
    pub asset_id: u64,
}

impl TryFrom<AssetCreateParams> for RustAssetCreateParams {
    type Error = UtilsError;

    fn try_from(params: AssetCreateParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;

        // Convert metadata_hash if present
        let metadata_hash = match params.metadata_hash {
            Some(hash_vec) => {
                if hash_vec.len() != 32 {
                    return Err(UtilsError::UtilsError {
                        message: format!(
                            "metadata_hash must be exactly 32 bytes, got {}",
                            hash_vec.len()
                        ),
                    });
                }
                let mut hash_array = [0u8; 32];
                hash_array.copy_from_slice(&hash_vec);
                Some(hash_array)
            }
            None => None,
        };

        Ok(RustAssetCreateParams {
            common_params,
            total: params.total,
            decimals: params.decimals,
            default_frozen: params.default_frozen,
            asset_name: params.asset_name,
            unit_name: params.unit_name,
            url: params.url,
            metadata_hash,
            manager: parse_optional_address(params.manager, "manager")?,
            reserve: parse_optional_address(params.reserve, "reserve")?,
            freeze: parse_optional_address(params.freeze, "freeze")?,
            clawback: parse_optional_address(params.clawback, "clawback")?,
        })
    }
}

impl TryFrom<AssetReconfigureParams> for RustAssetReconfigureParams {
    type Error = UtilsError;

    fn try_from(params: AssetReconfigureParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;

        Ok(RustAssetReconfigureParams {
            common_params,
            asset_id: params.asset_id,
            manager: parse_optional_address(params.manager, "manager")?,
            reserve: parse_optional_address(params.reserve, "reserve")?,
            freeze: parse_optional_address(params.freeze, "freeze")?,
            clawback: parse_optional_address(params.clawback, "clawback")?,
        })
    }
}

impl TryFrom<AssetDestroyParams> for RustAssetDestroyParams {
    type Error = UtilsError;

    fn try_from(params: AssetDestroyParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;
        Ok(RustAssetDestroyParams {
            common_params,
            asset_id: params.asset_id,
        })
    }
}
