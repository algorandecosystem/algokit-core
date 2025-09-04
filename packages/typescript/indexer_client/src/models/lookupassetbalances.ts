import type { MiniAssetHolding } from "./index";

/**
 * (empty)
 */
export type LookupAssetBalances = { balances: MiniAssetHolding[]; "current-round": number; "next-token"?: string };
