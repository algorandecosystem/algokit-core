import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AvmValue, AvmValueDto } from './avm-value'
import { AvmValue as AvmValueModel } from './avm-value'

/**
 * Represents an AVM key-value pair in an application store.
 */
export type AvmKeyValue = {
  key: Uint8Array
  value: AvmValue
}

// JSON DTO shape for AvmKeyValue with wire keys and JSON-safe primitives
export type AvmKeyValueDto = {
  key: string
  value: AvmValueDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AvmKeyValue): AvmKeyValueDto {
  const out: any = {}
  {
    const v = (value as any)['key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['key'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['value']
    if (v === undefined) {
      // omit undefined
    } else {
      out['value'] = v === undefined ? v : AvmValueModel.toDto(v)
    }
  }
  return out as AvmKeyValueDto
}

export function fromDto(dto: AvmKeyValueDto): AvmKeyValue {
  const out: any = {}
  {
    const v = (dto as any)['key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['key'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['value']
    if (v === undefined) {
      // omit undefined
    } else {
      out['value'] = v === undefined ? v : AvmValueModel.fromDto(v)
    }
  }
  return out as AvmKeyValue
}

// Msgpack codecs
export function encodeMsgpack(value: AvmKeyValue): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AvmKeyValue {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AvmKeyValue): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AvmKeyValue {
  return fromDto(raw as AvmKeyValueDto)
}

// Array helpers
export function encodeMsgpackArray(values: AvmKeyValue[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AvmKeyValue[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AvmKeyValue[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AvmKeyValue[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AvmKeyValueMsgpackDto = {
  key: Uint8Array
  value: ReturnType<(typeof AvmValueModel)['toMsgpackDto']>
}

function toMsgpackDto(value: AvmKeyValue): AvmKeyValueMsgpackDto {
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
      out['value'] = AvmValueModel.toMsgpackDto(v)
    }
  }
  return out as AvmKeyValueMsgpackDto
}

function fromMsgpackDto(dto: AvmKeyValueMsgpackDto): AvmKeyValue {
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
      out['value'] = AvmValueModel.fromMsgpackDto(v)
    }
  }
  return out as AvmKeyValue
}

export const AvmKeyValue = {
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
