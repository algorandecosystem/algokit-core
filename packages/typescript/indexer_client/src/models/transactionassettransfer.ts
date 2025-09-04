/**
 * Fields for an asset transfer transaction.
 *
 * Definition:
 * data/transactions/asset.go : AssetTransferTxnFields
 */
export type TransactionAssetTransfer = {
  amount: bigint;
  "asset-id": bigint;
  "close-amount"?: bigint;
  "close-to"?: string;
  receiver: string;
  sender?: string;
};
