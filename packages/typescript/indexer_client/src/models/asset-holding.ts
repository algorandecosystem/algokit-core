/**
 * Describes an asset held by an account.
 *
 * Definition:
 * data/basics/userBalance.go : AssetHolding
 */
export type AssetHolding = {
  /**
   * number of units held.
   */
  amount: bigint;

  /**
   * Asset ID of the holding.
   */
  assetId: bigint;

  /**
   * whether or not the holding is frozen.
   */
  isFrozen: boolean;

  /**
   * Whether or not the asset holding is currently deleted from its account.
   */
  deleted?: boolean;

  /**
   * Round during which the account opted into this asset holding.
   */
  optedInAtRound?: bigint;

  /**
   * Round during which the account opted out of this asset holding.
   */
  optedOutAtRound?: bigint;
};
