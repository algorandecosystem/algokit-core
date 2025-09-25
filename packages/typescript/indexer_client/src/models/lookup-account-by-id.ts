import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Account, AccountDto } from './account'
import { Account as AccountModel } from './account'

export type LookupAccountById = {
  account: Account

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint
}

// JSON DTO shape for LookupAccountById with wire keys and JSON-safe primitives
export type LookupAccountByIdDto = {
  account: AccountDto
  'current-round': bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LookupAccountById): LookupAccountByIdDto {
  const out: any = {}
  {
    const v = (value as any)['account']
    if (v === undefined) {
      // omit undefined
    } else {
      out['account'] = v === undefined ? v : AccountModel.toDto(v)
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
  return out as LookupAccountByIdDto
}

export function fromDto(dto: LookupAccountByIdDto): LookupAccountById {
  const out: any = {}
  {
    const v = (dto as any)['account']
    if (v === undefined) {
      // omit undefined
    } else {
      out['account'] = v === undefined ? v : AccountModel.fromDto(v)
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
  return out as LookupAccountById
}

// Msgpack codecs
export function encodeMsgpack(value: LookupAccountById): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LookupAccountById {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LookupAccountById): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LookupAccountById {
  return fromDto(raw as LookupAccountByIdDto)
}

// Array helpers
export function encodeMsgpackArray(values: LookupAccountById[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LookupAccountById[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LookupAccountById[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LookupAccountById[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LookupAccountByIdMsgpackDto = {
  account: ReturnType<(typeof AccountModel)['toMsgpackDto']>
  'current-round': bigint
}

function toMsgpackDto(value: LookupAccountById): LookupAccountByIdMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['account']
    if (v === undefined) {
    } else {
      out['account'] = AccountModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
    } else {
      out['current-round'] = v
    }
  }
  return out as LookupAccountByIdMsgpackDto
}

function fromMsgpackDto(dto: LookupAccountByIdMsgpackDto): LookupAccountById {
  const out: any = {}
  {
    const v = (dto as any)['account']
    if (v === undefined) {
    } else {
      out['account'] = AccountModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
    } else {
      out['currentRound'] = v
    }
  }
  return out as LookupAccountById
}

export const LookupAccountById = {
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
