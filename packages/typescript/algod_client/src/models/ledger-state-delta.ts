import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Ledger StateDelta object
 */
export type LedgerStateDelta = {}

// JSON DTO shape for LedgerStateDelta with wire keys and JSON-safe primitives
export type LedgerStateDeltaDto = {}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LedgerStateDelta): LedgerStateDeltaDto {
  const out: any = {}
  return out as LedgerStateDeltaDto
}

export function fromDto(dto: LedgerStateDeltaDto): LedgerStateDelta {
  const out: any = {}
  return out as LedgerStateDelta
}

// Msgpack codecs
export function encodeMsgpack(value: LedgerStateDelta): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LedgerStateDelta {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LedgerStateDelta): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LedgerStateDelta {
  return fromDto(raw as LedgerStateDeltaDto)
}

// Array helpers
export function encodeMsgpackArray(values: LedgerStateDelta[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LedgerStateDelta[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LedgerStateDelta[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LedgerStateDelta[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LedgerStateDeltaMsgpackDto = {}

function toMsgpackDto(value: LedgerStateDelta): LedgerStateDeltaMsgpackDto {
  const out: any = {}
  return out as LedgerStateDeltaMsgpackDto
}

function fromMsgpackDto(dto: LedgerStateDeltaMsgpackDto): LedgerStateDelta {
  const out: any = {}
  return out as LedgerStateDelta
}

export const LedgerStateDelta = {
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
