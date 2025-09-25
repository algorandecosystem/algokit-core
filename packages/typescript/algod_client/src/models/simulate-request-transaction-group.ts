import { encodeSignedTransaction, encodeSignedTransactions, decodeSignedTransaction } from '@algorandfoundation/algokit-transact'
import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * A transaction group to simulate.
 */
export type SimulateRequestTransactionGroup = {
  /**
   * An atomic transaction group.
   */
  txns: SignedTransaction[]
}

// JSON DTO shape for SimulateRequestTransactionGroup with wire keys and JSON-safe primitives
export type SimulateRequestTransactionGroupDto = {
  txns: unknown[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulateRequestTransactionGroup): SimulateRequestTransactionGroupDto {
  const out: any = {}
  {
    const v = (value as any)['txns']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txns'] = (v as any[]).map((item) => {
        if (item && typeof item === 'object' && 'txn' in (item as any)) return item as any
        return toBase64(encodeSignedTransaction(item as any))
      })
    }
  }
  return out as SimulateRequestTransactionGroupDto
}

export function fromDto(dto: SimulateRequestTransactionGroupDto): SimulateRequestTransactionGroup {
  const out: any = {}
  {
    const v = (dto as any)['txns']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txns'] = (v as any[]).map((item) => {
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
  return out as SimulateRequestTransactionGroup
}

// Msgpack codecs
export function encodeMsgpack(value: SimulateRequestTransactionGroup): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulateRequestTransactionGroup {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulateRequestTransactionGroup): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulateRequestTransactionGroup {
  return fromDto(raw as SimulateRequestTransactionGroupDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulateRequestTransactionGroup[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulateRequestTransactionGroup[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulateRequestTransactionGroup[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulateRequestTransactionGroup[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulateRequestTransactionGroupMsgpackDto = {
  txns: Uint8Array[]
}

function toMsgpackDto(value: SimulateRequestTransactionGroup): SimulateRequestTransactionGroupMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['txns']
    if (v === undefined) {
    } else {
      out['txns'] = (v as any[]).map((item) => encodeSignedTransaction(item as any))
    }
  }
  return out as SimulateRequestTransactionGroupMsgpackDto
}

function fromMsgpackDto(dto: SimulateRequestTransactionGroupMsgpackDto): SimulateRequestTransactionGroup {
  const out: any = {}
  {
    const v = (dto as any)['txns']
    if (v === undefined) {
    } else {
      out['txns'] = (v as any[]).map((item) => {
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
  return out as SimulateRequestTransactionGroup
}

export const SimulateRequestTransactionGroup = {
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
