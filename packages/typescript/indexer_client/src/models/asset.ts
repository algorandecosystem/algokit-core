import type { AssetParams } from './index'
/**
 * Specifies both the unique identifier and the parameters for an asset
 */
export type Asset = {
  /**
   * unique asset identifier
   */
  index: bigint

  /**
   * Whether or not this asset is currently deleted.
   */
  deleted?: boolean

  /**
   * Round during which this asset was created.
   */
  createdAtRound?: bigint

  /**
   * Round during which this asset was destroyed.
   */
  destroyedAtRound?: bigint
  params: AssetParams
}
