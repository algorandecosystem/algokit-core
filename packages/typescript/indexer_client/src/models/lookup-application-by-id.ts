import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Application, ApplicationDto } from './application'
import { Application as ApplicationModel } from './application'

export type LookupApplicationById = {
  application?: Application

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint
}

// JSON DTO shape for LookupApplicationById with wire keys and JSON-safe primitives
export type LookupApplicationByIdDto = {
  application?: ApplicationDto
  'current-round': bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LookupApplicationById): LookupApplicationByIdDto {
  const out: any = {}
  {
    const v = (value as any)['application']
    if (v === undefined) {
      // omit undefined
    } else {
      out['application'] = v === undefined ? v : ApplicationModel.toDto(v)
    }
  }
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['current-round'] = v
    }
  }
  return out as LookupApplicationByIdDto
}

export function fromDto(dto: LookupApplicationByIdDto): LookupApplicationById {
  const out: any = {}
  {
    const v = (dto as any)['application']
    if (v === undefined) {
      // omit undefined
    } else {
      out['application'] = v === undefined ? v : ApplicationModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['currentRound'] = v as any
    }
  }
  return out as LookupApplicationById
}

// Msgpack codecs
export function encodeMsgpack(value: LookupApplicationById): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LookupApplicationById {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LookupApplicationById): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LookupApplicationById {
  return fromDto(raw as LookupApplicationByIdDto)
}

// Array helpers
export function encodeMsgpackArray(values: LookupApplicationById[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LookupApplicationById[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LookupApplicationById[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LookupApplicationById[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LookupApplicationByIdMsgpackDto = {
  application?: ReturnType<(typeof ApplicationModel)['toMsgpackDto']>
  'current-round': bigint
}

function toMsgpackDto(value: LookupApplicationById): LookupApplicationByIdMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['application']
    if (v === undefined) {
    } else {
      out['application'] = ApplicationModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
    } else {
      out['current-round'] = v
    }
  }
  return out as LookupApplicationByIdMsgpackDto
}

function fromMsgpackDto(dto: LookupApplicationByIdMsgpackDto): LookupApplicationById {
  const out: any = {}
  {
    const v = (dto as any)['application']
    if (v === undefined) {
    } else {
      out['application'] = ApplicationModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
    } else {
      out['currentRound'] = v
    }
  }
  return out as LookupApplicationById
}

export const LookupApplicationById = {
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
