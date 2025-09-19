import type { Asset } from "./index";

export type LookupAssetById = {
  asset: Asset;

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint;
};
