import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Asset, AssetDto } from './asset'
import { Asset as AssetModel } from './asset'

export type LookupAssetById = {
  asset: Asset

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint
}

// JSON DTO shape for LookupAssetById with wire keys and JSON-safe primitives
export type LookupAssetByIdDto = {
  asset: AssetDto
  'current-round': bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LookupAssetById): LookupAssetByIdDto {
  const out: any = {}
  {
    const v = (value as any)['asset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset'] = v === undefined ? v : AssetModel.toDto(v)
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
  return out as LookupAssetByIdDto
}

export function fromDto(dto: LookupAssetByIdDto): LookupAssetById {
  const out: any = {}
  {
    const v = (dto as any)['asset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset'] = v === undefined ? v : AssetModel.fromDto(v)
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
  return out as LookupAssetById
}

// Msgpack codecs
export function encodeMsgpack(value: LookupAssetById): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LookupAssetById {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LookupAssetById): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LookupAssetById {
  return fromDto(raw as LookupAssetByIdDto)
}

// Array helpers
export function encodeMsgpackArray(values: LookupAssetById[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LookupAssetById[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LookupAssetById[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LookupAssetById[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LookupAssetByIdMsgpackDto = {
  asset: ReturnType<(typeof AssetModel)['toMsgpackDto']>
  'current-round': bigint
}

function toMsgpackDto(value: LookupAssetById): LookupAssetByIdMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['asset']
    if (v === undefined) {
    } else {
      out['asset'] = AssetModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
    } else {
      out['current-round'] = v
    }
  }
  return out as LookupAssetByIdMsgpackDto
}

function fromMsgpackDto(dto: LookupAssetByIdMsgpackDto): LookupAssetById {
  const out: any = {}
  {
    const v = (dto as any)['asset']
    if (v === undefined) {
    } else {
      out['asset'] = AssetModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
    } else {
      out['currentRound'] = v
    }
  }
  return out as LookupAssetById
}

export const LookupAssetById = {
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
