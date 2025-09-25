import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * An error response with optional data field.
 */
export type ErrorResponse = {
  data?: {}
  message: string
}

// JSON DTO shape for ErrorResponse with wire keys and JSON-safe primitives
export type ErrorResponseDto = {
  data?: {}
  message: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ErrorResponse): ErrorResponseDto {
  const out: any = {}
  {
    const v = (value as any)['data']
    if (v === undefined) {
      // omit undefined
    } else {
      out['data'] = v
    }
  }
  {
    const v = (value as any)['message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['message'] = v
    }
  }
  return out as ErrorResponseDto
}

export function fromDto(dto: ErrorResponseDto): ErrorResponse {
  const out: any = {}
  {
    const v = (dto as any)['data']
    if (v === undefined) {
      // omit undefined
    } else {
      out['data'] = v as any
    }
  }
  {
    const v = (dto as any)['message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['message'] = v as any
    }
  }
  return out as ErrorResponse
}

// Msgpack codecs
export function encodeMsgpack(value: ErrorResponse): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ErrorResponse {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ErrorResponse): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ErrorResponse {
  return fromDto(raw as ErrorResponseDto)
}

// Array helpers
export function encodeMsgpackArray(values: ErrorResponse[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ErrorResponse[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ErrorResponse[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ErrorResponse[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ErrorResponseMsgpackDto = {
  data?: {}
  message: string
}

function toMsgpackDto(value: ErrorResponse): ErrorResponseMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['data']
    if (v === undefined) {
    } else {
      out['data'] = v
    }
  }
  {
    const v = (value as any)['message']
    if (v === undefined) {
    } else {
      out['message'] = v
    }
  }
  return out as ErrorResponseMsgpackDto
}

function fromMsgpackDto(dto: ErrorResponseMsgpackDto): ErrorResponse {
  const out: any = {}
  {
    const v = (dto as any)['data']
    if (v === undefined) {
    } else {
      out['data'] = v
    }
  }
  {
    const v = (dto as any)['message']
    if (v === undefined) {
    } else {
      out['message'] = v
    }
  }
  return out as ErrorResponse
}

export const ErrorResponse = {
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
