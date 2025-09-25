import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Account, AccountDto } from './account'
import { Account as AccountModel } from './account'

export type SearchForAccounts = {
  accounts: Account[]

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
}

// JSON DTO shape for SearchForAccounts with wire keys and JSON-safe primitives
export type SearchForAccountsDto = {
  accounts: AccountDto[]
  'current-round': bigint
  'next-token'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SearchForAccounts): SearchForAccountsDto {
  const out: any = {}
  {
    const v = (value as any)['accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['accounts'] = (v as any[]).map((item) => AccountModel.toDto(item))
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
  return out as SearchForAccountsDto
}

export function fromDto(dto: SearchForAccountsDto): SearchForAccounts {
  const out: any = {}
  {
    const v = (dto as any)['accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['accounts'] = (v as any[]).map((item) => AccountModel.fromDto(item))
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
  return out as SearchForAccounts
}

// Msgpack codecs
export function encodeMsgpack(value: SearchForAccounts): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SearchForAccounts {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SearchForAccounts): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SearchForAccounts {
  return fromDto(raw as SearchForAccountsDto)
}

// Array helpers
export function encodeMsgpackArray(values: SearchForAccounts[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SearchForAccounts[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SearchForAccounts[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SearchForAccounts[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SearchForAccountsMsgpackDto = {
  accounts: ReturnType<(typeof AccountModel)['toMsgpackDto']>[]
  'current-round': bigint
  'next-token'?: string
}

function toMsgpackDto(value: SearchForAccounts): SearchForAccountsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['accounts']
    if (v === undefined) {
    } else {
      out['accounts'] = (v as any[]).map((item) => AccountModel.toMsgpackDto(item))
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
  return out as SearchForAccountsMsgpackDto
}

function fromMsgpackDto(dto: SearchForAccountsMsgpackDto): SearchForAccounts {
  const out: any = {}
  {
    const v = (dto as any)['accounts']
    if (v === undefined) {
    } else {
      out['accounts'] = (v as any[]).map((item) => AccountModel.fromMsgpackDto(item))
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
  return out as SearchForAccounts
}

export const SearchForAccounts = {
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
