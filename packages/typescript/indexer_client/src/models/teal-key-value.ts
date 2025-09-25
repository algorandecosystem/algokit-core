import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { TealValue, TealValueDto } from './teal-value'
import { TealValue as TealValueModel } from './teal-value'

/**
 * Represents a key-value pair in an application store.
 */
export type TealKeyValue = {
  key: string
  value: TealValue
}

// JSON DTO shape for TealKeyValue with wire keys and JSON-safe primitives
export type TealKeyValueDto = {
  key: string
  value: TealValueDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TealKeyValue): TealKeyValueDto {
  const out: any = {}
  {
    const v = (value as any)['key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['key'] = v
    }
  }
  {
    const v = (value as any)['value']
    if (v === undefined) {
      // omit undefined
    } else {
      out['value'] = v === undefined ? v : TealValueModel.toDto(v)
    }
  }
  return out as TealKeyValueDto
}

export function fromDto(dto: TealKeyValueDto): TealKeyValue {
  const out: any = {}
  {
    const v = (dto as any)['key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['key'] = v as any
    }
  }
  {
    const v = (dto as any)['value']
    if (v === undefined) {
      // omit undefined
    } else {
      out['value'] = v === undefined ? v : TealValueModel.fromDto(v)
    }
  }
  return out as TealKeyValue
}

// Msgpack codecs
export function encodeMsgpack(value: TealKeyValue): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TealKeyValue {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TealKeyValue): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TealKeyValue {
  return fromDto(raw as TealKeyValueDto)
}

// Array helpers
export function encodeMsgpackArray(values: TealKeyValue[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TealKeyValue[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TealKeyValue[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TealKeyValue[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TealKeyValueMsgpackDto = {
  key: string
  value: ReturnType<(typeof TealValueModel)['toMsgpackDto']>
}

function toMsgpackDto(value: TealKeyValue): TealKeyValueMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['key']
    if (v === undefined) {
    } else {
      out['key'] = v
    }
  }
  {
    const v = (value as any)['value']
    if (v === undefined) {
    } else {
      out['value'] = TealValueModel.toMsgpackDto(v)
    }
  }
  return out as TealKeyValueMsgpackDto
}

function fromMsgpackDto(dto: TealKeyValueMsgpackDto): TealKeyValue {
  const out: any = {}
  {
    const v = (dto as any)['key']
    if (v === undefined) {
    } else {
      out['key'] = v
    }
  }
  {
    const v = (dto as any)['value']
    if (v === undefined) {
    } else {
      out['value'] = TealValueModel.fromMsgpackDto(v)
    }
  }
  return out as TealKeyValue
}

export const TealKeyValue = {
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
