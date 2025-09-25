import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * An catchpoint abort response.
 */
export type AbortCatchup = {
  /**
   * Catchup abort response string
   */
  catchupMessage: string
}

// JSON DTO shape for AbortCatchup with wire keys and JSON-safe primitives
export type AbortCatchupDto = {
  'catchup-message': string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AbortCatchup): AbortCatchupDto {
  const out: any = {}
  {
    const v = (value as any)['catchupMessage']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchup-message'] = v
    }
  }
  return out as AbortCatchupDto
}

export function fromDto(dto: AbortCatchupDto): AbortCatchup {
  const out: any = {}
  {
    const v = (dto as any)['catchup-message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchupMessage'] = v as any
    }
  }
  return out as AbortCatchup
}

// Msgpack codecs
export function encodeMsgpack(value: AbortCatchup): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AbortCatchup {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AbortCatchup): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AbortCatchup {
  return fromDto(raw as AbortCatchupDto)
}

// Array helpers
export function encodeMsgpackArray(values: AbortCatchup[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AbortCatchup[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AbortCatchup[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AbortCatchup[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AbortCatchupMsgpackDto = {
  'catchup-message': string
}

function toMsgpackDto(value: AbortCatchup): AbortCatchupMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['catchupMessage']
    if (v === undefined) {
    } else {
      out['catchup-message'] = v
    }
  }
  return out as AbortCatchupMsgpackDto
}

function fromMsgpackDto(dto: AbortCatchupMsgpackDto): AbortCatchup {
  const out: any = {}
  {
    const v = (dto as any)['catchup-message']
    if (v === undefined) {
    } else {
      out['catchupMessage'] = v
    }
  }
  return out as AbortCatchup
}

export const AbortCatchup = {
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
