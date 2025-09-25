import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type StateProofTracking = {
  /**
   * State Proof Type. Note the raw object uses map with this as key.
   */
  type?: bigint

  /**
   * \[v\] Root of a vector commitment containing online accounts that will help sign the proof.
   */
  votersCommitment?: Uint8Array

  /**
   * \[t\] The total number of microalgos held by the online accounts during the StateProof round.
   */
  onlineTotalWeight?: bigint

  /**
   * \[n\] Next round for which we will accept a state proof transaction.
   */
  nextRound?: bigint
}

// JSON DTO shape for StateProofTracking with wire keys and JSON-safe primitives
export type StateProofTrackingDto = {
  type?: string
  'voters-commitment'?: string
  'online-total-weight'?: bigint
  'next-round'?: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateProofTracking): StateProofTrackingDto {
  const out: any = {}
  {
    const v = (value as any)['type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['type'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['votersCommitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['voters-commitment'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['onlineTotalWeight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['online-total-weight'] = v
    }
  }
  {
    const v = (value as any)['nextRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-round'] = v
    }
  }
  return out as StateProofTrackingDto
}

export function fromDto(dto: StateProofTrackingDto): StateProofTracking {
  const out: any = {}
  {
    const v = (dto as any)['type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['type'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['voters-commitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['votersCommitment'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['online-total-weight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['onlineTotalWeight'] = v as any
    }
  }
  {
    const v = (dto as any)['next-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextRound'] = v as any
    }
  }
  return out as StateProofTracking
}

// Msgpack codecs
export function encodeMsgpack(value: StateProofTracking): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StateProofTracking {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StateProofTracking): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StateProofTracking {
  return fromDto(raw as StateProofTrackingDto)
}

// Array helpers
export function encodeMsgpackArray(values: StateProofTracking[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StateProofTracking[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StateProofTracking[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StateProofTracking[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StateProofTrackingMsgpackDto = {
  type?: bigint
  'voters-commitment'?: Uint8Array
  'online-total-weight'?: bigint
  'next-round'?: bigint
}

function toMsgpackDto(value: StateProofTracking): StateProofTrackingMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['type']
    if (v === undefined) {
    } else {
      out['type'] = v
    }
  }
  {
    const v = (value as any)['votersCommitment']
    if (v === undefined) {
    } else {
      out['voters-commitment'] = v
    }
  }
  {
    const v = (value as any)['onlineTotalWeight']
    if (v === undefined) {
    } else {
      out['online-total-weight'] = v
    }
  }
  {
    const v = (value as any)['nextRound']
    if (v === undefined) {
    } else {
      out['next-round'] = v
    }
  }
  return out as StateProofTrackingMsgpackDto
}

function fromMsgpackDto(dto: StateProofTrackingMsgpackDto): StateProofTracking {
  const out: any = {}
  {
    const v = (dto as any)['type']
    if (v === undefined) {
    } else {
      out['type'] = v
    }
  }
  {
    const v = (dto as any)['voters-commitment']
    if (v === undefined) {
    } else {
      out['votersCommitment'] = v
    }
  }
  {
    const v = (dto as any)['online-total-weight']
    if (v === undefined) {
    } else {
      out['onlineTotalWeight'] = v
    }
  }
  {
    const v = (dto as any)['next-round']
    if (v === undefined) {
    } else {
      out['nextRound'] = v
    }
  }
  return out as StateProofTracking
}

export const StateProofTracking = {
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
