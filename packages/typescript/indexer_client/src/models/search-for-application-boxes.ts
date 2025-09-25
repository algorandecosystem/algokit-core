import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { BoxDescriptor, BoxDescriptorDto } from './box-descriptor'
import { BoxDescriptor as BoxDescriptorModel } from './box-descriptor'

export type SearchForApplicationBoxes = {
  /**
   * \[appidx\] application index.
   */
  applicationId: bigint
  boxes: BoxDescriptor[]

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
}

// JSON DTO shape for SearchForApplicationBoxes with wire keys and JSON-safe primitives
export type SearchForApplicationBoxesDto = {
  'application-id': bigint
  boxes: BoxDescriptorDto[]
  'next-token'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SearchForApplicationBoxes): SearchForApplicationBoxesDto {
  const out: any = {}
  {
    const v = (value as any)['applicationId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['application-id'] = v
    }
  }
  {
    const v = (value as any)['boxes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxDescriptorModel.toDto(item))
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
  return out as SearchForApplicationBoxesDto
}

export function fromDto(dto: SearchForApplicationBoxesDto): SearchForApplicationBoxes {
  const out: any = {}
  {
    const v = (dto as any)['application-id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['applicationId'] = v as any
    }
  }
  {
    const v = (dto as any)['boxes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxDescriptorModel.fromDto(item))
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
  return out as SearchForApplicationBoxes
}

// Msgpack codecs
export function encodeMsgpack(value: SearchForApplicationBoxes): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SearchForApplicationBoxes {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SearchForApplicationBoxes): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SearchForApplicationBoxes {
  return fromDto(raw as SearchForApplicationBoxesDto)
}

// Array helpers
export function encodeMsgpackArray(values: SearchForApplicationBoxes[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SearchForApplicationBoxes[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SearchForApplicationBoxes[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SearchForApplicationBoxes[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SearchForApplicationBoxesMsgpackDto = {
  'application-id': bigint
  boxes: ReturnType<(typeof BoxDescriptorModel)['toMsgpackDto']>[]
  'next-token'?: string
}

function toMsgpackDto(value: SearchForApplicationBoxes): SearchForApplicationBoxesMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['applicationId']
    if (v === undefined) {
    } else {
      out['application-id'] = v
    }
  }
  {
    const v = (value as any)['boxes']
    if (v === undefined) {
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxDescriptorModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['nextToken']
    if (v === undefined) {
    } else {
      out['next-token'] = v
    }
  }
  return out as SearchForApplicationBoxesMsgpackDto
}

function fromMsgpackDto(dto: SearchForApplicationBoxesMsgpackDto): SearchForApplicationBoxes {
  const out: any = {}
  {
    const v = (dto as any)['application-id']
    if (v === undefined) {
    } else {
      out['applicationId'] = v
    }
  }
  {
    const v = (dto as any)['boxes']
    if (v === undefined) {
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxDescriptorModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['next-token']
    if (v === undefined) {
    } else {
      out['nextToken'] = v
    }
  }
  return out as SearchForApplicationBoxes
}

export const SearchForApplicationBoxes = {
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
