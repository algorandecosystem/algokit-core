import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Box descriptor describes an app box without a value.
 */
export type BoxDescriptor = {
  /**
   * Base64 encoded box name
   */
  name: Uint8Array
}

// JSON DTO shape for BoxDescriptor with wire keys and JSON-safe primitives
export type BoxDescriptorDto = {
  name: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: BoxDescriptor): BoxDescriptorDto {
  const out: any = {}
  {
    const v = (value as any)['name']
    if (v === undefined) {
      // omit undefined
    } else {
      out['name'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as BoxDescriptorDto
}

export function fromDto(dto: BoxDescriptorDto): BoxDescriptor {
  const out: any = {}
  {
    const v = (dto as any)['name']
    if (v === undefined) {
      // omit undefined
    } else {
      out['name'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as BoxDescriptor
}

// Msgpack codecs
export function encodeMsgpack(value: BoxDescriptor): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): BoxDescriptor {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: BoxDescriptor): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): BoxDescriptor {
  return fromDto(raw as BoxDescriptorDto)
}

// Array helpers
export function encodeMsgpackArray(values: BoxDescriptor[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): BoxDescriptor[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: BoxDescriptor[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): BoxDescriptor[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type BoxDescriptorMsgpackDto = {
  name: Uint8Array
}

function toMsgpackDto(value: BoxDescriptor): BoxDescriptorMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['name']
    if (v === undefined) {
    } else {
      out['name'] = v
    }
  }
  return out as BoxDescriptorMsgpackDto
}

function fromMsgpackDto(dto: BoxDescriptorMsgpackDto): BoxDescriptor {
  const out: any = {}
  {
    const v = (dto as any)['name']
    if (v === undefined) {
    } else {
      out['name'] = v
    }
  }
  return out as BoxDescriptor
}

export const BoxDescriptor = {
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
