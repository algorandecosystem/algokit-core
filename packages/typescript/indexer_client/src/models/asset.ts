import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AssetParams, AssetParamsDto } from './asset-params'
import { AssetParams as AssetParamsModel } from './asset-params'

/**
 * Specifies both the unique identifier and the parameters for an asset
 */
export type Asset = {
  /**
   * unique asset identifier
   */
  index: bigint

  /**
   * Whether or not this asset is currently deleted.
   */
  deleted?: boolean

  /**
   * Round during which this asset was created.
   */
  createdAtRound?: bigint

  /**
   * Round during which this asset was destroyed.
   */
  destroyedAtRound?: bigint
  params: AssetParams
}

// JSON DTO shape for Asset with wire keys and JSON-safe primitives
export type AssetDto = {
  index: string
  deleted?: boolean
  'created-at-round'?: string
  'destroyed-at-round'?: string
  params: AssetParamsDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: Asset): AssetDto {
  const out: any = {}
  {
    const v = (value as any)['index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['index'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
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
    const v = (value as any)['createdAtRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['created-at-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['destroyedAtRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['destroyed-at-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['params']
    if (v === undefined) {
      // omit undefined
    } else {
      out['params'] = v === undefined ? v : AssetParamsModel.toDto(v)
    }
  }
  return out as AssetDto
}

export function fromDto(dto: AssetDto): Asset {
  const out: any = {}
  {
    const v = (dto as any)['index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['index'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
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
    const v = (dto as any)['created-at-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['createdAtRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['destroyed-at-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['destroyedAtRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['params']
    if (v === undefined) {
      // omit undefined
    } else {
      out['params'] = v === undefined ? v : AssetParamsModel.fromDto(v)
    }
  }
  return out as Asset
}

// Msgpack codecs
export function encodeMsgpack(value: Asset): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): Asset {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: Asset): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): Asset {
  return fromDto(raw as AssetDto)
}

// Array helpers
export function encodeMsgpackArray(values: Asset[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): Asset[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: Asset[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): Asset[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AssetMsgpackDto = {
  index: bigint
  deleted?: boolean
  'created-at-round'?: bigint
  'destroyed-at-round'?: bigint
  params: ReturnType<(typeof AssetParamsModel)['toMsgpackDto']>
}

function toMsgpackDto(value: Asset): AssetMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['index']
    if (v === undefined) {
    } else {
      out['index'] = v
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
    const v = (value as any)['createdAtRound']
    if (v === undefined) {
    } else {
      out['created-at-round'] = v
    }
  }
  {
    const v = (value as any)['destroyedAtRound']
    if (v === undefined) {
    } else {
      out['destroyed-at-round'] = v
    }
  }
  {
    const v = (value as any)['params']
    if (v === undefined) {
    } else {
      out['params'] = AssetParamsModel.toMsgpackDto(v)
    }
  }
  return out as AssetMsgpackDto
}

function fromMsgpackDto(dto: AssetMsgpackDto): Asset {
  const out: any = {}
  {
    const v = (dto as any)['index']
    if (v === undefined) {
    } else {
      out['index'] = v
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
    const v = (dto as any)['created-at-round']
    if (v === undefined) {
    } else {
      out['createdAtRound'] = v
    }
  }
  {
    const v = (dto as any)['destroyed-at-round']
    if (v === undefined) {
    } else {
      out['destroyedAtRound'] = v
    }
  }
  {
    const v = (dto as any)['params']
    if (v === undefined) {
    } else {
      out['params'] = AssetParamsModel.fromMsgpackDto(v)
    }
  }
  return out as Asset
}

export const Asset = {
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
