import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type GetBlockTimeStampOffset = {
  /**
   * Timestamp offset in seconds.
   */
  offset: bigint
}

// JSON DTO shape for GetBlockTimeStampOffset with wire keys and JSON-safe primitives
export type GetBlockTimeStampOffsetDto = {
  offset: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetBlockTimeStampOffset): GetBlockTimeStampOffsetDto {
  const out: any = {}
  {
    const v = (value as any)['offset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['offset'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as GetBlockTimeStampOffsetDto
}

export function fromDto(dto: GetBlockTimeStampOffsetDto): GetBlockTimeStampOffset {
  const out: any = {}
  {
    const v = (dto as any)['offset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['offset'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as GetBlockTimeStampOffset
}

// Msgpack codecs
export function encodeMsgpack(value: GetBlockTimeStampOffset): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetBlockTimeStampOffset {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetBlockTimeStampOffset): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetBlockTimeStampOffset {
  return fromDto(raw as GetBlockTimeStampOffsetDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetBlockTimeStampOffset[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetBlockTimeStampOffset[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetBlockTimeStampOffset[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetBlockTimeStampOffset[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetBlockTimeStampOffsetMsgpackDto = {
  offset: bigint
}

function toMsgpackDto(value: GetBlockTimeStampOffset): GetBlockTimeStampOffsetMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['offset']
    if (v === undefined) {
    } else {
      out['offset'] = v
    }
  }
  return out as GetBlockTimeStampOffsetMsgpackDto
}

function fromMsgpackDto(dto: GetBlockTimeStampOffsetMsgpackDto): GetBlockTimeStampOffset {
  const out: any = {}
  {
    const v = (dto as any)['offset']
    if (v === undefined) {
    } else {
      out['offset'] = v
    }
  }
  return out as GetBlockTimeStampOffset
}

export const GetBlockTimeStampOffset = {
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
