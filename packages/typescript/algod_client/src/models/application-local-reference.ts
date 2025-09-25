import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * References an account's local state for an application.
 */
export type ApplicationLocalReference = {
  /**
   * Address of the account with the local state.
   */
  account: string

  /**
   * Application ID of the local state application.
   */
  app: bigint
}

// JSON DTO shape for ApplicationLocalReference with wire keys and JSON-safe primitives
export type ApplicationLocalReferenceDto = {
  account: string
  app: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ApplicationLocalReference): ApplicationLocalReferenceDto {
  const out: any = {}
  {
    const v = (value as any)['account']
    if (v === undefined) {
      // omit undefined
    } else {
      out['account'] = v
    }
  }
  {
    const v = (value as any)['app']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as ApplicationLocalReferenceDto
}

export function fromDto(dto: ApplicationLocalReferenceDto): ApplicationLocalReference {
  const out: any = {}
  {
    const v = (dto as any)['account']
    if (v === undefined) {
      // omit undefined
    } else {
      out['account'] = v as any
    }
  }
  {
    const v = (dto as any)['app']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as ApplicationLocalReference
}

// Msgpack codecs
export function encodeMsgpack(value: ApplicationLocalReference): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ApplicationLocalReference {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ApplicationLocalReference): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ApplicationLocalReference {
  return fromDto(raw as ApplicationLocalReferenceDto)
}

// Array helpers
export function encodeMsgpackArray(values: ApplicationLocalReference[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ApplicationLocalReference[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ApplicationLocalReference[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ApplicationLocalReference[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ApplicationLocalReferenceMsgpackDto = {
  account: string
  app: bigint
}

function toMsgpackDto(value: ApplicationLocalReference): ApplicationLocalReferenceMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['account']
    if (v === undefined) {
    } else {
      out['account'] = v
    }
  }
  {
    const v = (value as any)['app']
    if (v === undefined) {
    } else {
      out['app'] = v
    }
  }
  return out as ApplicationLocalReferenceMsgpackDto
}

function fromMsgpackDto(dto: ApplicationLocalReferenceMsgpackDto): ApplicationLocalReference {
  const out: any = {}
  {
    const v = (dto as any)['account']
    if (v === undefined) {
    } else {
      out['account'] = v
    }
  }
  {
    const v = (dto as any)['app']
    if (v === undefined) {
    } else {
      out['app'] = v
    }
  }
  return out as ApplicationLocalReference
}

export const ApplicationLocalReference = {
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
