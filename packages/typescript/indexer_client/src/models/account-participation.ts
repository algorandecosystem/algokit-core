import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * AccountParticipation describes the parameters used by this account in consensus protocol.
 */
export type AccountParticipation = {
  /**
   * Selection public key (if any) currently registered for this round.
   */
  selectionParticipationKey: Uint8Array

  /**
   * First round for which this participation is valid.
   */
  voteFirstValid: bigint

  /**
   * Number of subkeys in each batch of participation keys.
   */
  voteKeyDilution: bigint

  /**
   * Last round for which this participation is valid.
   */
  voteLastValid: bigint

  /**
   * root participation public key (if any) currently registered for this round.
   */
  voteParticipationKey: Uint8Array

  /**
   * Root of the state proof key (if any)
   */
  stateProofKey?: Uint8Array
}

// JSON DTO shape for AccountParticipation with wire keys and JSON-safe primitives
export type AccountParticipationDto = {
  'selection-participation-key': string
  'vote-first-valid': string
  'vote-key-dilution': string
  'vote-last-valid': string
  'vote-participation-key': string
  'state-proof-key'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AccountParticipation): AccountParticipationDto {
  const out: any = {}
  {
    const v = (value as any)['selectionParticipationKey']
    if (v === undefined) {
      // omit undefined
    } else {
      out['selection-participation-key'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['voteFirstValid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['vote-first-valid'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['voteKeyDilution']
    if (v === undefined) {
      // omit undefined
    } else {
      out['vote-key-dilution'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['voteLastValid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['vote-last-valid'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['voteParticipationKey']
    if (v === undefined) {
      // omit undefined
    } else {
      out['vote-participation-key'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['stateProofKey']
    if (v === undefined) {
      // omit undefined
    } else {
      out['state-proof-key'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as AccountParticipationDto
}

export function fromDto(dto: AccountParticipationDto): AccountParticipation {
  const out: any = {}
  {
    const v = (dto as any)['selection-participation-key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['selectionParticipationKey'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['vote-first-valid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['voteFirstValid'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['vote-key-dilution']
    if (v === undefined) {
      // omit undefined
    } else {
      out['voteKeyDilution'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['vote-last-valid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['voteLastValid'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['vote-participation-key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['voteParticipationKey'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['state-proof-key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stateProofKey'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as AccountParticipation
}

// Msgpack codecs
export function encodeMsgpack(value: AccountParticipation): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AccountParticipation {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AccountParticipation): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AccountParticipation {
  return fromDto(raw as AccountParticipationDto)
}

// Array helpers
export function encodeMsgpackArray(values: AccountParticipation[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AccountParticipation[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AccountParticipation[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AccountParticipation[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AccountParticipationMsgpackDto = {
  'selection-participation-key': Uint8Array
  'vote-first-valid': bigint
  'vote-key-dilution': bigint
  'vote-last-valid': bigint
  'vote-participation-key': Uint8Array
  'state-proof-key'?: Uint8Array
}

function toMsgpackDto(value: AccountParticipation): AccountParticipationMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['selectionParticipationKey']
    if (v === undefined) {
    } else {
      out['selection-participation-key'] = v
    }
  }
  {
    const v = (value as any)['voteFirstValid']
    if (v === undefined) {
    } else {
      out['vote-first-valid'] = v
    }
  }
  {
    const v = (value as any)['voteKeyDilution']
    if (v === undefined) {
    } else {
      out['vote-key-dilution'] = v
    }
  }
  {
    const v = (value as any)['voteLastValid']
    if (v === undefined) {
    } else {
      out['vote-last-valid'] = v
    }
  }
  {
    const v = (value as any)['voteParticipationKey']
    if (v === undefined) {
    } else {
      out['vote-participation-key'] = v
    }
  }
  {
    const v = (value as any)['stateProofKey']
    if (v === undefined) {
    } else {
      out['state-proof-key'] = v
    }
  }
  return out as AccountParticipationMsgpackDto
}

function fromMsgpackDto(dto: AccountParticipationMsgpackDto): AccountParticipation {
  const out: any = {}
  {
    const v = (dto as any)['selection-participation-key']
    if (v === undefined) {
    } else {
      out['selectionParticipationKey'] = v
    }
  }
  {
    const v = (dto as any)['vote-first-valid']
    if (v === undefined) {
    } else {
      out['voteFirstValid'] = v
    }
  }
  {
    const v = (dto as any)['vote-key-dilution']
    if (v === undefined) {
    } else {
      out['voteKeyDilution'] = v
    }
  }
  {
    const v = (dto as any)['vote-last-valid']
    if (v === undefined) {
    } else {
      out['voteLastValid'] = v
    }
  }
  {
    const v = (dto as any)['vote-participation-key']
    if (v === undefined) {
    } else {
      out['voteParticipationKey'] = v
    }
  }
  {
    const v = (dto as any)['state-proof-key']
    if (v === undefined) {
    } else {
      out['stateProofKey'] = v
    }
  }
  return out as AccountParticipation
}

export const AccountParticipation = {
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
