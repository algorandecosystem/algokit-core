import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type AddParticipationKey = {
  /**
   * encoding of the participation ID.
   */
  partId: string
}

// JSON DTO shape for AddParticipationKey with wire keys and JSON-safe primitives
export type AddParticipationKeyDto = {
  partId: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AddParticipationKey): AddParticipationKeyDto {
  const out: any = {}
  {
    const v = (value as any)['partId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['partId'] = v
    }
  }
  return out as AddParticipationKeyDto
}

export function fromDto(dto: AddParticipationKeyDto): AddParticipationKey {
  const out: any = {}
  {
    const v = (dto as any)['partId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['partId'] = v as any
    }
  }
  return out as AddParticipationKey
}

// Msgpack codecs
export function encodeMsgpack(value: AddParticipationKey): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AddParticipationKey {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AddParticipationKey): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AddParticipationKey {
  return fromDto(raw as AddParticipationKeyDto)
}

// Array helpers
export function encodeMsgpackArray(values: AddParticipationKey[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AddParticipationKey[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AddParticipationKey[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AddParticipationKey[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AddParticipationKeyMsgpackDto = {
  partId: string
}

function toMsgpackDto(value: AddParticipationKey): AddParticipationKeyMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['partId']
    if (v === undefined) {
    } else {
      out['partId'] = v
    }
  }
  return out as AddParticipationKeyMsgpackDto
}

function fromMsgpackDto(dto: AddParticipationKeyMsgpackDto): AddParticipationKey {
  const out: any = {}
  {
    const v = (dto as any)['partId']
    if (v === undefined) {
    } else {
      out['partId'] = v
    }
  }
  return out as AddParticipationKey
}

export const AddParticipationKey = {
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
