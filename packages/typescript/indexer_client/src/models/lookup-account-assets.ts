import type { AssetHolding } from './index'

export type LookupAccountAssets = {
  /**
   * Round at which the results were computed.
   */
  currentRound: bigint

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
  assets: AssetHolding[]
}
