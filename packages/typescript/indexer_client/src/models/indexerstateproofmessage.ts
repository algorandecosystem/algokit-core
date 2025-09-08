export type IndexerStateProofMessage = {
  blockHeadersCommitment?: string;
  votersCommitment?: string;
  lnProvenWeight?: bigint;
  firstAttestedRound?: bigint;
  latestAttestedRound?: bigint;
};
