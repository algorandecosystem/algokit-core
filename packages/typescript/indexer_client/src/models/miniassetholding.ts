/**
 * A simplified version of AssetHolding
 */
export type MiniAssetHolding = {
  address: string;
  amount: bigint;
  isFrozen: boolean;

  /**
   * Whether or not this asset holding is currently deleted from its account.
   */
  deleted?: boolean;

  /**
   * Round during which the account opted into the asset.
   */
  optedInAtRound?: bigint;

  /**
   * Round during which the account opted out of the asset.
   */
  optedOutAtRound?: bigint;
};
