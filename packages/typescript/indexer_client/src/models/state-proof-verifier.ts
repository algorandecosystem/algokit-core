import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type StateProofVerifier = {
  /**
   * \[cmt\] Represents the root of the vector commitment tree.
   */
  commitment?: Uint8Array

  /**
   * \[lf\] Key lifetime.
   */
  keyLifetime?: bigint
}

// JSON DTO shape for StateProofVerifier with wire keys and JSON-safe primitives
export type StateProofVerifierDto = {
  commitment?: string
  'key-lifetime'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateProofVerifier): StateProofVerifierDto {
  const out: any = {}
  {
    const v = (value as any)['commitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['commitment'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['keyLifetime']
    if (v === undefined) {
      // omit undefined
    } else {
      out['key-lifetime'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as StateProofVerifierDto
}

export function fromDto(dto: StateProofVerifierDto): StateProofVerifier {
  const out: any = {}
  {
    const v = (dto as any)['commitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['commitment'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['key-lifetime']
    if (v === undefined) {
      // omit undefined
    } else {
      out['keyLifetime'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as StateProofVerifier
}

// Msgpack codecs
export function encodeMsgpack(value: StateProofVerifier): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StateProofVerifier {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StateProofVerifier): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StateProofVerifier {
  return fromDto(raw as StateProofVerifierDto)
}

// Array helpers
export function encodeMsgpackArray(values: StateProofVerifier[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StateProofVerifier[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StateProofVerifier[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StateProofVerifier[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StateProofVerifierMsgpackDto = {
  commitment?: Uint8Array
  'key-lifetime'?: bigint
}

function toMsgpackDto(value: StateProofVerifier): StateProofVerifierMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['commitment']
    if (v === undefined) {
    } else {
      out['commitment'] = v
    }
  }
  {
    const v = (value as any)['keyLifetime']
    if (v === undefined) {
    } else {
      out['key-lifetime'] = v
    }
  }
  return out as StateProofVerifierMsgpackDto
}

function fromMsgpackDto(dto: StateProofVerifierMsgpackDto): StateProofVerifier {
  const out: any = {}
  {
    const v = (dto as any)['commitment']
    if (v === undefined) {
    } else {
      out['commitment'] = v
    }
  }
  {
    const v = (dto as any)['key-lifetime']
    if (v === undefined) {
    } else {
      out['keyLifetime'] = v
    }
  }
  return out as StateProofVerifier
}

export const StateProofVerifier = {
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
