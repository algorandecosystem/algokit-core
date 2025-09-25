import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * The logged messages from an app call along with the app ID and outer transaction ID. Logs appear in the same order that they were emitted.
 */
export type AppCallLogs = {
  /**
   * An array of logs
   */
  logs: Uint8Array[]

  /**
   * The application from which the logs were generated
   */
  appId: bigint

  /**
   * The transaction ID of the outer app call that lead to these logs
   */
  txId: string
}

// JSON DTO shape for AppCallLogs with wire keys and JSON-safe primitives
export type AppCallLogsDto = {
  logs: string[]
  'application-index': string
  txId: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AppCallLogs): AppCallLogsDto {
  const out: any = {}
  {
    const v = (value as any)['logs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logs'] = (v as Uint8Array[]).map(toBase64)
    }
  }
  {
    const v = (value as any)['appId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['application-index'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['txId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txId'] = v
    }
  }
  return out as AppCallLogsDto
}

export function fromDto(dto: AppCallLogsDto): AppCallLogs {
  const out: any = {}
  {
    const v = (dto as any)['logs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logs'] = (v as string[]).map(fromBase64)
    }
  }
  {
    const v = (dto as any)['application-index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appId'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['txId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txId'] = v as any
    }
  }
  return out as AppCallLogs
}

// Msgpack codecs
export function encodeMsgpack(value: AppCallLogs): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AppCallLogs {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AppCallLogs): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AppCallLogs {
  return fromDto(raw as AppCallLogsDto)
}

// Array helpers
export function encodeMsgpackArray(values: AppCallLogs[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AppCallLogs[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AppCallLogs[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AppCallLogs[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AppCallLogsMsgpackDto = {
  logs: Uint8Array[]
  'application-index': bigint
  txId: string
}

function toMsgpackDto(value: AppCallLogs): AppCallLogsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['logs']
    if (v === undefined) {
    } else {
      out['logs'] = v as any[]
    }
  }
  {
    const v = (value as any)['appId']
    if (v === undefined) {
    } else {
      out['application-index'] = v
    }
  }
  {
    const v = (value as any)['txId']
    if (v === undefined) {
    } else {
      out['txId'] = v
    }
  }
  return out as AppCallLogsMsgpackDto
}

function fromMsgpackDto(dto: AppCallLogsMsgpackDto): AppCallLogs {
  const out: any = {}
  {
    const v = (dto as any)['logs']
    if (v === undefined) {
    } else {
      out['logs'] = v as any[]
    }
  }
  {
    const v = (dto as any)['application-index']
    if (v === undefined) {
    } else {
      out['appId'] = v
    }
  }
  {
    const v = (dto as any)['txId']
    if (v === undefined) {
    } else {
      out['txId'] = v
    }
  }
  return out as AppCallLogs
}

export const AppCallLogs = {
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
