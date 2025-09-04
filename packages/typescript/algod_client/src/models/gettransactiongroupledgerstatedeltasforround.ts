import type { LedgerStateDeltaForTransactionGroup } from "./index";

/**
 * Response containing all ledger state deltas for transaction groups, with their associated Ids, in a single round.
 */
export type GetTransactionGroupLedgerStateDeltasForRound = { Deltas: LedgerStateDeltaForTransactionGroup[] };
