import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Block, BlockDto } from './block'
import { Block as BlockModel } from './block'

export type SearchForBlockHeaders = {
  /**
   * Round at which the results were computed.
   */
  currentRound: bigint

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
  blocks: Block[]
}

// JSON DTO shape for SearchForBlockHeaders with wire keys and JSON-safe primitives
export type SearchForBlockHeadersDto = {
  'current-round': bigint
  'next-token'?: string
  blocks: BlockDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SearchForBlockHeaders): SearchForBlockHeadersDto {
  const out: any = {}
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['current-round'] = v
    }
  }
  {
    const v = (value as any)['nextToken']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-token'] = v
    }
  }
  {
    const v = (value as any)['blocks']
    if (v === undefined) {
      // omit undefined
    } else {
      out['blocks'] = (v as any[]).map((item) => BlockModel.toDto(item))
    }
  }
  return out as SearchForBlockHeadersDto
}

export function fromDto(dto: SearchForBlockHeadersDto): SearchForBlockHeaders {
  const out: any = {}
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['currentRound'] = v as any
    }
  }
  {
    const v = (dto as any)['next-token']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextToken'] = v as any
    }
  }
  {
    const v = (dto as any)['blocks']
    if (v === undefined) {
      // omit undefined
    } else {
      out['blocks'] = (v as any[]).map((item) => BlockModel.fromDto(item))
    }
  }
  return out as SearchForBlockHeaders
}

// Msgpack codecs
export function encodeMsgpack(value: SearchForBlockHeaders): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SearchForBlockHeaders {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SearchForBlockHeaders): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SearchForBlockHeaders {
  return fromDto(raw as SearchForBlockHeadersDto)
}

// Array helpers
export function encodeMsgpackArray(values: SearchForBlockHeaders[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SearchForBlockHeaders[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SearchForBlockHeaders[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SearchForBlockHeaders[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SearchForBlockHeadersMsgpackDto = {
  'current-round': bigint
  'next-token'?: string
  blocks: ReturnType<(typeof BlockModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: SearchForBlockHeaders): SearchForBlockHeadersMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
    } else {
      out['current-round'] = v
    }
  }
  {
    const v = (value as any)['nextToken']
    if (v === undefined) {
    } else {
      out['next-token'] = v
    }
  }
  {
    const v = (value as any)['blocks']
    if (v === undefined) {
    } else {
      out['blocks'] = (v as any[]).map((item) => BlockModel.toMsgpackDto(item))
    }
  }
  return out as SearchForBlockHeadersMsgpackDto
}

function fromMsgpackDto(dto: SearchForBlockHeadersMsgpackDto): SearchForBlockHeaders {
  const out: any = {}
  {
    const v = (dto as any)['current-round']
    if (v === undefined) {
    } else {
      out['currentRound'] = v
    }
  }
  {
    const v = (dto as any)['next-token']
    if (v === undefined) {
    } else {
      out['nextToken'] = v
    }
  }
  {
    const v = (dto as any)['blocks']
    if (v === undefined) {
    } else {
      out['blocks'] = (v as any[]).map((item) => BlockModel.fromMsgpackDto(item))
    }
  }
  return out as SearchForBlockHeaders
}

export const SearchForBlockHeaders = {
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
