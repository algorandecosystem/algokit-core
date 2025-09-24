import type { HashFactory } from './index'

export type MerkleArrayProof = {
  /**
   * \[pth\]
   */
  path?: Uint8Array[]
  hashFactory?: HashFactory

  /**
   * \[td\]
   */
  treeDepth?: bigint
}
