import type { StateProofSignature } from './index'

export type StateProofSigSlot = {
  signature?: StateProofSignature

  /**
   * \[l\] The total weight of signatures in the lower-numbered slots.
   */
  lowerSigWeight?: bigint
}
