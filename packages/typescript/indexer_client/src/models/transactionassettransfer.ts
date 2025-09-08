/**
 * Fields for an asset transfer transaction.
 *
 * Definition:
 * data/transactions/asset.go : AssetTransferTxnFields
 */
export type TransactionAssetTransfer = {
  amount: bigint;
  assetId: bigint;
  closeAmount?: bigint;
  closeTo?: string;
  receiver: string;
  sender?: string;
};
