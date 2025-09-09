/**
 * Fields for an asset freeze transaction.
 *
 * Definition:
 * data/transactions/asset.go : AssetFreezeTxnFields
 */
export type TransactionAssetFreeze = {
  /**
   * \[fadd\] Address of the account whose asset is being frozen or thawed.
   */
  address: string;

  /**
   * \[faid\] ID of the asset being frozen or thawed.
   */
  assetId: bigint;

  /**
   * \[afrz\] The new freeze status.
   */
  newFreezeStatus: boolean;
};
