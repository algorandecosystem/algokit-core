/**
 * Describes an asset held by an account.
 *
 * Definition:
 * data/basics/userBalance.go : AssetHolding
 */
export type AssetHolding = {
  amount: bigint;
  assetId: bigint;
  isFrozen: boolean;
  deleted?: boolean;
  optedInAtRound?: bigint;
  optedOutAtRound?: bigint;
};
