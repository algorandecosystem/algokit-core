import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Represents a TEAL value.
 */
export type TealValue = {
  /**
   * type of the value. Value `1` refers to **bytes**, value `2` refers to **uint**
   */
  type: bigint

  /**
   * bytes value.
   */
  bytes: Uint8Array

  /**
   * uint value.
   */
  uint: bigint
}

// JSON DTO shape for TealValue with wire keys and JSON-safe primitives
export type TealValueDto = {
  type: bigint
  bytes: string
  uint: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TealValue): TealValueDto {
  const out: any = {}
  {
    const v = (value as any)['type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['type'] = v
    }
  }
  {
    const v = (value as any)['bytes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['bytes'] = v === undefined ? v : toBase64(v as Uint8Array)
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
  return out as TealValueDto
}

export function fromDto(dto: TealValueDto): TealValue {
  const out: any = {}
  {
    const v = (dto as any)['type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['type'] = v as any
    }
  }
  {
    const v = (dto as any)['bytes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['bytes'] = v === undefined ? v : fromBase64(v as string)
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
  return out as TealValue
}

// Msgpack codecs
export function encodeMsgpack(value: TealValue): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TealValue {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TealValue): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TealValue {
  return fromDto(raw as TealValueDto)
}

// Array helpers
export function encodeMsgpackArray(values: TealValue[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TealValue[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TealValue[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TealValue[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TealValueMsgpackDto = {
  type: bigint
  bytes: Uint8Array
  uint: bigint
}

function toMsgpackDto(value: TealValue): TealValueMsgpackDto {
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
  return out as TealValueMsgpackDto
}

function fromMsgpackDto(dto: TealValueMsgpackDto): TealValue {
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
  return out as TealValue
}

export const TealValue = {
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
