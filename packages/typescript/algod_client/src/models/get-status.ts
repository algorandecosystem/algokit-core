import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * NodeStatus contains the information about a node status
 */
export type GetStatus = {
  /**
   * CatchupTime in nanoseconds
   */
  catchupTime: bigint

  /**
   * LastRound indicates the last round seen
   */
  lastRound: bigint

  /**
   * LastVersion indicates the last consensus version supported
   */
  lastVersion: string

  /**
   * NextVersion of consensus protocol to use
   */
  nextVersion: string

  /**
   * NextVersionRound is the round at which the next consensus version will apply
   */
  nextVersionRound: bigint

  /**
   * NextVersionSupported indicates whether the next consensus version is supported by this node
   */
  nextVersionSupported: boolean

  /**
   * StoppedAtUnsupportedRound indicates that the node does not support the new rounds and has stopped making progress
   */
  stoppedAtUnsupportedRound: boolean

  /**
   * TimeSinceLastRound in nanoseconds
   */
  timeSinceLastRound: bigint

  /**
   * The last catchpoint seen by the node
   */
  lastCatchpoint?: string

  /**
   * The current catchpoint that is being caught up to
   */
  catchpoint?: string

  /**
   * The total number of accounts included in the current catchpoint
   */
  catchpointTotalAccounts?: bigint

  /**
   * The number of accounts from the current catchpoint that have been processed so far as part of the catchup
   */
  catchpointProcessedAccounts?: bigint

  /**
   * The number of accounts from the current catchpoint that have been verified so far as part of the catchup
   */
  catchpointVerifiedAccounts?: bigint

  /**
   * The total number of key-values (KVs) included in the current catchpoint
   */
  catchpointTotalKvs?: bigint

  /**
   * The number of key-values (KVs) from the current catchpoint that have been processed so far as part of the catchup
   */
  catchpointProcessedKvs?: bigint

  /**
   * The number of key-values (KVs) from the current catchpoint that have been verified so far as part of the catchup
   */
  catchpointVerifiedKvs?: bigint

  /**
   * The total number of blocks that are required to complete the current catchpoint catchup
   */
  catchpointTotalBlocks?: bigint

  /**
   * The number of blocks that have already been obtained by the node as part of the catchup
   */
  catchpointAcquiredBlocks?: bigint

  /**
   * Upgrade delay
   */
  upgradeDelay?: bigint

  /**
   * This node's upgrade vote
   */
  upgradeNodeVote?: boolean

  /**
   * Yes votes required for consensus upgrade
   */
  upgradeVotesRequired?: bigint

  /**
   * Total votes cast for consensus upgrade
   */
  upgradeVotes?: bigint

  /**
   * Yes votes cast for consensus upgrade
   */
  upgradeYesVotes?: bigint

  /**
   * No votes cast for consensus upgrade
   */
  upgradeNoVotes?: bigint

  /**
   * Next protocol round
   */
  upgradeNextProtocolVoteBefore?: bigint

  /**
   * Total voting rounds for current upgrade
   */
  upgradeVoteRounds?: bigint
}

// JSON DTO shape for GetStatus with wire keys and JSON-safe primitives
export type GetStatusDto = {
  'catchup-time': string
  'last-round': string
  'last-version': string
  'next-version': string
  'next-version-round': bigint
  'next-version-supported': boolean
  'stopped-at-unsupported-round': boolean
  'time-since-last-round': string
  'last-catchpoint'?: string
  catchpoint?: string
  'catchpoint-total-accounts'?: string
  'catchpoint-processed-accounts'?: string
  'catchpoint-verified-accounts'?: string
  'catchpoint-total-kvs'?: string
  'catchpoint-processed-kvs'?: string
  'catchpoint-verified-kvs'?: string
  'catchpoint-total-blocks'?: string
  'catchpoint-acquired-blocks'?: string
  'upgrade-delay'?: bigint
  'upgrade-node-vote'?: boolean
  'upgrade-votes-required'?: bigint
  'upgrade-votes'?: bigint
  'upgrade-yes-votes'?: bigint
  'upgrade-no-votes'?: bigint
  'upgrade-next-protocol-vote-before'?: bigint
  'upgrade-vote-rounds'?: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetStatus): GetStatusDto {
  const out: any = {}
  {
    const v = (value as any)['catchupTime']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchup-time'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['lastRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['lastVersion']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-version'] = v
    }
  }
  {
    const v = (value as any)['nextVersion']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-version'] = v
    }
  }
  {
    const v = (value as any)['nextVersionRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-version-round'] = v
    }
  }
  {
    const v = (value as any)['nextVersionSupported']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-version-supported'] = v
    }
  }
  {
    const v = (value as any)['stoppedAtUnsupportedRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stopped-at-unsupported-round'] = v
    }
  }
  {
    const v = (value as any)['timeSinceLastRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['time-since-last-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['lastCatchpoint']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-catchpoint'] = v
    }
  }
  {
    const v = (value as any)['catchpoint']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpoint'] = v
    }
  }
  {
    const v = (value as any)['catchpointTotalAccounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpoint-total-accounts'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['catchpointProcessedAccounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpoint-processed-accounts'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['catchpointVerifiedAccounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpoint-verified-accounts'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['catchpointTotalKvs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpoint-total-kvs'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['catchpointProcessedKvs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpoint-processed-kvs'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['catchpointVerifiedKvs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpoint-verified-kvs'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['catchpointTotalBlocks']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpoint-total-blocks'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['catchpointAcquiredBlocks']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpoint-acquired-blocks'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['upgradeDelay']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-delay'] = v
    }
  }
  {
    const v = (value as any)['upgradeNodeVote']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-node-vote'] = v
    }
  }
  {
    const v = (value as any)['upgradeVotesRequired']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-votes-required'] = v
    }
  }
  {
    const v = (value as any)['upgradeVotes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-votes'] = v
    }
  }
  {
    const v = (value as any)['upgradeYesVotes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-yes-votes'] = v
    }
  }
  {
    const v = (value as any)['upgradeNoVotes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-no-votes'] = v
    }
  }
  {
    const v = (value as any)['upgradeNextProtocolVoteBefore']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-next-protocol-vote-before'] = v
    }
  }
  {
    const v = (value as any)['upgradeVoteRounds']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-vote-rounds'] = v
    }
  }
  return out as GetStatusDto
}

export function fromDto(dto: GetStatusDto): GetStatus {
  const out: any = {}
  {
    const v = (dto as any)['catchup-time']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchupTime'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['last-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['last-version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastVersion'] = v as any
    }
  }
  {
    const v = (dto as any)['next-version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextVersion'] = v as any
    }
  }
  {
    const v = (dto as any)['next-version-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextVersionRound'] = v as any
    }
  }
  {
    const v = (dto as any)['next-version-supported']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextVersionSupported'] = v as any
    }
  }
  {
    const v = (dto as any)['stopped-at-unsupported-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stoppedAtUnsupportedRound'] = v as any
    }
  }
  {
    const v = (dto as any)['time-since-last-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['timeSinceLastRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['last-catchpoint']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastCatchpoint'] = v as any
    }
  }
  {
    const v = (dto as any)['catchpoint']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpoint'] = v as any
    }
  }
  {
    const v = (dto as any)['catchpoint-total-accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpointTotalAccounts'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['catchpoint-processed-accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpointProcessedAccounts'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['catchpoint-verified-accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpointVerifiedAccounts'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['catchpoint-total-kvs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpointTotalKvs'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['catchpoint-processed-kvs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpointProcessedKvs'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['catchpoint-verified-kvs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpointVerifiedKvs'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['catchpoint-total-blocks']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpointTotalBlocks'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['catchpoint-acquired-blocks']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchpointAcquiredBlocks'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['upgrade-delay']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeDelay'] = v as any
    }
  }
  {
    const v = (dto as any)['upgrade-node-vote']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeNodeVote'] = v as any
    }
  }
  {
    const v = (dto as any)['upgrade-votes-required']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeVotesRequired'] = v as any
    }
  }
  {
    const v = (dto as any)['upgrade-votes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeVotes'] = v as any
    }
  }
  {
    const v = (dto as any)['upgrade-yes-votes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeYesVotes'] = v as any
    }
  }
  {
    const v = (dto as any)['upgrade-no-votes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeNoVotes'] = v as any
    }
  }
  {
    const v = (dto as any)['upgrade-next-protocol-vote-before']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeNextProtocolVoteBefore'] = v as any
    }
  }
  {
    const v = (dto as any)['upgrade-vote-rounds']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeVoteRounds'] = v as any
    }
  }
  return out as GetStatus
}

// Msgpack codecs
export function encodeMsgpack(value: GetStatus): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetStatus {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetStatus): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetStatus {
  return fromDto(raw as GetStatusDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetStatus[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetStatus[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetStatus[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetStatus[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetStatusMsgpackDto = {
  'catchup-time': bigint
  'last-round': bigint
  'last-version': string
  'next-version': string
  'next-version-round': bigint
  'next-version-supported': boolean
  'stopped-at-unsupported-round': boolean
  'time-since-last-round': bigint
  'last-catchpoint'?: string
  catchpoint?: string
  'catchpoint-total-accounts'?: bigint
  'catchpoint-processed-accounts'?: bigint
  'catchpoint-verified-accounts'?: bigint
  'catchpoint-total-kvs'?: bigint
  'catchpoint-processed-kvs'?: bigint
  'catchpoint-verified-kvs'?: bigint
  'catchpoint-total-blocks'?: bigint
  'catchpoint-acquired-blocks'?: bigint
  'upgrade-delay'?: bigint
  'upgrade-node-vote'?: boolean
  'upgrade-votes-required'?: bigint
  'upgrade-votes'?: bigint
  'upgrade-yes-votes'?: bigint
  'upgrade-no-votes'?: bigint
  'upgrade-next-protocol-vote-before'?: bigint
  'upgrade-vote-rounds'?: bigint
}

function toMsgpackDto(value: GetStatus): GetStatusMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['catchupTime']
    if (v === undefined) {
    } else {
      out['catchup-time'] = v
    }
  }
  {
    const v = (value as any)['lastRound']
    if (v === undefined) {
    } else {
      out['last-round'] = v
    }
  }
  {
    const v = (value as any)['lastVersion']
    if (v === undefined) {
    } else {
      out['last-version'] = v
    }
  }
  {
    const v = (value as any)['nextVersion']
    if (v === undefined) {
    } else {
      out['next-version'] = v
    }
  }
  {
    const v = (value as any)['nextVersionRound']
    if (v === undefined) {
    } else {
      out['next-version-round'] = v
    }
  }
  {
    const v = (value as any)['nextVersionSupported']
    if (v === undefined) {
    } else {
      out['next-version-supported'] = v
    }
  }
  {
    const v = (value as any)['stoppedAtUnsupportedRound']
    if (v === undefined) {
    } else {
      out['stopped-at-unsupported-round'] = v
    }
  }
  {
    const v = (value as any)['timeSinceLastRound']
    if (v === undefined) {
    } else {
      out['time-since-last-round'] = v
    }
  }
  {
    const v = (value as any)['lastCatchpoint']
    if (v === undefined) {
    } else {
      out['last-catchpoint'] = v
    }
  }
  {
    const v = (value as any)['catchpoint']
    if (v === undefined) {
    } else {
      out['catchpoint'] = v
    }
  }
  {
    const v = (value as any)['catchpointTotalAccounts']
    if (v === undefined) {
    } else {
      out['catchpoint-total-accounts'] = v
    }
  }
  {
    const v = (value as any)['catchpointProcessedAccounts']
    if (v === undefined) {
    } else {
      out['catchpoint-processed-accounts'] = v
    }
  }
  {
    const v = (value as any)['catchpointVerifiedAccounts']
    if (v === undefined) {
    } else {
      out['catchpoint-verified-accounts'] = v
    }
  }
  {
    const v = (value as any)['catchpointTotalKvs']
    if (v === undefined) {
    } else {
      out['catchpoint-total-kvs'] = v
    }
  }
  {
    const v = (value as any)['catchpointProcessedKvs']
    if (v === undefined) {
    } else {
      out['catchpoint-processed-kvs'] = v
    }
  }
  {
    const v = (value as any)['catchpointVerifiedKvs']
    if (v === undefined) {
    } else {
      out['catchpoint-verified-kvs'] = v
    }
  }
  {
    const v = (value as any)['catchpointTotalBlocks']
    if (v === undefined) {
    } else {
      out['catchpoint-total-blocks'] = v
    }
  }
  {
    const v = (value as any)['catchpointAcquiredBlocks']
    if (v === undefined) {
    } else {
      out['catchpoint-acquired-blocks'] = v
    }
  }
  {
    const v = (value as any)['upgradeDelay']
    if (v === undefined) {
    } else {
      out['upgrade-delay'] = v
    }
  }
  {
    const v = (value as any)['upgradeNodeVote']
    if (v === undefined) {
    } else {
      out['upgrade-node-vote'] = v
    }
  }
  {
    const v = (value as any)['upgradeVotesRequired']
    if (v === undefined) {
    } else {
      out['upgrade-votes-required'] = v
    }
  }
  {
    const v = (value as any)['upgradeVotes']
    if (v === undefined) {
    } else {
      out['upgrade-votes'] = v
    }
  }
  {
    const v = (value as any)['upgradeYesVotes']
    if (v === undefined) {
    } else {
      out['upgrade-yes-votes'] = v
    }
  }
  {
    const v = (value as any)['upgradeNoVotes']
    if (v === undefined) {
    } else {
      out['upgrade-no-votes'] = v
    }
  }
  {
    const v = (value as any)['upgradeNextProtocolVoteBefore']
    if (v === undefined) {
    } else {
      out['upgrade-next-protocol-vote-before'] = v
    }
  }
  {
    const v = (value as any)['upgradeVoteRounds']
    if (v === undefined) {
    } else {
      out['upgrade-vote-rounds'] = v
    }
  }
  return out as GetStatusMsgpackDto
}

function fromMsgpackDto(dto: GetStatusMsgpackDto): GetStatus {
  const out: any = {}
  {
    const v = (dto as any)['catchup-time']
    if (v === undefined) {
    } else {
      out['catchupTime'] = v
    }
  }
  {
    const v = (dto as any)['last-round']
    if (v === undefined) {
    } else {
      out['lastRound'] = v
    }
  }
  {
    const v = (dto as any)['last-version']
    if (v === undefined) {
    } else {
      out['lastVersion'] = v
    }
  }
  {
    const v = (dto as any)['next-version']
    if (v === undefined) {
    } else {
      out['nextVersion'] = v
    }
  }
  {
    const v = (dto as any)['next-version-round']
    if (v === undefined) {
    } else {
      out['nextVersionRound'] = v
    }
  }
  {
    const v = (dto as any)['next-version-supported']
    if (v === undefined) {
    } else {
      out['nextVersionSupported'] = v
    }
  }
  {
    const v = (dto as any)['stopped-at-unsupported-round']
    if (v === undefined) {
    } else {
      out['stoppedAtUnsupportedRound'] = v
    }
  }
  {
    const v = (dto as any)['time-since-last-round']
    if (v === undefined) {
    } else {
      out['timeSinceLastRound'] = v
    }
  }
  {
    const v = (dto as any)['last-catchpoint']
    if (v === undefined) {
    } else {
      out['lastCatchpoint'] = v
    }
  }
  {
    const v = (dto as any)['catchpoint']
    if (v === undefined) {
    } else {
      out['catchpoint'] = v
    }
  }
  {
    const v = (dto as any)['catchpoint-total-accounts']
    if (v === undefined) {
    } else {
      out['catchpointTotalAccounts'] = v
    }
  }
  {
    const v = (dto as any)['catchpoint-processed-accounts']
    if (v === undefined) {
    } else {
      out['catchpointProcessedAccounts'] = v
    }
  }
  {
    const v = (dto as any)['catchpoint-verified-accounts']
    if (v === undefined) {
    } else {
      out['catchpointVerifiedAccounts'] = v
    }
  }
  {
    const v = (dto as any)['catchpoint-total-kvs']
    if (v === undefined) {
    } else {
      out['catchpointTotalKvs'] = v
    }
  }
  {
    const v = (dto as any)['catchpoint-processed-kvs']
    if (v === undefined) {
    } else {
      out['catchpointProcessedKvs'] = v
    }
  }
  {
    const v = (dto as any)['catchpoint-verified-kvs']
    if (v === undefined) {
    } else {
      out['catchpointVerifiedKvs'] = v
    }
  }
  {
    const v = (dto as any)['catchpoint-total-blocks']
    if (v === undefined) {
    } else {
      out['catchpointTotalBlocks'] = v
    }
  }
  {
    const v = (dto as any)['catchpoint-acquired-blocks']
    if (v === undefined) {
    } else {
      out['catchpointAcquiredBlocks'] = v
    }
  }
  {
    const v = (dto as any)['upgrade-delay']
    if (v === undefined) {
    } else {
      out['upgradeDelay'] = v
    }
  }
  {
    const v = (dto as any)['upgrade-node-vote']
    if (v === undefined) {
    } else {
      out['upgradeNodeVote'] = v
    }
  }
  {
    const v = (dto as any)['upgrade-votes-required']
    if (v === undefined) {
    } else {
      out['upgradeVotesRequired'] = v
    }
  }
  {
    const v = (dto as any)['upgrade-votes']
    if (v === undefined) {
    } else {
      out['upgradeVotes'] = v
    }
  }
  {
    const v = (dto as any)['upgrade-yes-votes']
    if (v === undefined) {
    } else {
      out['upgradeYesVotes'] = v
    }
  }
  {
    const v = (dto as any)['upgrade-no-votes']
    if (v === undefined) {
    } else {
      out['upgradeNoVotes'] = v
    }
  }
  {
    const v = (dto as any)['upgrade-next-protocol-vote-before']
    if (v === undefined) {
    } else {
      out['upgradeNextProtocolVoteBefore'] = v
    }
  }
  {
    const v = (dto as any)['upgrade-vote-rounds']
    if (v === undefined) {
    } else {
      out['upgradeVoteRounds'] = v
    }
  }
  return out as GetStatus
}

export const GetStatus = {
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
