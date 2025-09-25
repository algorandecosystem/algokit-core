import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type GetBlockTxids = {
  /**
   * Block transaction IDs.
   */
  blockTxids: string[]
}

// JSON DTO shape for GetBlockTxids with wire keys and JSON-safe primitives
export type GetBlockTxidsDto = {
  blockTxids: string[][]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetBlockTxids): GetBlockTxidsDto {
  const out: any = {}
  {
    const v = (value as any)['blockTxids']
    if (v === undefined) {
      // omit undefined
    } else {
      out['blockTxids'] = v as any[]
    }
  }
  return out as GetBlockTxidsDto
}

export function fromDto(dto: GetBlockTxidsDto): GetBlockTxids {
  const out: any = {}
  {
    const v = (dto as any)['blockTxids']
    if (v === undefined) {
      // omit undefined
    } else {
      out['blockTxids'] = v as any[]
    }
  }
  return out as GetBlockTxids
}

// Msgpack codecs
export function encodeMsgpack(value: GetBlockTxids): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetBlockTxids {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetBlockTxids): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetBlockTxids {
  return fromDto(raw as GetBlockTxidsDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetBlockTxids[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetBlockTxids[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetBlockTxids[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetBlockTxids[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetBlockTxidsMsgpackDto = {
  blockTxids: string[][]
}

function toMsgpackDto(value: GetBlockTxids): GetBlockTxidsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['blockTxids']
    if (v === undefined) {
    } else {
      out['blockTxids'] = v as any[]
    }
  }
  return out as GetBlockTxidsMsgpackDto
}

function fromMsgpackDto(dto: GetBlockTxidsMsgpackDto): GetBlockTxids {
  const out: any = {}
  {
    const v = (dto as any)['blockTxids']
    if (v === undefined) {
    } else {
      out['blockTxids'] = v as any[]
    }
  }
  return out as GetBlockTxids
}

export const GetBlockTxids = {
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
