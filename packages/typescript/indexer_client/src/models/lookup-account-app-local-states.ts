import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { ApplicationLocalState, ApplicationLocalStateDto } from './application-local-state'
import { ApplicationLocalState as ApplicationLocalStateModel } from './application-local-state'

export type LookupAccountAppLocalStates = {
  appsLocalStates: ApplicationLocalState[]

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
}

// JSON DTO shape for LookupAccountAppLocalStates with wire keys and JSON-safe primitives
export type LookupAccountAppLocalStatesDto = {
  'apps-local-states': ApplicationLocalStateDto[]
  'current-round': bigint
  'next-token'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LookupAccountAppLocalStates): LookupAccountAppLocalStatesDto {
  const out: any = {}
  {
    const v = (value as any)['appsLocalStates']
    if (v === undefined) {
      // omit undefined
    } else {
      out['apps-local-states'] = (v as any[]).map((item) => ApplicationLocalStateModel.toDto(item))
    }
  }
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
  return out as LookupAccountAppLocalStatesDto
}

export function fromDto(dto: LookupAccountAppLocalStatesDto): LookupAccountAppLocalStates {
  const out: any = {}
  {
    const v = (dto as any)['apps-local-states']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appsLocalStates'] = (v as any[]).map((item) => ApplicationLocalStateModel.fromDto(item))
    }
  }
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
  return out as LookupAccountAppLocalStates
}

// Msgpack codecs
export function encodeMsgpack(value: LookupAccountAppLocalStates): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LookupAccountAppLocalStates {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LookupAccountAppLocalStates): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LookupAccountAppLocalStates {
  return fromDto(raw as LookupAccountAppLocalStatesDto)
}

// Array helpers
export function encodeMsgpackArray(values: LookupAccountAppLocalStates[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LookupAccountAppLocalStates[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LookupAccountAppLocalStates[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LookupAccountAppLocalStates[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LookupAccountAppLocalStatesMsgpackDto = {
  'apps-local-states': ReturnType<(typeof ApplicationLocalStateModel)['toMsgpackDto']>[]
  'current-round': bigint
  'next-token'?: string
}

function toMsgpackDto(value: LookupAccountAppLocalStates): LookupAccountAppLocalStatesMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['appsLocalStates']
    if (v === undefined) {
    } else {
      out['apps-local-states'] = (v as any[]).map((item) => ApplicationLocalStateModel.toMsgpackDto(item))
    }
  }
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
  return out as LookupAccountAppLocalStatesMsgpackDto
}

function fromMsgpackDto(dto: LookupAccountAppLocalStatesMsgpackDto): LookupAccountAppLocalStates {
  const out: any = {}
  {
    const v = (dto as any)['apps-local-states']
    if (v === undefined) {
    } else {
      out['appsLocalStates'] = (v as any[]).map((item) => ApplicationLocalStateModel.fromMsgpackDto(item))
    }
  }
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
  return out as LookupAccountAppLocalStates
}

export const LookupAccountAppLocalStates = {
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
