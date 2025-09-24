/**
 * Represents the message that the state proofs are attesting to.
 */
export type StateProofMessage = {
  /**
   * The vector commitment root on all light block headers within a state proof interval.
   */
  blockHeadersCommitment: Uint8Array

  /**
   * The vector commitment root of the top N accounts to sign the next StateProof.
   */
  votersCommitment: Uint8Array

  /**
   * An integer value representing the natural log of the proven weight with 16 bits of precision. This value would be used to verify the next state proof.
   */
  lnProvenWeight: bigint

  /**
   * The first round the message attests to.
   */
  firstAttestedRound: bigint

  /**
   * The last round the message attests to.
   */
  lastAttestedRound: bigint
}
