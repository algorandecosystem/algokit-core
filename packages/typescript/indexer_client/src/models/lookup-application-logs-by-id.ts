import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { ApplicationLogData, ApplicationLogDataDto } from './application-log-data'
import { ApplicationLogData as ApplicationLogDataModel } from './application-log-data'

export type LookupApplicationLogsById = {
  /**
   * \[appidx\] application index.
   */
  applicationId: bigint

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string
  logData?: ApplicationLogData[]
}

// JSON DTO shape for LookupApplicationLogsById with wire keys and JSON-safe primitives
export type LookupApplicationLogsByIdDto = {
  'application-id': bigint
  'current-round': bigint
  'next-token'?: string
  'log-data'?: ApplicationLogDataDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LookupApplicationLogsById): LookupApplicationLogsByIdDto {
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
    const v = (value as any)['logData']
    if (v === undefined) {
      // omit undefined
    } else {
      out['log-data'] = (v as any[]).map((item) => ApplicationLogDataModel.toDto(item))
    }
  }
  return out as LookupApplicationLogsByIdDto
}

export function fromDto(dto: LookupApplicationLogsByIdDto): LookupApplicationLogsById {
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
    const v = (dto as any)['log-data']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logData'] = (v as any[]).map((item) => ApplicationLogDataModel.fromDto(item))
    }
  }
  return out as LookupApplicationLogsById
}

// Msgpack codecs
export function encodeMsgpack(value: LookupApplicationLogsById): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LookupApplicationLogsById {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LookupApplicationLogsById): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LookupApplicationLogsById {
  return fromDto(raw as LookupApplicationLogsByIdDto)
}

// Array helpers
export function encodeMsgpackArray(values: LookupApplicationLogsById[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LookupApplicationLogsById[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LookupApplicationLogsById[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LookupApplicationLogsById[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LookupApplicationLogsByIdMsgpackDto = {
  'application-id': bigint
  'current-round': bigint
  'next-token'?: string
  'log-data'?: ReturnType<(typeof ApplicationLogDataModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: LookupApplicationLogsById): LookupApplicationLogsByIdMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['applicationId']
    if (v === undefined) {
    } else {
      out['application-id'] = v
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
  {
    const v = (value as any)['logData']
    if (v === undefined) {
    } else {
      out['log-data'] = (v as any[]).map((item) => ApplicationLogDataModel.toMsgpackDto(item))
    }
  }
  return out as LookupApplicationLogsByIdMsgpackDto
}

function fromMsgpackDto(dto: LookupApplicationLogsByIdMsgpackDto): LookupApplicationLogsById {
  const out: any = {}
  {
    const v = (dto as any)['application-id']
    if (v === undefined) {
    } else {
      out['applicationId'] = v
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
  {
    const v = (dto as any)['log-data']
    if (v === undefined) {
    } else {
      out['logData'] = (v as any[]).map((item) => ApplicationLogDataModel.fromMsgpackDto(item))
    }
  }
  return out as LookupApplicationLogsById
}

export const LookupApplicationLogsById = {
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
