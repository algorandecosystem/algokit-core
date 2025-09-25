import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * LocalsRef names a local state by referring to an Address and App it belongs to.
 */
export type LocalsRef = {
  /**
   * \[d\] Address in access list, or the sender of the transaction.
   */
  address: string

  /**
   * \[p\] Application ID for app in access list, or zero if referring to the called application.
   */
  app: bigint
}

// JSON DTO shape for LocalsRef with wire keys and JSON-safe primitives
export type LocalsRefDto = {
  address: string
  app: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LocalsRef): LocalsRefDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['app']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app'] = v
    }
  }
  return out as LocalsRefDto
}

export function fromDto(dto: LocalsRefDto): LocalsRef {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['address'] = v as any
    }
  }
  {
    const v = (dto as any)['app']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app'] = v as any
    }
  }
  return out as LocalsRef
}

// Msgpack codecs
export function encodeMsgpack(value: LocalsRef): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LocalsRef {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LocalsRef): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LocalsRef {
  return fromDto(raw as LocalsRefDto)
}

// Array helpers
export function encodeMsgpackArray(values: LocalsRef[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LocalsRef[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LocalsRef[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LocalsRef[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LocalsRefMsgpackDto = {
  address: string
  app: bigint
}

function toMsgpackDto(value: LocalsRef): LocalsRefMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['app']
    if (v === undefined) {
    } else {
      out['app'] = v
    }
  }
  return out as LocalsRefMsgpackDto
}

function fromMsgpackDto(dto: LocalsRefMsgpackDto): LocalsRef {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (dto as any)['app']
    if (v === undefined) {
    } else {
      out['app'] = v
    }
  }
  return out as LocalsRef
}

export const LocalsRef = {
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
