import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type TealDisassemble = {
  /**
   * disassembled Teal code
   */
  result: string
}

// JSON DTO shape for TealDisassemble with wire keys and JSON-safe primitives
export type TealDisassembleDto = {
  result: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TealDisassemble): TealDisassembleDto {
  const out: any = {}
  {
    const v = (value as any)['result']
    if (v === undefined) {
      // omit undefined
    } else {
      out['result'] = v
    }
  }
  return out as TealDisassembleDto
}

export function fromDto(dto: TealDisassembleDto): TealDisassemble {
  const out: any = {}
  {
    const v = (dto as any)['result']
    if (v === undefined) {
      // omit undefined
    } else {
      out['result'] = v as any
    }
  }
  return out as TealDisassemble
}

// Msgpack codecs
export function encodeMsgpack(value: TealDisassemble): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TealDisassemble {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TealDisassemble): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TealDisassemble {
  return fromDto(raw as TealDisassembleDto)
}

// Array helpers
export function encodeMsgpackArray(values: TealDisassemble[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TealDisassemble[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TealDisassemble[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TealDisassemble[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TealDisassembleMsgpackDto = {
  result: string
}

function toMsgpackDto(value: TealDisassemble): TealDisassembleMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['result']
    if (v === undefined) {
    } else {
      out['result'] = v
    }
  }
  return out as TealDisassembleMsgpackDto
}

function fromMsgpackDto(dto: TealDisassembleMsgpackDto): TealDisassemble {
  const out: any = {}
  {
    const v = (dto as any)['result']
    if (v === undefined) {
    } else {
      out['result'] = v
    }
  }
  return out as TealDisassemble
}

export const TealDisassemble = {
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
