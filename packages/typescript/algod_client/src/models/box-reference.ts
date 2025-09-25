import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * References a box of an application.
 */
export type BoxReference = {
  /**
   * Application ID which this box belongs to
   */
  app: bigint

  /**
   * Base64 encoded box name
   */
  name: Uint8Array
}

// JSON DTO shape for BoxReference with wire keys and JSON-safe primitives
export type BoxReferenceDto = {
  app: bigint
  name: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: BoxReference): BoxReferenceDto {
  const out: any = {}
  {
    const v = (value as any)['app']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app'] = v
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
  return out as BoxReferenceDto
}

export function fromDto(dto: BoxReferenceDto): BoxReference {
  const out: any = {}
  {
    const v = (dto as any)['app']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app'] = v as any
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
  return out as BoxReference
}

// Msgpack codecs
export function encodeMsgpack(value: BoxReference): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): BoxReference {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: BoxReference): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): BoxReference {
  return fromDto(raw as BoxReferenceDto)
}

// Array helpers
export function encodeMsgpackArray(values: BoxReference[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): BoxReference[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: BoxReference[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): BoxReference[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type BoxReferenceMsgpackDto = {
  app: bigint
  name: Uint8Array
}

function toMsgpackDto(value: BoxReference): BoxReferenceMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['app']
    if (v === undefined) {
    } else {
      out['app'] = v
    }
  }
  {
    const v = (value as any)['name']
    if (v === undefined) {
    } else {
      out['name'] = v
    }
  }
  return out as BoxReferenceMsgpackDto
}

function fromMsgpackDto(dto: BoxReferenceMsgpackDto): BoxReference {
  const out: any = {}
  {
    const v = (dto as any)['app']
    if (v === undefined) {
    } else {
      out['app'] = v
    }
  }
  {
    const v = (dto as any)['name']
    if (v === undefined) {
    } else {
      out['name'] = v
    }
  }
  return out as BoxReference
}

export const BoxReference = {
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
