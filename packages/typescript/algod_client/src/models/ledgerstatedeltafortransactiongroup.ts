import type { LedgerStateDelta } from "./index";

/**
 * Contains a ledger delta for a single transaction group
 */
export type LedgerStateDeltaForTransactionGroup = { Delta: LedgerStateDelta; Ids: string[] };
