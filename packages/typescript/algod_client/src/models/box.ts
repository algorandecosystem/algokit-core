import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Box name and its content.
 */
export type Box = {
  /**
   * The round for which this information is relevant
   */
  round: bigint

  /**
   * The box name, base64 encoded
   */
  name: Uint8Array

  /**
   * The box value, base64 encoded.
   */
  value: Uint8Array
}

// JSON DTO shape for Box with wire keys and JSON-safe primitives
export type BoxDto = {
  round: string
  name: string
  value: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: Box): BoxDto {
  const out: any = {}
  {
    const v = (value as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['name']
    if (v === undefined) {
      // omit undefined
    } else {
      out['name'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['value']
    if (v === undefined) {
      // omit undefined
    } else {
      out['value'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as BoxDto
}

export function fromDto(dto: BoxDto): Box {
  const out: any = {}
  {
    const v = (dto as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['name']
    if (v === undefined) {
      // omit undefined
    } else {
      out['name'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['value']
    if (v === undefined) {
      // omit undefined
    } else {
      out['value'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as Box
}

// Msgpack codecs
export function encodeMsgpack(value: Box): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): Box {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: Box): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): Box {
  return fromDto(raw as BoxDto)
}

// Array helpers
export function encodeMsgpackArray(values: Box[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): Box[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: Box[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): Box[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type BoxMsgpackDto = {
  round: bigint
  name: Uint8Array
  value: Uint8Array
}

function toMsgpackDto(value: Box): BoxMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (value as any)['name']
    if (v === undefined) {
    } else {
      out['name'] = v
    }
  }
  {
    const v = (value as any)['value']
    if (v === undefined) {
    } else {
      out['value'] = v
    }
  }
  return out as BoxMsgpackDto
}

function fromMsgpackDto(dto: BoxMsgpackDto): Box {
  const out: any = {}
  {
    const v = (dto as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (dto as any)['name']
    if (v === undefined) {
    } else {
      out['name'] = v
    }
  }
  {
    const v = (dto as any)['value']
    if (v === undefined) {
    } else {
      out['value'] = v
    }
  }
  return out as Box
}

export const Box = {
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
