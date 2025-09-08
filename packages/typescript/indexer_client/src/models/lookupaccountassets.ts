import type { AssetHolding } from "./index";

/**
 * (empty)
 */
export type LookupAccountAssets = { currentRound: bigint; nextToken?: string; assets: AssetHolding[] };
