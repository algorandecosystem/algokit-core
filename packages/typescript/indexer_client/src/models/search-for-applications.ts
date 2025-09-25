import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Application, ApplicationDto } from './application'
import { Application as ApplicationModel } from './application'

export type SearchForApplications = {
  applications: Application[]

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
}

// JSON DTO shape for SearchForApplications with wire keys and JSON-safe primitives
export type SearchForApplicationsDto = {
  applications: ApplicationDto[]
  'current-round': bigint
  'next-token'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SearchForApplications): SearchForApplicationsDto {
  const out: any = {}
  {
    const v = (value as any)['applications']
    if (v === undefined) {
      // omit undefined
    } else {
      out['applications'] = (v as any[]).map((item) => ApplicationModel.toDto(item))
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
  return out as SearchForApplicationsDto
}

export function fromDto(dto: SearchForApplicationsDto): SearchForApplications {
  const out: any = {}
  {
    const v = (dto as any)['applications']
    if (v === undefined) {
      // omit undefined
    } else {
      out['applications'] = (v as any[]).map((item) => ApplicationModel.fromDto(item))
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
  return out as SearchForApplications
}

// Msgpack codecs
export function encodeMsgpack(value: SearchForApplications): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SearchForApplications {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SearchForApplications): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SearchForApplications {
  return fromDto(raw as SearchForApplicationsDto)
}

// Array helpers
export function encodeMsgpackArray(values: SearchForApplications[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SearchForApplications[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SearchForApplications[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SearchForApplications[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SearchForApplicationsMsgpackDto = {
  applications: ReturnType<(typeof ApplicationModel)['toMsgpackDto']>[]
  'current-round': bigint
  'next-token'?: string
}

function toMsgpackDto(value: SearchForApplications): SearchForApplicationsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['applications']
    if (v === undefined) {
    } else {
      out['applications'] = (v as any[]).map((item) => ApplicationModel.toMsgpackDto(item))
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
  return out as SearchForApplicationsMsgpackDto
}

function fromMsgpackDto(dto: SearchForApplicationsMsgpackDto): SearchForApplications {
  const out: any = {}
  {
    const v = (dto as any)['applications']
    if (v === undefined) {
    } else {
      out['applications'] = (v as any[]).map((item) => ApplicationModel.fromMsgpackDto(item))
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
  return out as SearchForApplications
}

export const SearchForApplications = {
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
