export type StateProofVerifier = {
  /**
   * \[cmt\] Represents the root of the vector commitment tree.
   */
  commitment?: string;

  /**
   * \[lf\] Key lifetime.
   */
  keyLifetime?: bigint;
};
