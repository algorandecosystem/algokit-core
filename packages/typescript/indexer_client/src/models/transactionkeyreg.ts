/**
 * Fields for a keyreg transaction.
 *
 * Definition:
 * data/transactions/keyreg.go : KeyregTxnFields
 */
export type TransactionKeyreg = {
  nonParticipation?: boolean;
  selectionParticipationKey?: string;
  voteFirstValid?: bigint;
  voteKeyDilution?: bigint;
  voteLastValid?: bigint;
  voteParticipationKey?: string;
  stateProofKey?: string;
};
