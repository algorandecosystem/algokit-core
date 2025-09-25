import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { EvalDelta, EvalDeltaDto } from './eval-delta'
import { EvalDelta as EvalDeltaModel } from './eval-delta'

/**
 * Key-value pairs for StateDelta.
 */
export type EvalDeltaKeyValue = {
  key: string
  value: EvalDelta
}

// JSON DTO shape for EvalDeltaKeyValue with wire keys and JSON-safe primitives
export type EvalDeltaKeyValueDto = {
  key: string
  value: EvalDeltaDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: EvalDeltaKeyValue): EvalDeltaKeyValueDto {
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
      out['value'] = v === undefined ? v : EvalDeltaModel.toDto(v)
    }
  }
  return out as EvalDeltaKeyValueDto
}

export function fromDto(dto: EvalDeltaKeyValueDto): EvalDeltaKeyValue {
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
      out['value'] = v === undefined ? v : EvalDeltaModel.fromDto(v)
    }
  }
  return out as EvalDeltaKeyValue
}

// Msgpack codecs
export function encodeMsgpack(value: EvalDeltaKeyValue): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): EvalDeltaKeyValue {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: EvalDeltaKeyValue): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): EvalDeltaKeyValue {
  return fromDto(raw as EvalDeltaKeyValueDto)
}

// Array helpers
export function encodeMsgpackArray(values: EvalDeltaKeyValue[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): EvalDeltaKeyValue[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: EvalDeltaKeyValue[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): EvalDeltaKeyValue[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type EvalDeltaKeyValueMsgpackDto = {
  key: string
  value: ReturnType<(typeof EvalDeltaModel)['toMsgpackDto']>
}

function toMsgpackDto(value: EvalDeltaKeyValue): EvalDeltaKeyValueMsgpackDto {
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
      out['value'] = EvalDeltaModel.toMsgpackDto(v)
    }
  }
  return out as EvalDeltaKeyValueMsgpackDto
}

function fromMsgpackDto(dto: EvalDeltaKeyValueMsgpackDto): EvalDeltaKeyValue {
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
      out['value'] = EvalDeltaModel.fromMsgpackDto(v)
    }
  }
  return out as EvalDeltaKeyValue
}

export const EvalDeltaKeyValue = {
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
