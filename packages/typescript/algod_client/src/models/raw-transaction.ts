import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type RawTransaction = {
  /**
   * encoding of the transaction hash.
   */
  txId: string
}

// JSON DTO shape for RawTransaction with wire keys and JSON-safe primitives
export type RawTransactionDto = {
  txId: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: RawTransaction): RawTransactionDto {
  const out: any = {}
  {
    const v = (value as any)['txId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txId'] = v
    }
  }
  return out as RawTransactionDto
}

export function fromDto(dto: RawTransactionDto): RawTransaction {
  const out: any = {}
  {
    const v = (dto as any)['txId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txId'] = v as any
    }
  }
  return out as RawTransaction
}

// Msgpack codecs
export function encodeMsgpack(value: RawTransaction): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): RawTransaction {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: RawTransaction): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): RawTransaction {
  return fromDto(raw as RawTransactionDto)
}

// Array helpers
export function encodeMsgpackArray(values: RawTransaction[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): RawTransaction[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: RawTransaction[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): RawTransaction[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type RawTransactionMsgpackDto = {
  txId: string
}

function toMsgpackDto(value: RawTransaction): RawTransactionMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['txId']
    if (v === undefined) {
    } else {
      out['txId'] = v
    }
  }
  return out as RawTransactionMsgpackDto
}

function fromMsgpackDto(dto: RawTransactionMsgpackDto): RawTransaction {
  const out: any = {}
  {
    const v = (dto as any)['txId']
    if (v === undefined) {
    } else {
      out['txId'] = v
    }
  }
  return out as RawTransaction
}

export const RawTransaction = {
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
