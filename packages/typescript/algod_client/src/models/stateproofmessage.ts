/**
 * Represents the message that the state proofs are attesting to.
 */
export type StateProofMessage = {
  blockHeadersCommitment: string;
  votersCommitment: string;
  lnProvenWeight: bigint;
  firstAttestedRound: bigint;
  lastAttestedRound: bigint;
};
