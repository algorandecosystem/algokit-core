import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AssetHolding, AssetHoldingDto } from './asset-holding'
import { AssetHolding as AssetHoldingModel } from './asset-holding'
import type { AssetParams, AssetParamsDto } from './asset-params'
import { AssetParams as AssetParamsModel } from './asset-params'

/**
 * AccountAssetHolding describes the account's asset holding and asset parameters (if either exist) for a specific asset ID.
 */
export type AccountAssetHolding = {
  assetHolding: AssetHolding
  assetParams?: AssetParams
}

// JSON DTO shape for AccountAssetHolding with wire keys and JSON-safe primitives
export type AccountAssetHoldingDto = {
  'asset-holding': AssetHoldingDto
  'asset-params'?: AssetParamsDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AccountAssetHolding): AccountAssetHoldingDto {
  const out: any = {}
  {
    const v = (value as any)['assetHolding']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-holding'] = v === undefined ? v : AssetHoldingModel.toDto(v)
    }
  }
  {
    const v = (value as any)['assetParams']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-params'] = v === undefined ? v : AssetParamsModel.toDto(v)
    }
  }
  return out as AccountAssetHoldingDto
}

export function fromDto(dto: AccountAssetHoldingDto): AccountAssetHolding {
  const out: any = {}
  {
    const v = (dto as any)['asset-holding']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetHolding'] = v === undefined ? v : AssetHoldingModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['asset-params']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetParams'] = v === undefined ? v : AssetParamsModel.fromDto(v)
    }
  }
  return out as AccountAssetHolding
}

// Msgpack codecs
export function encodeMsgpack(value: AccountAssetHolding): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AccountAssetHolding {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AccountAssetHolding): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AccountAssetHolding {
  return fromDto(raw as AccountAssetHoldingDto)
}

// Array helpers
export function encodeMsgpackArray(values: AccountAssetHolding[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AccountAssetHolding[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AccountAssetHolding[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AccountAssetHolding[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AccountAssetHoldingMsgpackDto = {
  'asset-holding': ReturnType<(typeof AssetHoldingModel)['toMsgpackDto']>
  'asset-params'?: ReturnType<(typeof AssetParamsModel)['toMsgpackDto']>
}

function toMsgpackDto(value: AccountAssetHolding): AccountAssetHoldingMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['assetHolding']
    if (v === undefined) {
    } else {
      out['asset-holding'] = AssetHoldingModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['assetParams']
    if (v === undefined) {
    } else {
      out['asset-params'] = AssetParamsModel.toMsgpackDto(v)
    }
  }
  return out as AccountAssetHoldingMsgpackDto
}

function fromMsgpackDto(dto: AccountAssetHoldingMsgpackDto): AccountAssetHolding {
  const out: any = {}
  {
    const v = (dto as any)['asset-holding']
    if (v === undefined) {
    } else {
      out['assetHolding'] = AssetHoldingModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['asset-params']
    if (v === undefined) {
    } else {
      out['assetParams'] = AssetParamsModel.fromMsgpackDto(v)
    }
  }
  return out as AccountAssetHolding
}

export const AccountAssetHolding = {
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
