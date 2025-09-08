/**
 * AccountParticipation describes the parameters used by this account in consensus protocol.
 */
export type AccountParticipation = {
  selectionParticipationKey: string;
  voteFirstValid: bigint;
  voteKeyDilution: bigint;
  voteLastValid: bigint;
  voteParticipationKey: string;
  stateProofKey?: string;
};
