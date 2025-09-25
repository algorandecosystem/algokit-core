import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { EvalDeltaKeyValue, EvalDeltaKeyValueDto } from './eval-delta-key-value'
import { EvalDeltaKeyValue as EvalDeltaKeyValueModel } from './eval-delta-key-value'

/**
 * Application state delta.
 */
export type StateDelta = EvalDeltaKeyValue[]

// JSON DTO shape for StateDelta with wire keys and JSON-safe primitives
export type StateDeltaDto = StateDelta

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateDelta): StateDeltaDto {
  return value as unknown as StateDeltaDto
}

export function fromDto(dto: StateDeltaDto): StateDelta {
  return dto as unknown as StateDelta
}

// Msgpack codecs
export function encodeMsgpack(value: StateDelta): Uint8Array {
  return encodeMsgPack(value as any)
}

export function decodeMsgpack(bytes: Uint8Array): StateDelta {
  const raw: any = decodeMsgPack(bytes)
  return raw as StateDelta
}

// JSON codecs
export function encodeJson(value: StateDelta): unknown {
  return value as unknown
}

export function decodeJson(raw: unknown): StateDelta {
  return raw as StateDelta
}

// Array helpers

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)

export const StateDelta = {
  toDto,
  fromDto,
  encodeMsgpack,
  decodeMsgpack,
  encodeJson,
  decodeJson,
} as const
