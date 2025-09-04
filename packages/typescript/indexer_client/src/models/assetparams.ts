/**
 * AssetParams specifies the parameters for an asset.
 *
 * \[apar\] when part of an AssetConfig transaction.
 *
 * Definition:
 * data/transactions/asset.go : AssetParams
 */
export type AssetParams = {
  clawback?: string;
  creator: string;
  decimals: number;
  "default-frozen"?: boolean;
  freeze?: string;
  manager?: string;
  "metadata-hash"?: string;
  name?: string;
  "name-b64"?: string;
  reserve?: string;
  total: bigint;
  "unit-name"?: string;
  "unit-name-b64"?: string;
  url?: string;
  "url-b64"?: string;
};
