import type { AssetHolding, AssetParams } from "./index";

/**
 * AccountAssetResponse describes the account's asset holding and asset parameters (if either exist) for a specific asset ID. Asset parameters will only be returned if the provided address is the asset's creator.
 */
export type AccountAssetInformation = { round: bigint; "asset-holding"?: AssetHolding; "created-asset"?: AssetParams };
