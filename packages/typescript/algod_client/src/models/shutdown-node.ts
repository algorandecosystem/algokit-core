import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type ShutdownNode = {}

// JSON DTO shape for ShutdownNode with wire keys and JSON-safe primitives
export type ShutdownNodeDto = {}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ShutdownNode): ShutdownNodeDto {
  const out: any = {}
  return out as ShutdownNodeDto
}

export function fromDto(dto: ShutdownNodeDto): ShutdownNode {
  const out: any = {}
  return out as ShutdownNode
}

// Msgpack codecs
export function encodeMsgpack(value: ShutdownNode): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ShutdownNode {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ShutdownNode): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ShutdownNode {
  return fromDto(raw as ShutdownNodeDto)
}

// Array helpers
export function encodeMsgpackArray(values: ShutdownNode[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ShutdownNode[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ShutdownNode[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ShutdownNode[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ShutdownNodeMsgpackDto = {}

function toMsgpackDto(value: ShutdownNode): ShutdownNodeMsgpackDto {
  const out: any = {}
  return out as ShutdownNodeMsgpackDto
}

function fromMsgpackDto(dto: ShutdownNodeMsgpackDto): ShutdownNode {
  const out: any = {}
  return out as ShutdownNode
}

export const ShutdownNode = {
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
