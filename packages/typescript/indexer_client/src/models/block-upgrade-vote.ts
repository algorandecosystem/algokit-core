import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Fields relating to voting for a protocol upgrade.
 */
export type BlockUpgradeVote = {
  /**
   * \[upgradeyes\] Indicates a yes vote for the current proposal.
   */
  upgradeApprove?: boolean

  /**
   * \[upgradedelay\] Indicates the time between acceptance and execution.
   */
  upgradeDelay?: bigint

  /**
   * \[upgradeprop\] Indicates a proposed upgrade.
   */
  upgradePropose?: string
}

// JSON DTO shape for BlockUpgradeVote with wire keys and JSON-safe primitives
export type BlockUpgradeVoteDto = {
  'upgrade-approve'?: boolean
  'upgrade-delay'?: bigint
  'upgrade-propose'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: BlockUpgradeVote): BlockUpgradeVoteDto {
  const out: any = {}
  {
    const v = (value as any)['upgradeApprove']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-approve'] = v
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
    const v = (value as any)['upgradePropose']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgrade-propose'] = v
    }
  }
  return out as BlockUpgradeVoteDto
}

export function fromDto(dto: BlockUpgradeVoteDto): BlockUpgradeVote {
  const out: any = {}
  {
    const v = (dto as any)['upgrade-approve']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradeApprove'] = v as any
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
    const v = (dto as any)['upgrade-propose']
    if (v === undefined) {
      // omit undefined
    } else {
      out['upgradePropose'] = v as any
    }
  }
  return out as BlockUpgradeVote
}

// Msgpack codecs
export function encodeMsgpack(value: BlockUpgradeVote): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): BlockUpgradeVote {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: BlockUpgradeVote): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): BlockUpgradeVote {
  return fromDto(raw as BlockUpgradeVoteDto)
}

// Array helpers
export function encodeMsgpackArray(values: BlockUpgradeVote[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): BlockUpgradeVote[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: BlockUpgradeVote[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): BlockUpgradeVote[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type BlockUpgradeVoteMsgpackDto = {
  'upgrade-approve'?: boolean
  'upgrade-delay'?: bigint
  'upgrade-propose'?: string
}

function toMsgpackDto(value: BlockUpgradeVote): BlockUpgradeVoteMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['upgradeApprove']
    if (v === undefined) {
    } else {
      out['upgrade-approve'] = v
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
    const v = (value as any)['upgradePropose']
    if (v === undefined) {
    } else {
      out['upgrade-propose'] = v
    }
  }
  return out as BlockUpgradeVoteMsgpackDto
}

function fromMsgpackDto(dto: BlockUpgradeVoteMsgpackDto): BlockUpgradeVote {
  const out: any = {}
  {
    const v = (dto as any)['upgrade-approve']
    if (v === undefined) {
    } else {
      out['upgradeApprove'] = v
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
    const v = (dto as any)['upgrade-propose']
    if (v === undefined) {
    } else {
      out['upgradePropose'] = v
    }
  }
  return out as BlockUpgradeVote
}

export const BlockUpgradeVote = {
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
