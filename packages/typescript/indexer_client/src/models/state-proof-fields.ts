import type { MerkleArrayProof, StateProofReveal } from './index'
/**
 * \[sp\] represents a state proof.
 *
 * Definition:
 * crypto/stateproof/structs.go : StateProof
 */
export type StateProofFields = {
  /**
   * \[c\]
   */
  sigCommit?: Uint8Array

  /**
   * \[w\]
   */
  signedWeight?: bigint
  sigProofs?: MerkleArrayProof
  partProofs?: MerkleArrayProof

  /**
   * \[v\] Salt version of the merkle signature.
   */
  saltVersion?: bigint

  /**
   * \[r\] Note that this is actually stored as a map[uint64] - Reveal in the actual msgp
   */
  reveals?: StateProofReveal[]

  /**
   * \[pr\] Sequence of reveal positions.
   */
  positionsToReveal?: bigint[]
}
