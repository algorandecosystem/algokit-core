import type { Asset } from "./index";

/**
 * (empty)
 */
export type LookupAccountCreatedAssets = { assets: Asset[]; currentRound: bigint; nextToken?: string };
