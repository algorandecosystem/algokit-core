/**
 * HoldingRef names a holding by referring to an Address and Asset it belongs to.
 */
export type HoldingRef = {
  /**
   * \[d\] Address in access list, or the sender of the transaction.
   */
  address: string

  /**
   * \[s\] Asset ID for asset in access list.
   */
  asset: bigint
}
