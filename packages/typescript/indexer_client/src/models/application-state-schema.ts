import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Specifies maximums on the number of each type that may be stored.
 */
export type ApplicationStateSchema = {
  /**
   * number of uints.
   */
  numUint: bigint

  /**
   * number of byte slices.
   */
  numByteSlice: bigint
}

// JSON DTO shape for ApplicationStateSchema with wire keys and JSON-safe primitives
export type ApplicationStateSchemaDto = {
  'num-uint': bigint
  'num-byte-slice': bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ApplicationStateSchema): ApplicationStateSchemaDto {
  const out: any = {}
  {
    const v = (value as any)['numUint']
    if (v === undefined) {
      // omit undefined
    } else {
      out['num-uint'] = v
    }
  }
  {
    const v = (value as any)['numByteSlice']
    if (v === undefined) {
      // omit undefined
    } else {
      out['num-byte-slice'] = v
    }
  }
  return out as ApplicationStateSchemaDto
}

export function fromDto(dto: ApplicationStateSchemaDto): ApplicationStateSchema {
  const out: any = {}
  {
    const v = (dto as any)['num-uint']
    if (v === undefined) {
      // omit undefined
    } else {
      out['numUint'] = v as any
    }
  }
  {
    const v = (dto as any)['num-byte-slice']
    if (v === undefined) {
      // omit undefined
    } else {
      out['numByteSlice'] = v as any
    }
  }
  return out as ApplicationStateSchema
}

// Msgpack codecs
export function encodeMsgpack(value: ApplicationStateSchema): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ApplicationStateSchema {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ApplicationStateSchema): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ApplicationStateSchema {
  return fromDto(raw as ApplicationStateSchemaDto)
}

// Array helpers
export function encodeMsgpackArray(values: ApplicationStateSchema[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ApplicationStateSchema[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ApplicationStateSchema[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ApplicationStateSchema[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ApplicationStateSchemaMsgpackDto = {
  'num-uint': bigint
  'num-byte-slice': bigint
}

function toMsgpackDto(value: ApplicationStateSchema): ApplicationStateSchemaMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['numUint']
    if (v === undefined) {
    } else {
      out['num-uint'] = v
    }
  }
  {
    const v = (value as any)['numByteSlice']
    if (v === undefined) {
    } else {
      out['num-byte-slice'] = v
    }
  }
  return out as ApplicationStateSchemaMsgpackDto
}

function fromMsgpackDto(dto: ApplicationStateSchemaMsgpackDto): ApplicationStateSchema {
  const out: any = {}
  {
    const v = (dto as any)['num-uint']
    if (v === undefined) {
    } else {
      out['numUint'] = v
    }
  }
  {
    const v = (dto as any)['num-byte-slice']
    if (v === undefined) {
    } else {
      out['numByteSlice'] = v
    }
  }
  return out as ApplicationStateSchema
}

export const ApplicationStateSchema = {
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
