import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * An catchpoint start response.
 */
export type StartCatchup = {
  /**
   * Catchup start response string
   */
  catchupMessage: string
}

// JSON DTO shape for StartCatchup with wire keys and JSON-safe primitives
export type StartCatchupDto = {
  'catchup-message': string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StartCatchup): StartCatchupDto {
  const out: any = {}
  {
    const v = (value as any)['catchupMessage']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchup-message'] = v
    }
  }
  return out as StartCatchupDto
}

export function fromDto(dto: StartCatchupDto): StartCatchup {
  const out: any = {}
  {
    const v = (dto as any)['catchup-message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['catchupMessage'] = v as any
    }
  }
  return out as StartCatchup
}

// Msgpack codecs
export function encodeMsgpack(value: StartCatchup): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StartCatchup {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StartCatchup): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StartCatchup {
  return fromDto(raw as StartCatchupDto)
}

// Array helpers
export function encodeMsgpackArray(values: StartCatchup[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StartCatchup[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StartCatchup[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StartCatchup[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StartCatchupMsgpackDto = {
  'catchup-message': string
}

function toMsgpackDto(value: StartCatchup): StartCatchupMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['catchupMessage']
    if (v === undefined) {
    } else {
      out['catchup-message'] = v
    }
  }
  return out as StartCatchupMsgpackDto
}

function fromMsgpackDto(dto: StartCatchupMsgpackDto): StartCatchup {
  const out: any = {}
  {
    const v = (dto as any)['catchup-message']
    if (v === undefined) {
    } else {
      out['catchupMessage'] = v
    }
  }
  return out as StartCatchup
}

export const StartCatchup = {
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
