import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AvmValue, AvmValueDto } from './avm-value'
import { AvmValue as AvmValueModel } from './avm-value'

/**
 * A write operation into a scratch slot.
 */
export type ScratchChange = {
  /**
   * The scratch slot written.
   */
  slot: bigint
  newValue: AvmValue
}

// JSON DTO shape for ScratchChange with wire keys and JSON-safe primitives
export type ScratchChangeDto = {
  slot: bigint
  'new-value': AvmValueDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ScratchChange): ScratchChangeDto {
  const out: any = {}
  {
    const v = (value as any)['slot']
    if (v === undefined) {
      // omit undefined
    } else {
      out['slot'] = v
    }
  }
  {
    const v = (value as any)['newValue']
    if (v === undefined) {
      // omit undefined
    } else {
      out['new-value'] = v === undefined ? v : AvmValueModel.toDto(v)
    }
  }
  return out as ScratchChangeDto
}

export function fromDto(dto: ScratchChangeDto): ScratchChange {
  const out: any = {}
  {
    const v = (dto as any)['slot']
    if (v === undefined) {
      // omit undefined
    } else {
      out['slot'] = v as any
    }
  }
  {
    const v = (dto as any)['new-value']
    if (v === undefined) {
      // omit undefined
    } else {
      out['newValue'] = v === undefined ? v : AvmValueModel.fromDto(v)
    }
  }
  return out as ScratchChange
}

// Msgpack codecs
export function encodeMsgpack(value: ScratchChange): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ScratchChange {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ScratchChange): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ScratchChange {
  return fromDto(raw as ScratchChangeDto)
}

// Array helpers
export function encodeMsgpackArray(values: ScratchChange[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ScratchChange[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ScratchChange[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ScratchChange[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ScratchChangeMsgpackDto = {
  slot: bigint
  'new-value': ReturnType<(typeof AvmValueModel)['toMsgpackDto']>
}

function toMsgpackDto(value: ScratchChange): ScratchChangeMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['slot']
    if (v === undefined) {
    } else {
      out['slot'] = v
    }
  }
  {
    const v = (value as any)['newValue']
    if (v === undefined) {
    } else {
      out['new-value'] = AvmValueModel.toMsgpackDto(v)
    }
  }
  return out as ScratchChangeMsgpackDto
}

function fromMsgpackDto(dto: ScratchChangeMsgpackDto): ScratchChange {
  const out: any = {}
  {
    const v = (dto as any)['slot']
    if (v === undefined) {
    } else {
      out['slot'] = v
    }
  }
  {
    const v = (dto as any)['new-value']
    if (v === undefined) {
    } else {
      out['newValue'] = AvmValueModel.fromMsgpackDto(v)
    }
  }
  return out as ScratchChange
}

export const ScratchChange = {
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
