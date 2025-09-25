import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Transaction, TransactionDto } from './transaction'
import { Transaction as TransactionModel } from './transaction'

export type LookupAssetTransactions = {
  /**
   * Round at which the results were computed.
   */
  currentRound: bigint

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
  transactions: Transaction[]
}

// JSON DTO shape for LookupAssetTransactions with wire keys and JSON-safe primitives
export type LookupAssetTransactionsDto = {
  'current-round': bigint
  'next-token'?: string
  transactions: TransactionDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LookupAssetTransactions): LookupAssetTransactionsDto {
  const out: any = {}
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['current-round'] = v
    }
  }
  {
    const v = (value as any)['nextToken']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-token'] = v
    }
  }
  {
    const v = (value as any)['transactions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transactions'] = (v as any[]).map((item) => TransactionModel.toDto(item))
    }
  }
  return out as LookupAssetTransactionsDto
}

export function fromDto(dto: LookupAssetTransactionsDto): LookupAssetTransactions {
  const out: any = {}
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['currentRound'] = v as any
    }
  }
  {
    const v = (dto as any)['next-token']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextToken'] = v as any
    }
  }
  {
    const v = (dto as any)['transactions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['transactions'] = (v as any[]).map((item) => TransactionModel.fromDto(item))
    }
  }
  return out as LookupAssetTransactions
}

// Msgpack codecs
export function encodeMsgpack(value: LookupAssetTransactions): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LookupAssetTransactions {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LookupAssetTransactions): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LookupAssetTransactions {
  return fromDto(raw as LookupAssetTransactionsDto)
}

// Array helpers
export function encodeMsgpackArray(values: LookupAssetTransactions[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LookupAssetTransactions[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LookupAssetTransactions[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LookupAssetTransactions[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LookupAssetTransactionsMsgpackDto = {
  'current-round': bigint
  'next-token'?: string
  transactions: ReturnType<(typeof TransactionModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: LookupAssetTransactions): LookupAssetTransactionsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
    } else {
      out['current-round'] = v
    }
  }
  {
    const v = (value as any)['nextToken']
    if (v === undefined) {
    } else {
      out['next-token'] = v
    }
  }
  {
    const v = (value as any)['transactions']
    if (v === undefined) {
    } else {
      out['transactions'] = (v as any[]).map((item) => TransactionModel.toMsgpackDto(item))
    }
  }
  return out as LookupAssetTransactionsMsgpackDto
}

function fromMsgpackDto(dto: LookupAssetTransactionsMsgpackDto): LookupAssetTransactions {
  const out: any = {}
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
    } else {
      out['currentRound'] = v
    }
  }
  {
    const v = (dto as any)['next-token']
    if (v === undefined) {
    } else {
      out['nextToken'] = v
    }
  }
  {
    const v = (dto as any)['transactions']
    if (v === undefined) {
    } else {
      out['transactions'] = (v as any[]).map((item) => TransactionModel.fromMsgpackDto(item))
    }
  }
  return out as LookupAssetTransactions
}

export const LookupAssetTransactions = {
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
