import type { Account } from "./index";

/**
 * (empty)
 */
export type SearchForAccounts = { accounts: Account[]; "current-round": number; "next-token"?: string };
