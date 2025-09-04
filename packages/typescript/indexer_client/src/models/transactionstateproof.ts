import type { IndexerStateProofMessage, StateProofFields } from "./index";

/**
 * Fields for a state proof transaction.
 *
 * Definition:
 * data/transactions/stateproof.go : StateProofTxnFields
 */
export type TransactionStateProof = { "state-proof-type"?: bigint; "state-proof"?: StateProofFields; message?: IndexerStateProofMessage };
