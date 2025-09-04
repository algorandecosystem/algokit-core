import type { Transaction } from "./index";

/**
 * (empty)
 */
export type LookupAccountTransactions = { "current-round": number; "next-token"?: string; transactions: Transaction[] };
