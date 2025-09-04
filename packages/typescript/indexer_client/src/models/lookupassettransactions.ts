import type { Transaction } from "./index";

/**
 * (empty)
 */
export type LookupAssetTransactions = { "current-round": number; "next-token"?: string; transactions: Transaction[] };
