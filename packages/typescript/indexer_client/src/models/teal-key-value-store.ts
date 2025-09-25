import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { TealKeyValue, TealKeyValueDto } from './teal-key-value'
import { TealKeyValue as TealKeyValueModel } from './teal-key-value'

/**
 * Represents a key-value store for use in an application.
 */
export type TealKeyValueStore = TealKeyValue[]

// JSON DTO shape for TealKeyValueStore with wire keys and JSON-safe primitives
export type TealKeyValueStoreDto = TealKeyValueStore

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TealKeyValueStore): TealKeyValueStoreDto {
  return value as unknown as TealKeyValueStoreDto
}

export function fromDto(dto: TealKeyValueStoreDto): TealKeyValueStore {
  return dto as unknown as TealKeyValueStore
}

// Msgpack codecs
export function encodeMsgpack(value: TealKeyValueStore): Uint8Array {
  return encodeMsgPack(value as any)
}

export function decodeMsgpack(bytes: Uint8Array): TealKeyValueStore {
  const raw: any = decodeMsgPack(bytes)
  return raw as TealKeyValueStore
}

// JSON codecs
export function encodeJson(value: TealKeyValueStore): unknown {
  return value as unknown
}

export function decodeJson(raw: unknown): TealKeyValueStore {
  return raw as TealKeyValueStore
}

// Array helpers

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)

export const TealKeyValueStore = {
  toDto,
  fromDto,
  encodeMsgpack,
  decodeMsgpack,
  encodeJson,
  decodeJson,
} as const
