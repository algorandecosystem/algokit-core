import type { SignedTransaction } from '@algorandfoundation/algokit-transact'
/**
 * PendingTransactions is an array of signed transactions exactly as they were submitted.
 */
export type GetPendingTransactions = {
  /**
   * An array of signed transaction objects.
   */
  topTransactions: SignedTransaction[]

  /**
   * Total number of transactions in the pool.
   */
  totalTransactions: bigint
}
