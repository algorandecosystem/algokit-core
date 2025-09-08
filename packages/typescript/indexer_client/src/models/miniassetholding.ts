/**
 * A simplified version of AssetHolding
 */
export type MiniAssetHolding = {
  address: string;
  amount: bigint;
  isFrozen: boolean;
  deleted?: boolean;
  optedInAtRound?: bigint;
  optedOutAtRound?: bigint;
};
