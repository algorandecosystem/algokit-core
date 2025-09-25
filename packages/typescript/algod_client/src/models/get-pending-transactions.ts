import { encodeSignedTransaction, encodeSignedTransactions, decodeSignedTransaction } from '@algorandfoundation/algokit-transact'
import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * PendingTransactions is an array of signed transactions exactly as they were submitted.
 */
export type GetPendingTransactions = {
  /**
   * An array of signed transaction objects.
   */
  topTransactions: SignedTransaction[]

  /**
   * Total number of transactions in the pool.
   */
  totalTransactions: bigint
}

// JSON DTO shape for GetPendingTransactions with wire keys and JSON-safe primitives
export type GetPendingTransactionsDto = {
  'top-transactions': unknown[]
  'total-transactions': bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetPendingTransactions): GetPendingTransactionsDto {
  const out: any = {}
  {
    const v = (value as any)['topTransactions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['top-transactions'] = (v as any[]).map((item) => {
        if (item && typeof item === 'object' && 'txn' in (item as any)) return item as any
        return toBase64(encodeSignedTransaction(item as any))
      })
    }
  }
  {
    const v = (value as any)['totalTransactions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['total-transactions'] = v
    }
  }
  return out as GetPendingTransactionsDto
}

export function fromDto(dto: GetPendingTransactionsDto): GetPendingTransactions {
  const out: any = {}
  {
    const v = (dto as any)['top-transactions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['topTransactions'] = (v as any[]).map((item) => {
        if (item instanceof Uint8Array) return decodeSignedTransaction(item)
        if (typeof item === 'string') {
          try {
            return decodeSignedTransaction(fromBase64(item))
          } catch {
            return item
          }
        }
        if (typeof item === 'object' && item != null) {
          try {
            return decodeSignedTransaction(encodeMsgPack(item))
          } catch {
            return item
          }
        }
        return item
      })
    }
  }
  {
    const v = (dto as any)['total-transactions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['totalTransactions'] = v as any
    }
  }
  return out as GetPendingTransactions
}

// Msgpack codecs
export function encodeMsgpack(value: GetPendingTransactions): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetPendingTransactions {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetPendingTransactions): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetPendingTransactions {
  return fromDto(raw as GetPendingTransactionsDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetPendingTransactions[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetPendingTransactions[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetPendingTransactions[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetPendingTransactions[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetPendingTransactionsMsgpackDto = {
  'top-transactions': Uint8Array[]
  'total-transactions': bigint
}

function toMsgpackDto(value: GetPendingTransactions): GetPendingTransactionsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['topTransactions']
    if (v === undefined) {
    } else {
      out['top-transactions'] = (v as any[]).map((item) => encodeSignedTransaction(item as any))
    }
  }
  {
    const v = (value as any)['totalTransactions']
    if (v === undefined) {
    } else {
      out['total-transactions'] = v
    }
  }
  return out as GetPendingTransactionsMsgpackDto
}

function fromMsgpackDto(dto: GetPendingTransactionsMsgpackDto): GetPendingTransactions {
  const out: any = {}
  {
    const v = (dto as any)['top-transactions']
    if (v === undefined) {
    } else {
      out['topTransactions'] = (v as any[]).map((item) => {
        // eslint-disable-line @typescript-eslint/no-explicit-any
        if (item instanceof Uint8Array) return decodeSignedTransaction(item)
        if (typeof item === 'object' && item != null) {
          try {
            return decodeSignedTransaction(encodeMsgPack(item))
          } catch {
            return item as any
          }
        }
        return item as any
      })
    }
  }
  {
    const v = (dto as any)['total-transactions']
    if (v === undefined) {
    } else {
      out['totalTransactions'] = v
    }
  }
  return out as GetPendingTransactions
}

export const GetPendingTransactions = {
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
