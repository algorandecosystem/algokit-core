export type StateProofVerifier = {
  /**
   * \[cmt\] Represents the root of the vector commitment tree.
   */
  commitment?: Uint8Array

  /**
   * \[lf\] Key lifetime.
   */
  keyLifetime?: bigint
}
