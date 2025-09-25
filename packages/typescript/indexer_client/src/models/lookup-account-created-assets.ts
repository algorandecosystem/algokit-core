import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Asset, AssetDto } from './asset'
import { Asset as AssetModel } from './asset'

export type LookupAccountCreatedAssets = {
  assets: Asset[]

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
}

// JSON DTO shape for LookupAccountCreatedAssets with wire keys and JSON-safe primitives
export type LookupAccountCreatedAssetsDto = {
  assets: AssetDto[]
  'current-round': bigint
  'next-token'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LookupAccountCreatedAssets): LookupAccountCreatedAssetsDto {
  const out: any = {}
  {
    const v = (value as any)['assets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assets'] = (v as any[]).map((item) => AssetModel.toDto(item))
    }
  }
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['current-round'] = v
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
  return out as LookupAccountCreatedAssetsDto
}

export function fromDto(dto: LookupAccountCreatedAssetsDto): LookupAccountCreatedAssets {
  const out: any = {}
  {
    const v = (dto as any)['assets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assets'] = (v as any[]).map((item) => AssetModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['currentRound'] = v as any
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
  return out as LookupAccountCreatedAssets
}

// Msgpack codecs
export function encodeMsgpack(value: LookupAccountCreatedAssets): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LookupAccountCreatedAssets {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LookupAccountCreatedAssets): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LookupAccountCreatedAssets {
  return fromDto(raw as LookupAccountCreatedAssetsDto)
}

// Array helpers
export function encodeMsgpackArray(values: LookupAccountCreatedAssets[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LookupAccountCreatedAssets[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LookupAccountCreatedAssets[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LookupAccountCreatedAssets[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LookupAccountCreatedAssetsMsgpackDto = {
  assets: ReturnType<(typeof AssetModel)['toMsgpackDto']>[]
  'current-round': bigint
  'next-token'?: string
}

function toMsgpackDto(value: LookupAccountCreatedAssets): LookupAccountCreatedAssetsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['assets']
    if (v === undefined) {
    } else {
      out['assets'] = (v as any[]).map((item) => AssetModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
    } else {
      out['current-round'] = v
    }
  }
  {
    const v = (value as any)['nextToken']
    if (v === undefined) {
    } else {
      out['next-token'] = v
    }
  }
  return out as LookupAccountCreatedAssetsMsgpackDto
}

function fromMsgpackDto(dto: LookupAccountCreatedAssetsMsgpackDto): LookupAccountCreatedAssets {
  const out: any = {}
  {
    const v = (dto as any)['assets']
    if (v === undefined) {
    } else {
      out['assets'] = (v as any[]).map((item) => AssetModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
    } else {
      out['currentRound'] = v
    }
  }
  {
    const v = (dto as any)['next-token']
    if (v === undefined) {
    } else {
      out['nextToken'] = v
    }
  }
  return out as LookupAccountCreatedAssets
}

export const LookupAccountCreatedAssets = {
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
