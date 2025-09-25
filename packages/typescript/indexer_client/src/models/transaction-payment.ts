import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Fields for a payment transaction.
 *
 * Definition:
 * data/transactions/payment.go : PaymentTxnFields
 */
export type TransactionPayment = {
  /**
   * \[amt\] number of MicroAlgos intended to be transferred.
   */
  amount: bigint

  /**
   * Number of MicroAlgos that were sent to the close-remainder-to address when closing the sender account.
   */
  closeAmount?: bigint

  /**
   * \[close\] when set, indicates that the sending account should be closed and all remaining funds be transferred to this address.
   */
  closeRemainderTo?: string

  /**
   * \[rcv\] receiver's address.
   */
  receiver: string
}

// JSON DTO shape for TransactionPayment with wire keys and JSON-safe primitives
export type TransactionPaymentDto = {
  amount: string
  'close-amount'?: bigint
  'close-remainder-to'?: string
  receiver: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionPayment): TransactionPaymentDto {
  const out: any = {}
  {
    const v = (value as any)['amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['amount'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['closeAmount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['close-amount'] = v
    }
  }
  {
    const v = (value as any)['closeRemainderTo']
    if (v === undefined) {
      // omit undefined
    } else {
      out['close-remainder-to'] = v
    }
  }
  {
    const v = (value as any)['receiver']
    if (v === undefined) {
      // omit undefined
    } else {
      out['receiver'] = v
    }
  }
  return out as TransactionPaymentDto
}

export function fromDto(dto: TransactionPaymentDto): TransactionPayment {
  const out: any = {}
  {
    const v = (dto as any)['amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['amount'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['close-amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closeAmount'] = v as any
    }
  }
  {
    const v = (dto as any)['close-remainder-to']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closeRemainderTo'] = v as any
    }
  }
  {
    const v = (dto as any)['receiver']
    if (v === undefined) {
      // omit undefined
    } else {
      out['receiver'] = v as any
    }
  }
  return out as TransactionPayment
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionPayment): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionPayment {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionPayment): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionPayment {
  return fromDto(raw as TransactionPaymentDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionPayment[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionPayment[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionPayment[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionPayment[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionPaymentMsgpackDto = {
  amount: bigint
  'close-amount'?: bigint
  'close-remainder-to'?: string
  receiver: string
}

function toMsgpackDto(value: TransactionPayment): TransactionPaymentMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['amount']
    if (v === undefined) {
    } else {
      out['amount'] = v
    }
  }
  {
    const v = (value as any)['closeAmount']
    if (v === undefined) {
    } else {
      out['close-amount'] = v
    }
  }
  {
    const v = (value as any)['closeRemainderTo']
    if (v === undefined) {
    } else {
      out['close-remainder-to'] = v
    }
  }
  {
    const v = (value as any)['receiver']
    if (v === undefined) {
    } else {
      out['receiver'] = v
    }
  }
  return out as TransactionPaymentMsgpackDto
}

function fromMsgpackDto(dto: TransactionPaymentMsgpackDto): TransactionPayment {
  const out: any = {}
  {
    const v = (dto as any)['amount']
    if (v === undefined) {
    } else {
      out['amount'] = v
    }
  }
  {
    const v = (dto as any)['close-amount']
    if (v === undefined) {
    } else {
      out['closeAmount'] = v
    }
  }
  {
    const v = (dto as any)['close-remainder-to']
    if (v === undefined) {
    } else {
      out['closeRemainderTo'] = v
    }
  }
  {
    const v = (dto as any)['receiver']
    if (v === undefined) {
    } else {
      out['receiver'] = v
    }
  }
  return out as TransactionPayment
}

export const TransactionPayment = {
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
