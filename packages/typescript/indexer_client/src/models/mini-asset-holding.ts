import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * A simplified version of AssetHolding
 */
export type MiniAssetHolding = {
  address: string
  amount: bigint
  isFrozen: boolean

  /**
   * Whether or not this asset holding is currently deleted from its account.
   */
  deleted?: boolean

  /**
   * Round during which the account opted into the asset.
   */
  optedInAtRound?: bigint

  /**
   * Round during which the account opted out of the asset.
   */
  optedOutAtRound?: bigint
}

// JSON DTO shape for MiniAssetHolding with wire keys and JSON-safe primitives
export type MiniAssetHoldingDto = {
  address: string
  amount: string
  'is-frozen': boolean
  deleted?: boolean
  'opted-in-at-round'?: string
  'opted-out-at-round'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: MiniAssetHolding): MiniAssetHoldingDto {
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
    const v = (value as any)['amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['amount'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
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
  {
    const v = (value as any)['deleted']
    if (v === undefined) {
      // omit undefined
    } else {
      out['deleted'] = v
    }
  }
  {
    const v = (value as any)['optedInAtRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['opted-in-at-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['optedOutAtRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['opted-out-at-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as MiniAssetHoldingDto
}

export function fromDto(dto: MiniAssetHoldingDto): MiniAssetHolding {
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
    const v = (dto as any)['amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['amount'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
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
  {
    const v = (dto as any)['deleted']
    if (v === undefined) {
      // omit undefined
    } else {
      out['deleted'] = v as any
    }
  }
  {
    const v = (dto as any)['opted-in-at-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['optedInAtRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['opted-out-at-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['optedOutAtRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as MiniAssetHolding
}

// Msgpack codecs
export function encodeMsgpack(value: MiniAssetHolding): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): MiniAssetHolding {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: MiniAssetHolding): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): MiniAssetHolding {
  return fromDto(raw as MiniAssetHoldingDto)
}

// Array helpers
export function encodeMsgpackArray(values: MiniAssetHolding[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): MiniAssetHolding[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: MiniAssetHolding[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): MiniAssetHolding[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type MiniAssetHoldingMsgpackDto = {
  address: string
  amount: bigint
  'is-frozen': boolean
  deleted?: boolean
  'opted-in-at-round'?: bigint
  'opted-out-at-round'?: bigint
}

function toMsgpackDto(value: MiniAssetHolding): MiniAssetHoldingMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['amount']
    if (v === undefined) {
    } else {
      out['amount'] = v
    }
  }
  {
    const v = (value as any)['isFrozen']
    if (v === undefined) {
    } else {
      out['is-frozen'] = v
    }
  }
  {
    const v = (value as any)['deleted']
    if (v === undefined) {
    } else {
      out['deleted'] = v
    }
  }
  {
    const v = (value as any)['optedInAtRound']
    if (v === undefined) {
    } else {
      out['opted-in-at-round'] = v
    }
  }
  {
    const v = (value as any)['optedOutAtRound']
    if (v === undefined) {
    } else {
      out['opted-out-at-round'] = v
    }
  }
  return out as MiniAssetHoldingMsgpackDto
}

function fromMsgpackDto(dto: MiniAssetHoldingMsgpackDto): MiniAssetHolding {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (dto as any)['amount']
    if (v === undefined) {
    } else {
      out['amount'] = v
    }
  }
  {
    const v = (dto as any)['is-frozen']
    if (v === undefined) {
    } else {
      out['isFrozen'] = v
    }
  }
  {
    const v = (dto as any)['deleted']
    if (v === undefined) {
    } else {
      out['deleted'] = v
    }
  }
  {
    const v = (dto as any)['opted-in-at-round']
    if (v === undefined) {
    } else {
      out['optedInAtRound'] = v
    }
  }
  {
    const v = (dto as any)['opted-out-at-round']
    if (v === undefined) {
    } else {
      out['optedOutAtRound'] = v
    }
  }
  return out as MiniAssetHolding
}

export const MiniAssetHolding = {
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
