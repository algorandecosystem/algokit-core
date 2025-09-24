import type { SignedTransaction } from '@algorandfoundation/algokit-transact'
/**
 * A transaction group to simulate.
 */
export type SimulateRequestTransactionGroup = {
  /**
   * An atomic transaction group.
   */
  txns: SignedTransaction[]
}
