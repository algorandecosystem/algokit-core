import type { MerkleArrayProof, StateProofReveal } from "./index";

/**
 * \[sp\] represents a state proof.
 *
 * Definition:
 * crypto/stateproof/structs.go : StateProof
 */
export type StateProofFields = {
  sigCommit?: string;
  signedWeight?: bigint;
  sigProofs?: MerkleArrayProof;
  partProofs?: MerkleArrayProof;
  saltVersion?: bigint;
  reveals?: StateProofReveal[];
  positionsToReveal?: bigint[];
};
