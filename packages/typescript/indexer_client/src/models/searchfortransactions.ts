import type { Transaction } from "./index";

/**
 * (empty)
 */
export type SearchForTransactions = { "current-round": number; "next-token"?: string; transactions: Transaction[] };
