import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AssetParams, AssetParamsDto } from './asset-params'
import { AssetParams as AssetParamsModel } from './asset-params'

/**
 * Fields for asset allocation, re-configuration, and destruction.
 *
 *
 * A zero value for asset-id indicates asset creation.
 * A zero value for the params indicates asset destruction.
 *
 * Definition:
 * data/transactions/asset.go : AssetConfigTxnFields
 */
export type TransactionAssetConfig = {
  /**
   * \[xaid\] ID of the asset being configured or empty if creating.
   */
  assetId?: bigint
  params?: AssetParams
}

// JSON DTO shape for TransactionAssetConfig with wire keys and JSON-safe primitives
export type TransactionAssetConfigDto = {
  'asset-id'?: string
  params?: AssetParamsDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionAssetConfig): TransactionAssetConfigDto {
  const out: any = {}
  {
    const v = (value as any)['assetId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-id'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
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
  return out as TransactionAssetConfigDto
}

export function fromDto(dto: TransactionAssetConfigDto): TransactionAssetConfig {
  const out: any = {}
  {
    const v = (dto as any)['asset-id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetId'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
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
  return out as TransactionAssetConfig
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionAssetConfig): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionAssetConfig {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionAssetConfig): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionAssetConfig {
  return fromDto(raw as TransactionAssetConfigDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionAssetConfig[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionAssetConfig[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionAssetConfig[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionAssetConfig[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionAssetConfigMsgpackDto = {
  'asset-id'?: bigint
  params?: ReturnType<(typeof AssetParamsModel)['toMsgpackDto']>
}

function toMsgpackDto(value: TransactionAssetConfig): TransactionAssetConfigMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['assetId']
    if (v === undefined) {
    } else {
      out['asset-id'] = v
    }
  }
  {
    const v = (value as any)['params']
    if (v === undefined) {
    } else {
      out['params'] = AssetParamsModel.toMsgpackDto(v)
    }
  }
  return out as TransactionAssetConfigMsgpackDto
}

function fromMsgpackDto(dto: TransactionAssetConfigMsgpackDto): TransactionAssetConfig {
  const out: any = {}
  {
    const v = (dto as any)['asset-id']
    if (v === undefined) {
    } else {
      out['assetId'] = v
    }
  }
  {
    const v = (dto as any)['params']
    if (v === undefined) {
    } else {
      out['params'] = AssetParamsModel.fromMsgpackDto(v)
    }
  }
  return out as TransactionAssetConfig
}

export const TransactionAssetConfig = {
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
