import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AccountAssetHolding, AccountAssetHoldingDto } from './account-asset-holding'
import { AccountAssetHolding as AccountAssetHoldingModel } from './account-asset-holding'

export type AccountAssetsInformation = {
  /**
   * The round for which this information is relevant.
   */
  round: bigint

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
  assetHoldings?: AccountAssetHolding[]
}

// JSON DTO shape for AccountAssetsInformation with wire keys and JSON-safe primitives
export type AccountAssetsInformationDto = {
  round: string
  'next-token'?: string
  'asset-holdings'?: AccountAssetHoldingDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AccountAssetsInformation): AccountAssetsInformationDto {
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
    const v = (value as any)['nextToken']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-token'] = v
    }
  }
  {
    const v = (value as any)['assetHoldings']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-holdings'] = (v as any[]).map((item) => AccountAssetHoldingModel.toDto(item))
    }
  }
  return out as AccountAssetsInformationDto
}

export function fromDto(dto: AccountAssetsInformationDto): AccountAssetsInformation {
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
    const v = (dto as any)['next-token']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextToken'] = v as any
    }
  }
  {
    const v = (dto as any)['asset-holdings']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetHoldings'] = (v as any[]).map((item) => AccountAssetHoldingModel.fromDto(item))
    }
  }
  return out as AccountAssetsInformation
}

// Msgpack codecs
export function encodeMsgpack(value: AccountAssetsInformation): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AccountAssetsInformation {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AccountAssetsInformation): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AccountAssetsInformation {
  return fromDto(raw as AccountAssetsInformationDto)
}

// Array helpers
export function encodeMsgpackArray(values: AccountAssetsInformation[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AccountAssetsInformation[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AccountAssetsInformation[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AccountAssetsInformation[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AccountAssetsInformationMsgpackDto = {
  round: bigint
  'next-token'?: string
  'asset-holdings'?: ReturnType<(typeof AccountAssetHoldingModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: AccountAssetsInformation): AccountAssetsInformationMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (value as any)['nextToken']
    if (v === undefined) {
    } else {
      out['next-token'] = v
    }
  }
  {
    const v = (value as any)['assetHoldings']
    if (v === undefined) {
    } else {
      out['asset-holdings'] = (v as any[]).map((item) => AccountAssetHoldingModel.toMsgpackDto(item))
    }
  }
  return out as AccountAssetsInformationMsgpackDto
}

function fromMsgpackDto(dto: AccountAssetsInformationMsgpackDto): AccountAssetsInformation {
  const out: any = {}
  {
    const v = (dto as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (dto as any)['next-token']
    if (v === undefined) {
    } else {
      out['nextToken'] = v
    }
  }
  {
    const v = (dto as any)['asset-holdings']
    if (v === undefined) {
    } else {
      out['assetHoldings'] = (v as any[]).map((item) => AccountAssetHoldingModel.fromMsgpackDto(item))
    }
  }
  return out as AccountAssetsInformation
}

export const AccountAssetsInformation = {
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
