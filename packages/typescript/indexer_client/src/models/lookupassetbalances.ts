import type { MiniAssetHolding } from "./index";

/**
 * (empty)
 */
export type LookupAssetBalances = { balances: MiniAssetHolding[]; currentRound: bigint; nextToken?: string };
