/**
 * Fields for a keyreg transaction.
 *
 * Definition:
 * data/transactions/keyreg.go : KeyregTxnFields
 */
export type TransactionKeyreg = {
  "non-participation"?: boolean;
  "selection-participation-key"?: string;
  "vote-first-valid"?: bigint;
  "vote-key-dilution"?: bigint;
  "vote-last-valid"?: bigint;
  "vote-participation-key"?: string;
  "state-proof-key"?: string;
};
