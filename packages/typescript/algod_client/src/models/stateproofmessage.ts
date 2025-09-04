/**
 * Represents the message that the state proofs are attesting to.
 */
export type StateProofMessage = {
  BlockHeadersCommitment: string;
  VotersCommitment: string;
  LnProvenWeight: bigint;
  FirstAttestedRound: bigint;
  LastAttestedRound: bigint;
};
