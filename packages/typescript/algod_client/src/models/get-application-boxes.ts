import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { BoxDescriptor, BoxDescriptorDto } from './box-descriptor'
import { BoxDescriptor as BoxDescriptorModel } from './box-descriptor'

export type GetApplicationBoxes = {
  boxes: BoxDescriptor[]
}

// JSON DTO shape for GetApplicationBoxes with wire keys and JSON-safe primitives
export type GetApplicationBoxesDto = {
  boxes: BoxDescriptorDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetApplicationBoxes): GetApplicationBoxesDto {
  const out: any = {}
  {
    const v = (value as any)['boxes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxDescriptorModel.toDto(item))
    }
  }
  return out as GetApplicationBoxesDto
}

export function fromDto(dto: GetApplicationBoxesDto): GetApplicationBoxes {
  const out: any = {}
  {
    const v = (dto as any)['boxes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxDescriptorModel.fromDto(item))
    }
  }
  return out as GetApplicationBoxes
}

// Msgpack codecs
export function encodeMsgpack(value: GetApplicationBoxes): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetApplicationBoxes {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetApplicationBoxes): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetApplicationBoxes {
  return fromDto(raw as GetApplicationBoxesDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetApplicationBoxes[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetApplicationBoxes[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetApplicationBoxes[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetApplicationBoxes[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetApplicationBoxesMsgpackDto = {
  boxes: ReturnType<(typeof BoxDescriptorModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: GetApplicationBoxes): GetApplicationBoxesMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['boxes']
    if (v === undefined) {
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxDescriptorModel.toMsgpackDto(item))
    }
  }
  return out as GetApplicationBoxesMsgpackDto
}

function fromMsgpackDto(dto: GetApplicationBoxesMsgpackDto): GetApplicationBoxes {
  const out: any = {}
  {
    const v = (dto as any)['boxes']
    if (v === undefined) {
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxDescriptorModel.fromMsgpackDto(item))
    }
  }
  return out as GetApplicationBoxes
}

export const GetApplicationBoxes = {
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
