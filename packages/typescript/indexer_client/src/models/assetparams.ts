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
  decimals: bigint;
  defaultFrozen?: boolean;
  freeze?: string;
  manager?: string;
  metadataHash?: string;
  name?: string;
  nameB64?: string;
  reserve?: string;
  total: bigint;
  unitName?: string;
  unitNameB64?: string;
  url?: string;
  urlB64?: string;
};
