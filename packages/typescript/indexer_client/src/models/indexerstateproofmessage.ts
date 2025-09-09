export type IndexerStateProofMessage = {
  /**
   * \[b\]
   */
  blockHeadersCommitment?: string;

  /**
   * \[v\]
   */
  votersCommitment?: string;

  /**
   * \[P\]
   */
  lnProvenWeight?: bigint;

  /**
   * \[f\]
   */
  firstAttestedRound?: bigint;

  /**
   * \[l\]
   */
  latestAttestedRound?: bigint;
};
