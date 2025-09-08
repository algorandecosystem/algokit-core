import type { ApplicationLocalReference, AssetHoldingReference, BoxReference } from "./index";

/**
 * These are resources that were accessed by this group that would normally have caused failure, but were allowed in simulation. Depending on where this object is in the response, the unnamed resources it contains may or may not qualify for group resource sharing. If this is a field in SimulateTransactionGroupResult, the resources do qualify, but if this is a field in SimulateTransactionResult, they do not qualify. In order to make this group valid for actual submission, resources that qualify for group sharing can be made available by any transaction of the group; otherwise, resources must be placed in the same transaction which accessed them.
 */
export type SimulateUnnamedResourcesAccessed = {
  accounts?: string[];
  assets?: bigint[];
  apps?: bigint[];
  boxes?: BoxReference[];
  extraBoxRefs?: bigint;
  assetHoldings?: AssetHoldingReference[];
  appLocals?: ApplicationLocalReference[];
};
