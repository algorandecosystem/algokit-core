import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { BlockRewards, BlockRewardsDto } from './block-rewards'
import { BlockRewards as BlockRewardsModel } from './block-rewards'
import type { BlockUpgradeState, BlockUpgradeStateDto } from './block-upgrade-state'
import { BlockUpgradeState as BlockUpgradeStateModel } from './block-upgrade-state'
import type { BlockUpgradeVote, BlockUpgradeVoteDto } from './block-upgrade-vote'
import { BlockUpgradeVote as BlockUpgradeVoteModel } from './block-upgrade-vote'
import type { ParticipationUpdates, ParticipationUpdatesDto } from './participation-updates'
import { ParticipationUpdates as ParticipationUpdatesModel } from './participation-updates'
import type { StateProofTracking, StateProofTrackingDto } from './state-proof-tracking'
import { StateProofTracking as StateProofTrackingModel } from './state-proof-tracking'
import type { Transaction, TransactionDto } from './transaction'
import { Transaction as TransactionModel } from './transaction'

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
  genesisHash: Uint8Array

  /**
   * \[gen\] ID to which this block belongs.
   */
  genesisId: string

  /**
   * \[prev\] Previous block hash.
   */
  previousBlockHash: Uint8Array

  /**
   * \[prev512\] Previous block hash, using SHA-512.
   */
  previousBlockHash512?: Uint8Array
  rewards?: BlockRewards

  /**
   * \[rnd\] Current round on which this block was appended to the chain.
   */
  round: bigint

  /**
   * \[seed\] Sortition seed.
   */
  seed: Uint8Array

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
  transactionsRoot: Uint8Array

  /**
   * \[txn256\] TransactionsRootSHA256 is an auxiliary TransactionRoot, built using a vector commitment instead of a merkle tree, and SHA256 hash function instead of the default SHA512_256. This commitment can be used on environments where only the SHA256 function exists.
   */
  transactionsRootSha256: Uint8Array

  /**
   * \[txn512\] TransactionsRootSHA512 is an auxiliary TransactionRoot, built using a vector commitment instead of a merkle tree, and SHA512 hash function instead of the default SHA512_256.
   */
  transactionsRootSha512?: Uint8Array

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

// JSON DTO shape for Block with wire keys and JSON-safe primitives
export type BlockDto = {
  proposer?: string
  'fees-collected'?: bigint
  bonus?: bigint
  'proposer-payout'?: bigint
  'genesis-hash': string
  'genesis-id': string
  'previous-block-hash': string
  'previous-block-hash-512'?: string
  rewards?: BlockRewardsDto
  round: string
  seed: string
  'state-proof-tracking'?: StateProofTrackingDto[]
  timestamp: bigint
  transactions?: TransactionDto[]
  'transactions-root': string
  'transactions-root-sha256': string
  'transactions-root-sha512'?: string
  'txn-counter'?: bigint
  'upgrade-state'?: BlockUpgradeStateDto
  'upgrade-vote'?: BlockUpgradeVoteDto
  'participation-updates'?: ParticipationUpdatesDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: Block): BlockDto {
  const out: any = {}
  {
    const v = (value as any)['proposer']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proposer'] = v
    }
  }
  {
    const v = (value as any)['feesCollected']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fees-collected'] = v
    }
  }
  {
    const v = (value as any)['bonus']
    if (v === undefined) {
      // omit undefined
    } else {
      out['bonus'] = v
    }
  }
  {
    const v = (value as any)['proposerPayout']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proposer-payout'] = v
    }
  }
  {
    const v = (value as any)['genesisHash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesis-hash'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['genesisId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesis-id'] = v
    }
  }
  {
    const v = (value as any)['previousBlockHash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['previous-block-hash'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['previousBlockHash512']
    if (v === undefined) {
      // omit undefined
    } else {
      out['previous-block-hash-512'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewards'] = v === undefined ? v : BlockRewardsModel.toDto(v)
    }
  }
  {
    const v = (value as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['seed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['seed'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['stateProofTracking']
    if (v === undefined) {
      // omit undefined
    } else {
      out['state-proof-tracking'] = (v as any[]).map((item) => StateProofTrackingModel.toDto(item))
    }
  }
  {
    const v = (value as any)['timestamp']
    if (v === undefined) {
      // omit undefined
    } else {
      out['timestamp'] = v
    }
  }
  {
    const v = (value as any)['transactions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transactions'] = (v as any[]).map((item) => TransactionModel.toDto(item))
    }
  }
  {
    const v = (value as any)['transactionsRoot']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transactions-root'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['transactionsRootSha256']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transactions-root-sha256'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['transactionsRootSha512']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transactions-root-sha512'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['txnCounter']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txn-counter'] = v
    }
  }
  {
    const v = (value as any)['upgradeState']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-state'] = v === undefined ? v : BlockUpgradeStateModel.toDto(v)
    }
  }
  {
    const v = (value as any)['upgradeVote']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-vote'] = v === undefined ? v : BlockUpgradeVoteModel.toDto(v)
    }
  }
  {
    const v = (value as any)['participationUpdates']
    if (v === undefined) {
      // omit undefined
    } else {
      out['participation-updates'] = v === undefined ? v : ParticipationUpdatesModel.toDto(v)
    }
  }
  return out as BlockDto
}

export function fromDto(dto: BlockDto): Block {
  const out: any = {}
  {
    const v = (dto as any)['proposer']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proposer'] = v as any
    }
  }
  {
    const v = (dto as any)['fees-collected']
    if (v === undefined) {
      // omit undefined
    } else {
      out['feesCollected'] = v as any
    }
  }
  {
    const v = (dto as any)['bonus']
    if (v === undefined) {
      // omit undefined
    } else {
      out['bonus'] = v as any
    }
  }
  {
    const v = (dto as any)['proposer-payout']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proposerPayout'] = v as any
    }
  }
  {
    const v = (dto as any)['genesis-hash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesisHash'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['genesis-id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesisId'] = v as any
    }
  }
  {
    const v = (dto as any)['previous-block-hash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['previousBlockHash'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['previous-block-hash-512']
    if (v === undefined) {
      // omit undefined
    } else {
      out['previousBlockHash512'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewards'] = v === undefined ? v : BlockRewardsModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['seed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['seed'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['state-proof-tracking']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stateProofTracking'] = (v as any[]).map((item) => StateProofTrackingModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['timestamp']
    if (v === undefined) {
      // omit undefined
    } else {
      out['timestamp'] = v as any
    }
  }
  {
    const v = (dto as any)['transactions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transactions'] = (v as any[]).map((item) => TransactionModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['transactions-root']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transactionsRoot'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['transactions-root-sha256']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transactionsRootSha256'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['transactions-root-sha512']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transactionsRootSha512'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['txn-counter']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txnCounter'] = v as any
    }
  }
  {
    const v = (dto as any)['upgrade-state']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeState'] = v === undefined ? v : BlockUpgradeStateModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['upgrade-vote']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeVote'] = v === undefined ? v : BlockUpgradeVoteModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['participation-updates']
    if (v === undefined) {
      // omit undefined
    } else {
      out['participationUpdates'] = v === undefined ? v : ParticipationUpdatesModel.fromDto(v)
    }
  }
  return out as Block
}

// Msgpack codecs
export function encodeMsgpack(value: Block): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): Block {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: Block): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): Block {
  return fromDto(raw as BlockDto)
}

// Array helpers
export function encodeMsgpackArray(values: Block[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): Block[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: Block[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): Block[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type BlockMsgpackDto = {
  proposer?: string
  'fees-collected'?: bigint
  bonus?: bigint
  'proposer-payout'?: bigint
  'genesis-hash': Uint8Array
  'genesis-id': string
  'previous-block-hash': Uint8Array
  'previous-block-hash-512'?: Uint8Array
  rewards?: ReturnType<(typeof BlockRewardsModel)['toMsgpackDto']>
  round: bigint
  seed: Uint8Array
  'state-proof-tracking'?: ReturnType<(typeof StateProofTrackingModel)['toMsgpackDto']>[]
  timestamp: bigint
  transactions?: ReturnType<(typeof TransactionModel)['toMsgpackDto']>[]
  'transactions-root': Uint8Array
  'transactions-root-sha256': Uint8Array
  'transactions-root-sha512'?: Uint8Array
  'txn-counter'?: bigint
  'upgrade-state'?: ReturnType<(typeof BlockUpgradeStateModel)['toMsgpackDto']>
  'upgrade-vote'?: ReturnType<(typeof BlockUpgradeVoteModel)['toMsgpackDto']>
  'participation-updates'?: ReturnType<(typeof ParticipationUpdatesModel)['toMsgpackDto']>
}

function toMsgpackDto(value: Block): BlockMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['proposer']
    if (v === undefined) {
    } else {
      out['proposer'] = v
    }
  }
  {
    const v = (value as any)['feesCollected']
    if (v === undefined) {
    } else {
      out['fees-collected'] = v
    }
  }
  {
    const v = (value as any)['bonus']
    if (v === undefined) {
    } else {
      out['bonus'] = v
    }
  }
  {
    const v = (value as any)['proposerPayout']
    if (v === undefined) {
    } else {
      out['proposer-payout'] = v
    }
  }
  {
    const v = (value as any)['genesisHash']
    if (v === undefined) {
    } else {
      out['genesis-hash'] = v
    }
  }
  {
    const v = (value as any)['genesisId']
    if (v === undefined) {
    } else {
      out['genesis-id'] = v
    }
  }
  {
    const v = (value as any)['previousBlockHash']
    if (v === undefined) {
    } else {
      out['previous-block-hash'] = v
    }
  }
  {
    const v = (value as any)['previousBlockHash512']
    if (v === undefined) {
    } else {
      out['previous-block-hash-512'] = v
    }
  }
  {
    const v = (value as any)['rewards']
    if (v === undefined) {
    } else {
      out['rewards'] = BlockRewardsModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (value as any)['seed']
    if (v === undefined) {
    } else {
      out['seed'] = v
    }
  }
  {
    const v = (value as any)['stateProofTracking']
    if (v === undefined) {
    } else {
      out['state-proof-tracking'] = (v as any[]).map((item) => StateProofTrackingModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['timestamp']
    if (v === undefined) {
    } else {
      out['timestamp'] = v
    }
  }
  {
    const v = (value as any)['transactions']
    if (v === undefined) {
    } else {
      out['transactions'] = (v as any[]).map((item) => TransactionModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['transactionsRoot']
    if (v === undefined) {
    } else {
      out['transactions-root'] = v
    }
  }
  {
    const v = (value as any)['transactionsRootSha256']
    if (v === undefined) {
    } else {
      out['transactions-root-sha256'] = v
    }
  }
  {
    const v = (value as any)['transactionsRootSha512']
    if (v === undefined) {
    } else {
      out['transactions-root-sha512'] = v
    }
  }
  {
    const v = (value as any)['txnCounter']
    if (v === undefined) {
    } else {
      out['txn-counter'] = v
    }
  }
  {
    const v = (value as any)['upgradeState']
    if (v === undefined) {
    } else {
      out['upgrade-state'] = BlockUpgradeStateModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['upgradeVote']
    if (v === undefined) {
    } else {
      out['upgrade-vote'] = BlockUpgradeVoteModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['participationUpdates']
    if (v === undefined) {
    } else {
      out['participation-updates'] = ParticipationUpdatesModel.toMsgpackDto(v)
    }
  }
  return out as BlockMsgpackDto
}

function fromMsgpackDto(dto: BlockMsgpackDto): Block {
  const out: any = {}
  {
    const v = (dto as any)['proposer']
    if (v === undefined) {
    } else {
      out['proposer'] = v
    }
  }
  {
    const v = (dto as any)['fees-collected']
    if (v === undefined) {
    } else {
      out['feesCollected'] = v
    }
  }
  {
    const v = (dto as any)['bonus']
    if (v === undefined) {
    } else {
      out['bonus'] = v
    }
  }
  {
    const v = (dto as any)['proposer-payout']
    if (v === undefined) {
    } else {
      out['proposerPayout'] = v
    }
  }
  {
    const v = (dto as any)['genesis-hash']
    if (v === undefined) {
    } else {
      out['genesisHash'] = v
    }
  }
  {
    const v = (dto as any)['genesis-id']
    if (v === undefined) {
    } else {
      out['genesisId'] = v
    }
  }
  {
    const v = (dto as any)['previous-block-hash']
    if (v === undefined) {
    } else {
      out['previousBlockHash'] = v
    }
  }
  {
    const v = (dto as any)['previous-block-hash-512']
    if (v === undefined) {
    } else {
      out['previousBlockHash512'] = v
    }
  }
  {
    const v = (dto as any)['rewards']
    if (v === undefined) {
    } else {
      out['rewards'] = BlockRewardsModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (dto as any)['seed']
    if (v === undefined) {
    } else {
      out['seed'] = v
    }
  }
  {
    const v = (dto as any)['state-proof-tracking']
    if (v === undefined) {
    } else {
      out['stateProofTracking'] = (v as any[]).map((item) => StateProofTrackingModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['timestamp']
    if (v === undefined) {
    } else {
      out['timestamp'] = v
    }
  }
  {
    const v = (dto as any)['transactions']
    if (v === undefined) {
    } else {
      out['transactions'] = (v as any[]).map((item) => TransactionModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['transactions-root']
    if (v === undefined) {
    } else {
      out['transactionsRoot'] = v
    }
  }
  {
    const v = (dto as any)['transactions-root-sha256']
    if (v === undefined) {
    } else {
      out['transactionsRootSha256'] = v
    }
  }
  {
    const v = (dto as any)['transactions-root-sha512']
    if (v === undefined) {
    } else {
      out['transactionsRootSha512'] = v
    }
  }
  {
    const v = (dto as any)['txn-counter']
    if (v === undefined) {
    } else {
      out['txnCounter'] = v
    }
  }
  {
    const v = (dto as any)['upgrade-state']
    if (v === undefined) {
    } else {
      out['upgradeState'] = BlockUpgradeStateModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['upgrade-vote']
    if (v === undefined) {
    } else {
      out['upgradeVote'] = BlockUpgradeVoteModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['participation-updates']
    if (v === undefined) {
    } else {
      out['participationUpdates'] = ParticipationUpdatesModel.fromMsgpackDto(v)
    }
  }
  return out as Block
}

export const Block = {
  toDto,
  fromDto,
  encodeMsgpack,
  decodeMsgpack,
  encodeJson,
  decodeJson,
  toMsgpackDto,
  fromMsgpackDto,
  encodeMsgpackArray,
  decodeMsgpackArray,
  encodeJsonArray,
  decodeJsonArray,
} as const
