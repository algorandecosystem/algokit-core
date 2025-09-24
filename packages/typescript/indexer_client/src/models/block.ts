import type { BlockRewards, BlockUpgradeState, BlockUpgradeVote, ParticipationUpdates, StateProofTracking, Transaction } from './index'
/**
 * Block information.
 *
 * Definition:
 * data/bookkeeping/block.go : Block
 */
export type Block = {
  /**
   * the proposer of this block.
   */
  proposer?: string

  /**
   * the sum of all fees paid by transactions in this block.
   */
  feesCollected?: bigint

  /**
   * the potential bonus payout for this block.
   */
  bonus?: bigint

  /**
   * the actual amount transferred to the proposer from the fee sink.
   */
  proposerPayout?: bigint

  /**
   * \[gh\] hash to which this block belongs.
   */
  genesisHash: string

  /**
   * \[gen\] ID to which this block belongs.
   */
  genesisId: string

  /**
   * \[prev\] Previous block hash.
   */
  previousBlockHash: string

  /**
   * \[prev512\] Previous block hash, using SHA-512.
   */
  previousBlockHash512?: string
  rewards?: BlockRewards

  /**
   * \[rnd\] Current round on which this block was appended to the chain.
   */
  round: bigint

  /**
   * \[seed\] Sortition seed.
   */
  seed: string

  /**
   * Tracks the status of state proofs.
   */
  stateProofTracking?: StateProofTracking[]

  /**
   * \[ts\] Block creation timestamp in seconds since epoch
   */
  timestamp: bigint

  /**
   * \[txns\] list of transactions corresponding to a given round.
   */
  transactions?: Transaction[]

  /**
   * \[txn\] TransactionsRoot authenticates the set of transactions appearing in the block. More specifically, it's the root of a merkle tree whose leaves are the block's Txids, in lexicographic order. For the empty block, it's 0. Note that the TxnRoot does not authenticate the signatures on the transactions, only the transactions themselves. Two blocks with the same transactions but in a different order and with different signatures will have the same TxnRoot.
   */
  transactionsRoot: string

  /**
   * \[txn256\] TransactionsRootSHA256 is an auxiliary TransactionRoot, built using a vector commitment instead of a merkle tree, and SHA256 hash function instead of the default SHA512_256. This commitment can be used on environments where only the SHA256 function exists.
   */
  transactionsRootSha256: string

  /**
   * \[txn512\] TransactionsRootSHA512 is an auxiliary TransactionRoot, built using a vector commitment instead of a merkle tree, and SHA512 hash function instead of the default SHA512_256.
   */
  transactionsRootSha512?: string

  /**
   * \[tc\] TxnCounter counts the number of transactions committed in the ledger, from the time at which support for this feature was introduced.
   *
   * Specifically, TxnCounter is the number of the next transaction that will be committed after this block.  It is 0 when no transactions have ever been committed (since TxnCounter started being supported).
   */
  txnCounter?: bigint
  upgradeState?: BlockUpgradeState
  upgradeVote?: BlockUpgradeVote
  participationUpdates?: ParticipationUpdates
}
