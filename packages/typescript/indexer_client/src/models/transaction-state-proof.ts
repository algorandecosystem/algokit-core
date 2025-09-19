import type { IndexerStateProofMessage, StateProofFields } from "./index";

/**
 * Fields for a state proof transaction.
 *
 * Definition:
 * data/transactions/stateproof.go : StateProofTxnFields
 */
export type TransactionStateProof = {
  /**
   * \[sptype\] Type of the state proof. Integer representing an entry defined in protocol/stateproof.go
   */
  stateProofType?: bigint;
  stateProof?: StateProofFields;
  message?: IndexerStateProofMessage;
};
