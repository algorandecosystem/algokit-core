import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type GetBlock = {
  /**
   * Block header data.
   */
  block: {}

  /**
   * Optional certificate object. This is only included when the format is set to message pack.
   */
  cert?: {}
}

// JSON DTO shape for GetBlock with wire keys and JSON-safe primitives
export type GetBlockDto = {
  block: {}
  cert?: {}
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetBlock): GetBlockDto {
  const out: any = {}
  {
    const v = (value as any)['block']
    if (v === undefined) {
      // omit undefined
    } else {
      out['block'] = v
    }
  }
  {
    const v = (value as any)['cert']
    if (v === undefined) {
      // omit undefined
    } else {
      out['cert'] = v
    }
  }
  return out as GetBlockDto
}

export function fromDto(dto: GetBlockDto): GetBlock {
  const out: any = {}
  {
    const v = (dto as any)['block']
    if (v === undefined) {
      // omit undefined
    } else {
      out['block'] = v as any
    }
  }
  {
    const v = (dto as any)['cert']
    if (v === undefined) {
      // omit undefined
    } else {
      out['cert'] = v as any
    }
  }
  return out as GetBlock
}

// Msgpack codecs
export function encodeMsgpack(value: GetBlock): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetBlock {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetBlock): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetBlock {
  return fromDto(raw as GetBlockDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetBlock[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetBlock[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetBlock[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetBlock[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetBlockMsgpackDto = {
  block: {}
  cert?: {}
}

function toMsgpackDto(value: GetBlock): GetBlockMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['block']
    if (v === undefined) {
    } else {
      out['block'] = v
    }
  }
  {
    const v = (value as any)['cert']
    if (v === undefined) {
    } else {
      out['cert'] = v
    }
  }
  return out as GetBlockMsgpackDto
}

function fromMsgpackDto(dto: GetBlockMsgpackDto): GetBlock {
  const out: any = {}
  {
    const v = (dto as any)['block']
    if (v === undefined) {
    } else {
      out['block'] = v
    }
  }
  {
    const v = (dto as any)['cert']
    if (v === undefined) {
    } else {
      out['cert'] = v
    }
  }
  return out as GetBlock
}

export const GetBlock = {
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
