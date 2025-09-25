import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Describes an asset held by an account.
 *
 * Definition:
 * data/basics/userBalance.go : AssetHolding
 */
export type AssetHolding = {
  /**
   * \[a\] number of units held.
   */
  amount: bigint

  /**
   * Asset ID of the holding.
   */
  assetId: bigint

  /**
   * \[f\] whether or not the holding is frozen.
   */
  isFrozen: boolean
}

// JSON DTO shape for AssetHolding with wire keys and JSON-safe primitives
export type AssetHoldingDto = {
  amount: string
  'asset-id': string
  'is-frozen': boolean
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AssetHolding): AssetHoldingDto {
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
    const v = (value as any)['assetId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-id'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['isFrozen']
    if (v === undefined) {
      // omit undefined
    } else {
      out['is-frozen'] = v
    }
  }
  return out as AssetHoldingDto
}

export function fromDto(dto: AssetHoldingDto): AssetHolding {
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
    const v = (dto as any)['asset-id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetId'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['is-frozen']
    if (v === undefined) {
      // omit undefined
    } else {
      out['isFrozen'] = v as any
    }
  }
  return out as AssetHolding
}

// Msgpack codecs
export function encodeMsgpack(value: AssetHolding): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AssetHolding {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AssetHolding): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AssetHolding {
  return fromDto(raw as AssetHoldingDto)
}

// Array helpers
export function encodeMsgpackArray(values: AssetHolding[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AssetHolding[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AssetHolding[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AssetHolding[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AssetHoldingMsgpackDto = {
  amount: bigint
  'asset-id': bigint
  'is-frozen': boolean
}

function toMsgpackDto(value: AssetHolding): AssetHoldingMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['amount']
    if (v === undefined) {
    } else {
      out['amount'] = v
    }
  }
  {
    const v = (value as any)['assetId']
    if (v === undefined) {
    } else {
      out['asset-id'] = v
    }
  }
  {
    const v = (value as any)['isFrozen']
    if (v === undefined) {
    } else {
      out['is-frozen'] = v
    }
  }
  return out as AssetHoldingMsgpackDto
}

function fromMsgpackDto(dto: AssetHoldingMsgpackDto): AssetHolding {
  const out: any = {}
  {
    const v = (dto as any)['amount']
    if (v === undefined) {
    } else {
      out['amount'] = v
    }
  }
  {
    const v = (dto as any)['asset-id']
    if (v === undefined) {
    } else {
      out['assetId'] = v
    }
  }
  {
    const v = (dto as any)['is-frozen']
    if (v === undefined) {
    } else {
      out['isFrozen'] = v
    }
  }
  return out as AssetHolding
}

export const AssetHolding = {
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
