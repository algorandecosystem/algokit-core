import type { HbProofFields } from './index'
/**
 * Fields for a heartbeat transaction.
 *
 * Definition:
 * data/transactions/heartbeat.go : HeartbeatTxnFields
 */
export type TransactionHeartbeat = {
  /**
   * \[hbad\] HbAddress is the account this txn is proving onlineness for.
   */
  hbAddress: string
  hbProof: HbProofFields

  /**
   * \[hbsd\] HbSeed must be the block seed for the this transaction's firstValid block.
   */
  hbSeed: Uint8Array

  /**
   * \[hbvid\] HbVoteID must match the HbAddress account's current VoteID.
   */
  hbVoteId: Uint8Array

  /**
   * \[hbkd\] HbKeyDilution must match HbAddress account's current KeyDilution.
   */
  hbKeyDilution: bigint
}
