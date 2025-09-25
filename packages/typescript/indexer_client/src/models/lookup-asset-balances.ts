import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { MiniAssetHolding, MiniAssetHoldingDto } from './mini-asset-holding'
import { MiniAssetHolding as MiniAssetHoldingModel } from './mini-asset-holding'

export type LookupAssetBalances = {
  balances: MiniAssetHolding[]

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
}

// JSON DTO shape for LookupAssetBalances with wire keys and JSON-safe primitives
export type LookupAssetBalancesDto = {
  balances: MiniAssetHoldingDto[]
  'current-round': bigint
  'next-token'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LookupAssetBalances): LookupAssetBalancesDto {
  const out: any = {}
  {
    const v = (value as any)['balances']
    if (v === undefined) {
      // omit undefined
    } else {
      out['balances'] = (v as any[]).map((item) => MiniAssetHoldingModel.toDto(item))
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
  return out as LookupAssetBalancesDto
}

export function fromDto(dto: LookupAssetBalancesDto): LookupAssetBalances {
  const out: any = {}
  {
    const v = (dto as any)['balances']
    if (v === undefined) {
      // omit undefined
    } else {
      out['balances'] = (v as any[]).map((item) => MiniAssetHoldingModel.fromDto(item))
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
  return out as LookupAssetBalances
}

// Msgpack codecs
export function encodeMsgpack(value: LookupAssetBalances): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LookupAssetBalances {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LookupAssetBalances): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LookupAssetBalances {
  return fromDto(raw as LookupAssetBalancesDto)
}

// Array helpers
export function encodeMsgpackArray(values: LookupAssetBalances[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LookupAssetBalances[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LookupAssetBalances[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LookupAssetBalances[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LookupAssetBalancesMsgpackDto = {
  balances: ReturnType<(typeof MiniAssetHoldingModel)['toMsgpackDto']>[]
  'current-round': bigint
  'next-token'?: string
}

function toMsgpackDto(value: LookupAssetBalances): LookupAssetBalancesMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['balances']
    if (v === undefined) {
    } else {
      out['balances'] = (v as any[]).map((item) => MiniAssetHoldingModel.toMsgpackDto(item))
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
  return out as LookupAssetBalancesMsgpackDto
}

function fromMsgpackDto(dto: LookupAssetBalancesMsgpackDto): LookupAssetBalances {
  const out: any = {}
  {
    const v = (dto as any)['balances']
    if (v === undefined) {
    } else {
      out['balances'] = (v as any[]).map((item) => MiniAssetHoldingModel.fromMsgpackDto(item))
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
  return out as LookupAssetBalances
}

export const LookupAssetBalances = {
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
