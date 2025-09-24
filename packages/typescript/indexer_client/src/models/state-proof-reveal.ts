import type { StateProofParticipant, StateProofSigSlot } from './index'

export type StateProofReveal = {
  /**
   * The position in the signature and participants arrays corresponding to this entry.
   */
  position?: bigint
  sigSlot?: StateProofSigSlot
  participant?: StateProofParticipant
}
