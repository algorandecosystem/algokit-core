import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AssetHolding, AssetHoldingDto } from './asset-holding'
import { AssetHolding as AssetHoldingModel } from './asset-holding'
import type { AssetParams, AssetParamsDto } from './asset-params'
import { AssetParams as AssetParamsModel } from './asset-params'

export type AccountAssetInformation = {
  /**
   * The round for which this information is relevant.
   */
  round: bigint
  assetHolding?: AssetHolding
  createdAsset?: AssetParams
}

// JSON DTO shape for AccountAssetInformation with wire keys and JSON-safe primitives
export type AccountAssetInformationDto = {
  round: string
  'asset-holding'?: AssetHoldingDto
  'created-asset'?: AssetParamsDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AccountAssetInformation): AccountAssetInformationDto {
  const out: any = {}
  {
    const v = (value as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['assetHolding']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-holding'] = v === undefined ? v : AssetHoldingModel.toDto(v)
    }
  }
  {
    const v = (value as any)['createdAsset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['created-asset'] = v === undefined ? v : AssetParamsModel.toDto(v)
    }
  }
  return out as AccountAssetInformationDto
}

export function fromDto(dto: AccountAssetInformationDto): AccountAssetInformation {
  const out: any = {}
  {
    const v = (dto as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['asset-holding']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetHolding'] = v === undefined ? v : AssetHoldingModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['created-asset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['createdAsset'] = v === undefined ? v : AssetParamsModel.fromDto(v)
    }
  }
  return out as AccountAssetInformation
}

// Msgpack codecs
export function encodeMsgpack(value: AccountAssetInformation): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AccountAssetInformation {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AccountAssetInformation): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AccountAssetInformation {
  return fromDto(raw as AccountAssetInformationDto)
}

// Array helpers
export function encodeMsgpackArray(values: AccountAssetInformation[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AccountAssetInformation[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AccountAssetInformation[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AccountAssetInformation[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AccountAssetInformationMsgpackDto = {
  round: bigint
  'asset-holding'?: ReturnType<(typeof AssetHoldingModel)['toMsgpackDto']>
  'created-asset'?: ReturnType<(typeof AssetParamsModel)['toMsgpackDto']>
}

function toMsgpackDto(value: AccountAssetInformation): AccountAssetInformationMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (value as any)['assetHolding']
    if (v === undefined) {
    } else {
      out['asset-holding'] = AssetHoldingModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['createdAsset']
    if (v === undefined) {
    } else {
      out['created-asset'] = AssetParamsModel.toMsgpackDto(v)
    }
  }
  return out as AccountAssetInformationMsgpackDto
}

function fromMsgpackDto(dto: AccountAssetInformationMsgpackDto): AccountAssetInformation {
  const out: any = {}
  {
    const v = (dto as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (dto as any)['asset-holding']
    if (v === undefined) {
    } else {
      out['assetHolding'] = AssetHoldingModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['created-asset']
    if (v === undefined) {
    } else {
      out['createdAsset'] = AssetParamsModel.fromMsgpackDto(v)
    }
  }
  return out as AccountAssetInformation
}

export const AccountAssetInformation = {
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
