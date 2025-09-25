import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * The type of hash function used to create the proof, must be one of:
 * * sha512_256
 * * sha256
 */
export type Hashtype = 'sha512_256' | 'sha256'

// JSON DTO shape for Hashtype with wire keys and JSON-safe primitives
export type HashtypeDto = Hashtype

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: Hashtype): HashtypeDto {
  return value as unknown as HashtypeDto
}

export function fromDto(dto: HashtypeDto): Hashtype {
  return dto as unknown as Hashtype
}

// Msgpack codecs
export function encodeMsgpack(value: Hashtype): Uint8Array {
  return encodeMsgPack(value as any)
}

export function decodeMsgpack(bytes: Uint8Array): Hashtype {
  const raw: any = decodeMsgPack(bytes)
  return raw as Hashtype
}

// JSON codecs
export function encodeJson(value: Hashtype): unknown {
  return value as unknown
}

export function decodeJson(raw: unknown): Hashtype {
  return raw as Hashtype
}

// Array helpers

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)

export const Hashtype = {
  toDto,
  fromDto,
  encodeMsgpack,
  decodeMsgpack,
  encodeJson,
  decodeJson,
} as const
