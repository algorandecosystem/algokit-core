import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Represents a \[apls\] local-state or \[apgs\] global-state schema. These schemas determine how much storage may be used in a local-state or global-state for an application. The more space used, the larger minimum balance must be maintained in the account holding the data.
 */
export type StateSchema = {
  /**
   * Maximum number of TEAL uints that may be stored in the key/value store.
   */
  numUint: bigint

  /**
   * Maximum number of TEAL byte slices that may be stored in the key/value store.
   */
  numByteSlice: bigint
}

// JSON DTO shape for StateSchema with wire keys and JSON-safe primitives
export type StateSchemaDto = {
  'num-uint': bigint
  'num-byte-slice': bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateSchema): StateSchemaDto {
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
  return out as StateSchemaDto
}

export function fromDto(dto: StateSchemaDto): StateSchema {
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
  return out as StateSchema
}

// Msgpack codecs
export function encodeMsgpack(value: StateSchema): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StateSchema {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StateSchema): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StateSchema {
  return fromDto(raw as StateSchemaDto)
}

// Array helpers
export function encodeMsgpackArray(values: StateSchema[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StateSchema[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StateSchema[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StateSchema[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StateSchemaMsgpackDto = {
  'num-uint': bigint
  'num-byte-slice': bigint
}

function toMsgpackDto(value: StateSchema): StateSchemaMsgpackDto {
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
  return out as StateSchemaMsgpackDto
}

function fromMsgpackDto(dto: StateSchemaMsgpackDto): StateSchema {
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
  return out as StateSchema
}

export const StateSchema = {
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
