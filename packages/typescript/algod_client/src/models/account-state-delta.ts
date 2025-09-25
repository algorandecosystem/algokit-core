import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { StateDelta, StateDeltaDto } from './state-delta'
import { StateDelta as StateDeltaModel } from './state-delta'

/**
 * Application state delta.
 */
export type AccountStateDelta = {
  address: string
  delta: StateDelta
}

// JSON DTO shape for AccountStateDelta with wire keys and JSON-safe primitives
export type AccountStateDeltaDto = {
  address: string
  delta: StateDeltaDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AccountStateDelta): AccountStateDeltaDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['delta']
    if (v === undefined) {
      // omit undefined
    } else {
      out['delta'] = v === undefined ? v : StateDeltaModel.toDto(v)
    }
  }
  return out as AccountStateDeltaDto
}

export function fromDto(dto: AccountStateDeltaDto): AccountStateDelta {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['address'] = v as any
    }
  }
  {
    const v = (dto as any)['delta']
    if (v === undefined) {
      // omit undefined
    } else {
      out['delta'] = v === undefined ? v : StateDeltaModel.fromDto(v)
    }
  }
  return out as AccountStateDelta
}

// Msgpack codecs
export function encodeMsgpack(value: AccountStateDelta): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AccountStateDelta {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AccountStateDelta): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AccountStateDelta {
  return fromDto(raw as AccountStateDeltaDto)
}

// Array helpers
export function encodeMsgpackArray(values: AccountStateDelta[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AccountStateDelta[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AccountStateDelta[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AccountStateDelta[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AccountStateDeltaMsgpackDto = {
  address: string
  delta: ReturnType<(typeof StateDeltaModel)['toMsgpackDto']>
}

function toMsgpackDto(value: AccountStateDelta): AccountStateDeltaMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['delta']
    if (v === undefined) {
    } else {
      out['delta'] = StateDeltaModel.toMsgpackDto(v)
    }
  }
  return out as AccountStateDeltaMsgpackDto
}

function fromMsgpackDto(dto: AccountStateDeltaMsgpackDto): AccountStateDelta {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (dto as any)['delta']
    if (v === undefined) {
    } else {
      out['delta'] = StateDeltaModel.fromMsgpackDto(v)
    }
  }
  return out as AccountStateDelta
}

export const AccountStateDelta = {
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
