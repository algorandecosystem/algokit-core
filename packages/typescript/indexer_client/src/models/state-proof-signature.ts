import type { MerkleArrayProof } from './index'

export type StateProofSignature = {
  falconSignature?: Uint8Array
  merkleArrayIndex?: bigint
  proof?: MerkleArrayProof

  /**
   * \[vkey\]
   */
  verifyingKey?: Uint8Array
}
