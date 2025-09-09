import type { StateProofVerifier } from "./index";

export type StateProofParticipant = {
  verifier?: StateProofVerifier;

  /**
   * \[w\]
   */
  weight?: bigint;
};
