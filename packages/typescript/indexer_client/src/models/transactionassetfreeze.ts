/**
 * Fields for an asset freeze transaction.
 *
 * Definition:
 * data/transactions/asset.go : AssetFreezeTxnFields
 */
export type TransactionAssetFreeze = { address: string; "asset-id": bigint; "new-freeze-status": boolean };
