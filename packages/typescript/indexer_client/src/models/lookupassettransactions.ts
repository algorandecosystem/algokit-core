import type { Transaction } from "./index";

/**
 * (empty)
 */
export type LookupAssetTransactions = { currentRound: bigint; nextToken?: string; transactions: Transaction[] };
