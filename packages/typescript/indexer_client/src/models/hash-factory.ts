import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type HashFactory = {
  /**
   * \[t\]
   */
  hashType?: bigint
}

// JSON DTO shape for HashFactory with wire keys and JSON-safe primitives
export type HashFactoryDto = {
  'hash-type'?: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: HashFactory): HashFactoryDto {
  const out: any = {}
  {
    const v = (value as any)['hashType']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hash-type'] = v
    }
  }
  return out as HashFactoryDto
}

export function fromDto(dto: HashFactoryDto): HashFactory {
  const out: any = {}
  {
    const v = (dto as any)['hash-type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hashType'] = v as any
    }
  }
  return out as HashFactory
}

// Msgpack codecs
export function encodeMsgpack(value: HashFactory): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): HashFactory {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: HashFactory): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): HashFactory {
  return fromDto(raw as HashFactoryDto)
}

// Array helpers
export function encodeMsgpackArray(values: HashFactory[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): HashFactory[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: HashFactory[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): HashFactory[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type HashFactoryMsgpackDto = {
  'hash-type'?: bigint
}

function toMsgpackDto(value: HashFactory): HashFactoryMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['hashType']
    if (v === undefined) {
    } else {
      out['hash-type'] = v
    }
  }
  return out as HashFactoryMsgpackDto
}

function fromMsgpackDto(dto: HashFactoryMsgpackDto): HashFactory {
  const out: any = {}
  {
    const v = (dto as any)['hash-type']
    if (v === undefined) {
    } else {
      out['hashType'] = v
    }
  }
  return out as HashFactory
}

export const HashFactory = {
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
