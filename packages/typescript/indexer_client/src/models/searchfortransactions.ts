import type { Transaction } from "./index";

/**
 * (empty)
 */
export type SearchForTransactions = { currentRound: bigint; nextToken?: string; transactions: Transaction[] };
