export type IndexerStateProofMessage = {
  /**
   * \[b\]
   */
  blockHeadersCommitment?: Uint8Array

  /**
   * \[v\]
   */
  votersCommitment?: Uint8Array

  /**
   * \[P\]
   */
  lnProvenWeight?: bigint

  /**
   * \[f\]
   */
  firstAttestedRound?: bigint

  /**
   * \[l\]
   */
  latestAttestedRound?: bigint
}
