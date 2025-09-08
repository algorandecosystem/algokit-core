import type { Transaction } from "./index";

/**
 * (empty)
 */
export type LookupAccountTransactions = { currentRound: bigint; nextToken?: string; transactions: Transaction[] };
