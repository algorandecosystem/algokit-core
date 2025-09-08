import type { Account } from "./index";

/**
 * (empty)
 */
export type SearchForAccounts = { accounts: Account[]; currentRound: bigint; nextToken?: string };
