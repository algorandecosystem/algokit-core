import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Represents an AVM value.
 */
export type AvmValue = {
  /**
   * value type. Value `1` refers to **bytes**, value `2` refers to **uint64**
   */
  type: number

  /**
   * bytes value.
   */
  bytes?: string

  /**
   * uint value.
   */
  uint?: bigint
}

// JSON DTO shape for AvmValue with wire keys and JSON-safe primitives
export type AvmValueDto = {
  type: string
  bytes?: string
  uint?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AvmValue): AvmValueDto {
  const out: any = {}
  {
    const v = (value as any)['type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['type'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['bytes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['bytes'] = v
    }
  }
  {
    const v = (value as any)['uint']
    if (v === undefined) {
      // omit undefined
    } else {
      out['uint'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as AvmValueDto
}

export function fromDto(dto: AvmValueDto): AvmValue {
  const out: any = {}
  {
    const v = (dto as any)['type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['type'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['bytes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['bytes'] = v as any
    }
  }
  {
    const v = (dto as any)['uint']
    if (v === undefined) {
      // omit undefined
    } else {
      out['uint'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as AvmValue
}

// Msgpack codecs
export function encodeMsgpack(value: AvmValue): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AvmValue {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AvmValue): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AvmValue {
  return fromDto(raw as AvmValueDto)
}

// Array helpers
export function encodeMsgpackArray(values: AvmValue[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AvmValue[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AvmValue[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AvmValue[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AvmValueMsgpackDto = {
  type: number
  bytes?: string
  uint?: bigint
}

function toMsgpackDto(value: AvmValue): AvmValueMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['type']
    if (v === undefined) {
    } else {
      out['type'] = v
    }
  }
  {
    const v = (value as any)['bytes']
    if (v === undefined) {
    } else {
      out['bytes'] = v
    }
  }
  {
    const v = (value as any)['uint']
    if (v === undefined) {
    } else {
      out['uint'] = v
    }
  }
  return out as AvmValueMsgpackDto
}

function fromMsgpackDto(dto: AvmValueMsgpackDto): AvmValue {
  const out: any = {}
  {
    const v = (dto as any)['type']
    if (v === undefined) {
    } else {
      out['type'] = v
    }
  }
  {
    const v = (dto as any)['bytes']
    if (v === undefined) {
    } else {
      out['bytes'] = v
    }
  }
  {
    const v = (dto as any)['uint']
    if (v === undefined) {
    } else {
      out['uint'] = v
    }
  }
  return out as AvmValue
}

export const AvmValue = {
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
