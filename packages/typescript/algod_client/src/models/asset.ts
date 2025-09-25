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
  params: AssetParams
}

// JSON DTO shape for Asset with wire keys and JSON-safe primitives
export type AssetDto = {
  index: string
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
