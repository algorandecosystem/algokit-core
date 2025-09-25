import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { StateProofSignature, StateProofSignatureDto } from './state-proof-signature'
import { StateProofSignature as StateProofSignatureModel } from './state-proof-signature'

export type StateProofSigSlot = {
  signature?: StateProofSignature

  /**
   * \[l\] The total weight of signatures in the lower-numbered slots.
   */
  lowerSigWeight?: bigint
}

// JSON DTO shape for StateProofSigSlot with wire keys and JSON-safe primitives
export type StateProofSigSlotDto = {
  signature?: StateProofSignatureDto
  'lower-sig-weight'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateProofSigSlot): StateProofSigSlotDto {
  const out: any = {}
  {
    const v = (value as any)['signature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['signature'] = v === undefined ? v : StateProofSignatureModel.toDto(v)
    }
  }
  {
    const v = (value as any)['lowerSigWeight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lower-sig-weight'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as StateProofSigSlotDto
}

export function fromDto(dto: StateProofSigSlotDto): StateProofSigSlot {
  const out: any = {}
  {
    const v = (dto as any)['signature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['signature'] = v === undefined ? v : StateProofSignatureModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['lower-sig-weight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lowerSigWeight'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as StateProofSigSlot
}

// Msgpack codecs
export function encodeMsgpack(value: StateProofSigSlot): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StateProofSigSlot {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StateProofSigSlot): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StateProofSigSlot {
  return fromDto(raw as StateProofSigSlotDto)
}

// Array helpers
export function encodeMsgpackArray(values: StateProofSigSlot[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StateProofSigSlot[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StateProofSigSlot[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StateProofSigSlot[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StateProofSigSlotMsgpackDto = {
  signature?: ReturnType<(typeof StateProofSignatureModel)['toMsgpackDto']>
  'lower-sig-weight'?: bigint
}

function toMsgpackDto(value: StateProofSigSlot): StateProofSigSlotMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['signature']
    if (v === undefined) {
    } else {
      out['signature'] = StateProofSignatureModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['lowerSigWeight']
    if (v === undefined) {
    } else {
      out['lower-sig-weight'] = v
    }
  }
  return out as StateProofSigSlotMsgpackDto
}

function fromMsgpackDto(dto: StateProofSigSlotMsgpackDto): StateProofSigSlot {
  const out: any = {}
  {
    const v = (dto as any)['signature']
    if (v === undefined) {
    } else {
      out['signature'] = StateProofSignatureModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['lower-sig-weight']
    if (v === undefined) {
    } else {
      out['lowerSigWeight'] = v
    }
  }
  return out as StateProofSigSlot
}

export const StateProofSigSlot = {
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
