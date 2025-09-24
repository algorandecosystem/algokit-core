import type { HashFactory } from './index'

export type MerkleArrayProof = {
  /**
   * \[pth\]
   */
  path?: string[]
  hashFactory?: HashFactory

  /**
   * \[td\]
   */
  treeDepth?: bigint
}
