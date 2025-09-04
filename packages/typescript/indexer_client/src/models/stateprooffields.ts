import type { MerkleArrayProof, StateProofReveal } from "./index";

/**
 * \[sp\] represents a state proof.
 *
 * Definition:
 * crypto/stateproof/structs.go : StateProof
 */
export type StateProofFields = {
  "sig-commit"?: string;
  "signed-weight"?: bigint;
  "sig-proofs"?: MerkleArrayProof;
  "part-proofs"?: MerkleArrayProof;
  "salt-version"?: number;
  reveals?: StateProofReveal[];
  "positions-to-reveal"?: bigint[];
};
