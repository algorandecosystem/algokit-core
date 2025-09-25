import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Transaction, TransactionDto } from './transaction'
import { Transaction as TransactionModel } from './transaction'

export type LookupTransaction = {
  transaction: Transaction

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint
}

// JSON DTO shape for LookupTransaction with wire keys and JSON-safe primitives
export type LookupTransactionDto = {
  transaction: TransactionDto
  'current-round': bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LookupTransaction): LookupTransactionDto {
  const out: any = {}
  {
    const v = (value as any)['transaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transaction'] = v === undefined ? v : TransactionModel.toDto(v)
    }
  }
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['current-round'] = v
    }
  }
  return out as LookupTransactionDto
}

export function fromDto(dto: LookupTransactionDto): LookupTransaction {
  const out: any = {}
  {
    const v = (dto as any)['transaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transaction'] = v === undefined ? v : TransactionModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['currentRound'] = v as any
    }
  }
  return out as LookupTransaction
}

// Msgpack codecs
export function encodeMsgpack(value: LookupTransaction): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LookupTransaction {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LookupTransaction): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LookupTransaction {
  return fromDto(raw as LookupTransactionDto)
}

// Array helpers
export function encodeMsgpackArray(values: LookupTransaction[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LookupTransaction[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LookupTransaction[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LookupTransaction[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LookupTransactionMsgpackDto = {
  transaction: ReturnType<(typeof TransactionModel)['toMsgpackDto']>
  'current-round': bigint
}

function toMsgpackDto(value: LookupTransaction): LookupTransactionMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['transaction']
    if (v === undefined) {
    } else {
      out['transaction'] = TransactionModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
    } else {
      out['current-round'] = v
    }
  }
  return out as LookupTransactionMsgpackDto
}

function fromMsgpackDto(dto: LookupTransactionMsgpackDto): LookupTransaction {
  const out: any = {}
  {
    const v = (dto as any)['transaction']
    if (v === undefined) {
    } else {
      out['transaction'] = TransactionModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
    } else {
      out['currentRound'] = v
    }
  }
  return out as LookupTransaction
}

export const LookupTransaction = {
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
