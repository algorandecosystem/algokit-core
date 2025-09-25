import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Application, ApplicationDto } from './application'
import { Application as ApplicationModel } from './application'

export type LookupAccountCreatedApplications = {
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

// JSON DTO shape for LookupAccountCreatedApplications with wire keys and JSON-safe primitives
export type LookupAccountCreatedApplicationsDto = {
  applications: ApplicationDto[]
  'current-round': bigint
  'next-token'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LookupAccountCreatedApplications): LookupAccountCreatedApplicationsDto {
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
  return out as LookupAccountCreatedApplicationsDto
}

export function fromDto(dto: LookupAccountCreatedApplicationsDto): LookupAccountCreatedApplications {
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
  return out as LookupAccountCreatedApplications
}

// Msgpack codecs
export function encodeMsgpack(value: LookupAccountCreatedApplications): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LookupAccountCreatedApplications {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LookupAccountCreatedApplications): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LookupAccountCreatedApplications {
  return fromDto(raw as LookupAccountCreatedApplicationsDto)
}

// Array helpers
export function encodeMsgpackArray(values: LookupAccountCreatedApplications[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LookupAccountCreatedApplications[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LookupAccountCreatedApplications[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LookupAccountCreatedApplications[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LookupAccountCreatedApplicationsMsgpackDto = {
  applications: ReturnType<(typeof ApplicationModel)['toMsgpackDto']>[]
  'current-round': bigint
  'next-token'?: string
}

function toMsgpackDto(value: LookupAccountCreatedApplications): LookupAccountCreatedApplicationsMsgpackDto {
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
  return out as LookupAccountCreatedApplicationsMsgpackDto
}

function fromMsgpackDto(dto: LookupAccountCreatedApplicationsMsgpackDto): LookupAccountCreatedApplications {
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
  return out as LookupAccountCreatedApplications
}

export const LookupAccountCreatedApplications = {
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
