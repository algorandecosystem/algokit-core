import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type GetBlockHash = {
  /**
   * Block header hash.
   */
  blockHash: string
}

// JSON DTO shape for GetBlockHash with wire keys and JSON-safe primitives
export type GetBlockHashDto = {
  blockHash: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetBlockHash): GetBlockHashDto {
  const out: any = {}
  {
    const v = (value as any)['blockHash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['blockHash'] = v
    }
  }
  return out as GetBlockHashDto
}

export function fromDto(dto: GetBlockHashDto): GetBlockHash {
  const out: any = {}
  {
    const v = (dto as any)['blockHash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['blockHash'] = v as any
    }
  }
  return out as GetBlockHash
}

// Msgpack codecs
export function encodeMsgpack(value: GetBlockHash): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetBlockHash {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetBlockHash): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetBlockHash {
  return fromDto(raw as GetBlockHashDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetBlockHash[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetBlockHash[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetBlockHash[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetBlockHash[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetBlockHashMsgpackDto = {
  blockHash: string
}

function toMsgpackDto(value: GetBlockHash): GetBlockHashMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['blockHash']
    if (v === undefined) {
    } else {
      out['blockHash'] = v
    }
  }
  return out as GetBlockHashMsgpackDto
}

function fromMsgpackDto(dto: GetBlockHashMsgpackDto): GetBlockHash {
  const out: any = {}
  {
    const v = (dto as any)['blockHash']
    if (v === undefined) {
    } else {
      out['blockHash'] = v
    }
  }
  return out as GetBlockHash
}

export const GetBlockHash = {
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
