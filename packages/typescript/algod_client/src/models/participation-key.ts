import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AccountParticipation, AccountParticipationDto } from './account-participation'
import { AccountParticipation as AccountParticipationModel } from './account-participation'

/**
 * Represents a participation key used by the node.
 */
export type ParticipationKey = {
  /**
   * The key's ParticipationID.
   */
  id: string

  /**
   * Address the key was generated for.
   */
  address: string

  /**
   * When registered, this is the first round it may be used.
   */
  effectiveFirstValid?: bigint

  /**
   * When registered, this is the last round it may be used.
   */
  effectiveLastValid?: bigint

  /**
   * Round when this key was last used to vote.
   */
  lastVote?: bigint

  /**
   * Round when this key was last used to propose a block.
   */
  lastBlockProposal?: bigint

  /**
   * Round when this key was last used to generate a state proof.
   */
  lastStateProof?: bigint
  key: AccountParticipation
}

// JSON DTO shape for ParticipationKey with wire keys and JSON-safe primitives
export type ParticipationKeyDto = {
  id: string
  address: string
  'effective-first-valid'?: string
  'effective-last-valid'?: string
  'last-vote'?: bigint
  'last-block-proposal'?: bigint
  'last-state-proof'?: bigint
  key: AccountParticipationDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ParticipationKey): ParticipationKeyDto {
  const out: any = {}
  {
    const v = (value as any)['id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['id'] = v
    }
  }
  {
    const v = (value as any)['address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['effectiveFirstValid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['effective-first-valid'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['effectiveLastValid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['effective-last-valid'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['lastVote']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-vote'] = v
    }
  }
  {
    const v = (value as any)['lastBlockProposal']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-block-proposal'] = v
    }
  }
  {
    const v = (value as any)['lastStateProof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-state-proof'] = v
    }
  }
  {
    const v = (value as any)['key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['key'] = v === undefined ? v : AccountParticipationModel.toDto(v)
    }
  }
  return out as ParticipationKeyDto
}

export function fromDto(dto: ParticipationKeyDto): ParticipationKey {
  const out: any = {}
  {
    const v = (dto as any)['id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['id'] = v as any
    }
  }
  {
    const v = (dto as any)['address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['address'] = v as any
    }
  }
  {
    const v = (dto as any)['effective-first-valid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['effectiveFirstValid'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['effective-last-valid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['effectiveLastValid'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['last-vote']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastVote'] = v as any
    }
  }
  {
    const v = (dto as any)['last-block-proposal']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastBlockProposal'] = v as any
    }
  }
  {
    const v = (dto as any)['last-state-proof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastStateProof'] = v as any
    }
  }
  {
    const v = (dto as any)['key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['key'] = v === undefined ? v : AccountParticipationModel.fromDto(v)
    }
  }
  return out as ParticipationKey
}

// Msgpack codecs
export function encodeMsgpack(value: ParticipationKey): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ParticipationKey {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ParticipationKey): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ParticipationKey {
  return fromDto(raw as ParticipationKeyDto)
}

// Array helpers
export function encodeMsgpackArray(values: ParticipationKey[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ParticipationKey[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ParticipationKey[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ParticipationKey[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ParticipationKeyMsgpackDto = {
  id: string
  address: string
  'effective-first-valid'?: bigint
  'effective-last-valid'?: bigint
  'last-vote'?: bigint
  'last-block-proposal'?: bigint
  'last-state-proof'?: bigint
  key: ReturnType<(typeof AccountParticipationModel)['toMsgpackDto']>
}

function toMsgpackDto(value: ParticipationKey): ParticipationKeyMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['id']
    if (v === undefined) {
    } else {
      out['id'] = v
    }
  }
  {
    const v = (value as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['effectiveFirstValid']
    if (v === undefined) {
    } else {
      out['effective-first-valid'] = v
    }
  }
  {
    const v = (value as any)['effectiveLastValid']
    if (v === undefined) {
    } else {
      out['effective-last-valid'] = v
    }
  }
  {
    const v = (value as any)['lastVote']
    if (v === undefined) {
    } else {
      out['last-vote'] = v
    }
  }
  {
    const v = (value as any)['lastBlockProposal']
    if (v === undefined) {
    } else {
      out['last-block-proposal'] = v
    }
  }
  {
    const v = (value as any)['lastStateProof']
    if (v === undefined) {
    } else {
      out['last-state-proof'] = v
    }
  }
  {
    const v = (value as any)['key']
    if (v === undefined) {
    } else {
      out['key'] = AccountParticipationModel.toMsgpackDto(v)
    }
  }
  return out as ParticipationKeyMsgpackDto
}

function fromMsgpackDto(dto: ParticipationKeyMsgpackDto): ParticipationKey {
  const out: any = {}
  {
    const v = (dto as any)['id']
    if (v === undefined) {
    } else {
      out['id'] = v
    }
  }
  {
    const v = (dto as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (dto as any)['effective-first-valid']
    if (v === undefined) {
    } else {
      out['effectiveFirstValid'] = v
    }
  }
  {
    const v = (dto as any)['effective-last-valid']
    if (v === undefined) {
    } else {
      out['effectiveLastValid'] = v
    }
  }
  {
    const v = (dto as any)['last-vote']
    if (v === undefined) {
    } else {
      out['lastVote'] = v
    }
  }
  {
    const v = (dto as any)['last-block-proposal']
    if (v === undefined) {
    } else {
      out['lastBlockProposal'] = v
    }
  }
  {
    const v = (dto as any)['last-state-proof']
    if (v === undefined) {
    } else {
      out['lastStateProof'] = v
    }
  }
  {
    const v = (dto as any)['key']
    if (v === undefined) {
    } else {
      out['key'] = AccountParticipationModel.fromMsgpackDto(v)
    }
  }
  return out as ParticipationKey
}

export const ParticipationKey = {
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
