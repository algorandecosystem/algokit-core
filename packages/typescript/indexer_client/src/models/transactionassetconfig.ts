import type { AssetParams } from "./index";

/**
 * Fields for asset allocation, re-configuration, and destruction.
 *
 *
 * A zero value for asset-id indicates asset creation.
 * A zero value for the params indicates asset destruction.
 *
 * Definition:
 * data/transactions/asset.go : AssetConfigTxnFields
 */
export type TransactionAssetConfig = {
  /**
   * \[xaid\] ID of the asset being configured or empty if creating.
   */
  assetId?: bigint;
  params?: AssetParams;
};
