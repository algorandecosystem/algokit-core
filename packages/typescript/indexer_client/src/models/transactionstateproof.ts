import type { IndexerStateProofMessage, StateProofFields } from "./index";

/**
 * Fields for a state proof transaction.
 *
 * Definition:
 * data/transactions/stateproof.go : StateProofTxnFields
 */
export type TransactionStateProof = { stateProofType?: bigint; stateProof?: StateProofFields; message?: IndexerStateProofMessage };
