/**
 * A simplified version of AssetHolding
 */
export type MiniAssetHolding = {
  address: string;
  amount: bigint;
  "is-frozen": boolean;
  deleted?: boolean;
  "opted-in-at-round"?: bigint;
  "opted-out-at-round"?: bigint;
};
