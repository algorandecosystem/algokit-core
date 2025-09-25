import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AppCallLogs, AppCallLogsDto } from './app-call-logs'
import { AppCallLogs as AppCallLogsModel } from './app-call-logs'

export type GetBlockLogs = {
  logs: AppCallLogs[]
}

// JSON DTO shape for GetBlockLogs with wire keys and JSON-safe primitives
export type GetBlockLogsDto = {
  logs: AppCallLogsDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetBlockLogs): GetBlockLogsDto {
  const out: any = {}
  {
    const v = (value as any)['logs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logs'] = (v as any[]).map((item) => AppCallLogsModel.toDto(item))
    }
  }
  return out as GetBlockLogsDto
}

export function fromDto(dto: GetBlockLogsDto): GetBlockLogs {
  const out: any = {}
  {
    const v = (dto as any)['logs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logs'] = (v as any[]).map((item) => AppCallLogsModel.fromDto(item))
    }
  }
  return out as GetBlockLogs
}

// Msgpack codecs
export function encodeMsgpack(value: GetBlockLogs): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetBlockLogs {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetBlockLogs): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetBlockLogs {
  return fromDto(raw as GetBlockLogsDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetBlockLogs[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetBlockLogs[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetBlockLogs[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetBlockLogs[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetBlockLogsMsgpackDto = {
  logs: ReturnType<(typeof AppCallLogsModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: GetBlockLogs): GetBlockLogsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['logs']
    if (v === undefined) {
    } else {
      out['logs'] = (v as any[]).map((item) => AppCallLogsModel.toMsgpackDto(item))
    }
  }
  return out as GetBlockLogsMsgpackDto
}

function fromMsgpackDto(dto: GetBlockLogsMsgpackDto): GetBlockLogs {
  const out: any = {}
  {
    const v = (dto as any)['logs']
    if (v === undefined) {
    } else {
      out['logs'] = (v as any[]).map((item) => AppCallLogsModel.fromMsgpackDto(item))
    }
  }
  return out as GetBlockLogs
}

export const GetBlockLogs = {
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
