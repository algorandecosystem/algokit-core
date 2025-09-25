import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * References an asset held by an account.
 */
export type AssetHoldingReference = {
  /**
   * Address of the account holding the asset.
   */
  account: string

  /**
   * Asset ID of the holding.
   */
  asset: bigint
}

// JSON DTO shape for AssetHoldingReference with wire keys and JSON-safe primitives
export type AssetHoldingReferenceDto = {
  account: string
  asset: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AssetHoldingReference): AssetHoldingReferenceDto {
  const out: any = {}
  {
    const v = (value as any)['account']
    if (v === undefined) {
      // omit undefined
    } else {
      out['account'] = v
    }
  }
  {
    const v = (value as any)['asset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as AssetHoldingReferenceDto
}

export function fromDto(dto: AssetHoldingReferenceDto): AssetHoldingReference {
  const out: any = {}
  {
    const v = (dto as any)['account']
    if (v === undefined) {
      // omit undefined
    } else {
      out['account'] = v as any
    }
  }
  {
    const v = (dto as any)['asset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as AssetHoldingReference
}

// Msgpack codecs
export function encodeMsgpack(value: AssetHoldingReference): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AssetHoldingReference {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AssetHoldingReference): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AssetHoldingReference {
  return fromDto(raw as AssetHoldingReferenceDto)
}

// Array helpers
export function encodeMsgpackArray(values: AssetHoldingReference[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AssetHoldingReference[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AssetHoldingReference[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AssetHoldingReference[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AssetHoldingReferenceMsgpackDto = {
  account: string
  asset: bigint
}

function toMsgpackDto(value: AssetHoldingReference): AssetHoldingReferenceMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['account']
    if (v === undefined) {
    } else {
      out['account'] = v
    }
  }
  {
    const v = (value as any)['asset']
    if (v === undefined) {
    } else {
      out['asset'] = v
    }
  }
  return out as AssetHoldingReferenceMsgpackDto
}

function fromMsgpackDto(dto: AssetHoldingReferenceMsgpackDto): AssetHoldingReference {
  const out: any = {}
  {
    const v = (dto as any)['account']
    if (v === undefined) {
    } else {
      out['account'] = v
    }
  }
  {
    const v = (dto as any)['asset']
    if (v === undefined) {
    } else {
      out['asset'] = v
    }
  }
  return out as AssetHoldingReference
}

export const AssetHoldingReference = {
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
