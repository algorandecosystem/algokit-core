import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * HoldingRef names a holding by referring to an Address and Asset it belongs to.
 */
export type HoldingRef = {
  /**
   * \[d\] Address in access list, or the sender of the transaction.
   */
  address: string

  /**
   * \[s\] Asset ID for asset in access list.
   */
  asset: bigint
}

// JSON DTO shape for HoldingRef with wire keys and JSON-safe primitives
export type HoldingRefDto = {
  address: string
  asset: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: HoldingRef): HoldingRefDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['asset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset'] = v
    }
  }
  return out as HoldingRefDto
}

export function fromDto(dto: HoldingRefDto): HoldingRef {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['address'] = v as any
    }
  }
  {
    const v = (dto as any)['asset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset'] = v as any
    }
  }
  return out as HoldingRef
}

// Msgpack codecs
export function encodeMsgpack(value: HoldingRef): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): HoldingRef {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: HoldingRef): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): HoldingRef {
  return fromDto(raw as HoldingRefDto)
}

// Array helpers
export function encodeMsgpackArray(values: HoldingRef[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): HoldingRef[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: HoldingRef[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): HoldingRef[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type HoldingRefMsgpackDto = {
  address: string
  asset: bigint
}

function toMsgpackDto(value: HoldingRef): HoldingRefMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['asset']
    if (v === undefined) {
    } else {
      out['asset'] = v
    }
  }
  return out as HoldingRefMsgpackDto
}

function fromMsgpackDto(dto: HoldingRefMsgpackDto): HoldingRef {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (dto as any)['asset']
    if (v === undefined) {
    } else {
      out['asset'] = v
    }
  }
  return out as HoldingRef
}

export const HoldingRef = {
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
