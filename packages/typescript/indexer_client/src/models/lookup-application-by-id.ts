import type { Application } from './index'

export type LookupApplicationById = {
  application?: Application

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint
}
