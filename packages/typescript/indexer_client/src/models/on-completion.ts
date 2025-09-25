import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * \[apan\] defines the what additional actions occur with the transaction.
 *
 * Valid types:
 * * noop
 * * optin
 * * closeout
 * * clear
 * * update
 * * delete
 */
export type OnCompletion = 'noop' | 'optin' | 'closeout' | 'clear' | 'update' | 'delete'

// JSON DTO shape for OnCompletion with wire keys and JSON-safe primitives
export type OnCompletionDto = OnCompletion

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: OnCompletion): OnCompletionDto {
  return value as unknown as OnCompletionDto
}

export function fromDto(dto: OnCompletionDto): OnCompletion {
  return dto as unknown as OnCompletion
}

// Msgpack codecs
export function encodeMsgpack(value: OnCompletion): Uint8Array {
  return encodeMsgPack(value as any)
}

export function decodeMsgpack(bytes: Uint8Array): OnCompletion {
  const raw: any = decodeMsgPack(bytes)
  return raw as OnCompletion
}

// JSON codecs
export function encodeJson(value: OnCompletion): unknown {
  return value as unknown
}

export function decodeJson(raw: unknown): OnCompletion {
  return raw as OnCompletion
}

// Array helpers

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)

export const OnCompletion = {
  toDto,
  fromDto,
  encodeMsgpack,
  decodeMsgpack,
  encodeJson,
  decodeJson,
} as const
