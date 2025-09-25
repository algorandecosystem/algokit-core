import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type GetSyncRound = {
  /**
   * The minimum sync round for the ledger.
   */
  round: bigint
}

// JSON DTO shape for GetSyncRound with wire keys and JSON-safe primitives
export type GetSyncRoundDto = {
  round: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetSyncRound): GetSyncRoundDto {
  const out: any = {}
  {
    const v = (value as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as GetSyncRoundDto
}

export function fromDto(dto: GetSyncRoundDto): GetSyncRound {
  const out: any = {}
  {
    const v = (dto as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as GetSyncRound
}

// Msgpack codecs
export function encodeMsgpack(value: GetSyncRound): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetSyncRound {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetSyncRound): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetSyncRound {
  return fromDto(raw as GetSyncRoundDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetSyncRound[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetSyncRound[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetSyncRound[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetSyncRound[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetSyncRoundMsgpackDto = {
  round: bigint
}

function toMsgpackDto(value: GetSyncRound): GetSyncRoundMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  return out as GetSyncRoundMsgpackDto
}

function fromMsgpackDto(dto: GetSyncRoundMsgpackDto): GetSyncRound {
  const out: any = {}
  {
    const v = (dto as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  return out as GetSyncRound
}

export const GetSyncRound = {
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
