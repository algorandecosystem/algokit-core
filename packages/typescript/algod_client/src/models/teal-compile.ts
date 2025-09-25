import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type TealCompile = {
  /**
   * base32 SHA512_256 of program bytes (Address style)
   */
  hash: string

  /**
   * base64 encoded program bytes
   */
  result: string

  /**
   * JSON of the source map
   */
  sourcemap?: {}
}

// JSON DTO shape for TealCompile with wire keys and JSON-safe primitives
export type TealCompileDto = {
  hash: string
  result: string
  sourcemap?: {}
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TealCompile): TealCompileDto {
  const out: any = {}
  {
    const v = (value as any)['hash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hash'] = v
    }
  }
  {
    const v = (value as any)['result']
    if (v === undefined) {
      // omit undefined
    } else {
      out['result'] = v
    }
  }
  {
    const v = (value as any)['sourcemap']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sourcemap'] = v
    }
  }
  return out as TealCompileDto
}

export function fromDto(dto: TealCompileDto): TealCompile {
  const out: any = {}
  {
    const v = (dto as any)['hash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hash'] = v as any
    }
  }
  {
    const v = (dto as any)['result']
    if (v === undefined) {
      // omit undefined
    } else {
      out['result'] = v as any
    }
  }
  {
    const v = (dto as any)['sourcemap']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sourcemap'] = v as any
    }
  }
  return out as TealCompile
}

// Msgpack codecs
export function encodeMsgpack(value: TealCompile): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TealCompile {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TealCompile): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TealCompile {
  return fromDto(raw as TealCompileDto)
}

// Array helpers
export function encodeMsgpackArray(values: TealCompile[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TealCompile[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TealCompile[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TealCompile[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TealCompileMsgpackDto = {
  hash: string
  result: string
  sourcemap?: {}
}

function toMsgpackDto(value: TealCompile): TealCompileMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['hash']
    if (v === undefined) {
    } else {
      out['hash'] = v
    }
  }
  {
    const v = (value as any)['result']
    if (v === undefined) {
    } else {
      out['result'] = v
    }
  }
  {
    const v = (value as any)['sourcemap']
    if (v === undefined) {
    } else {
      out['sourcemap'] = v
    }
  }
  return out as TealCompileMsgpackDto
}

function fromMsgpackDto(dto: TealCompileMsgpackDto): TealCompile {
  const out: any = {}
  {
    const v = (dto as any)['hash']
    if (v === undefined) {
    } else {
      out['hash'] = v
    }
  }
  {
    const v = (dto as any)['result']
    if (v === undefined) {
    } else {
      out['result'] = v
    }
  }
  {
    const v = (dto as any)['sourcemap']
    if (v === undefined) {
    } else {
      out['sourcemap'] = v
    }
  }
  return out as TealCompile
}

export const TealCompile = {
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
