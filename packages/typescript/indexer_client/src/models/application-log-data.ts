import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Stores the global information associated with an application.
 */
export type ApplicationLogData = {
  /**
   * Transaction ID
   */
  txid: string

  /**
   * Logs for the application being executed by the transaction.
   */
  logs: Uint8Array[]
}

// JSON DTO shape for ApplicationLogData with wire keys and JSON-safe primitives
export type ApplicationLogDataDto = {
  txid: string
  logs: string[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ApplicationLogData): ApplicationLogDataDto {
  const out: any = {}
  {
    const v = (value as any)['txid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txid'] = v
    }
  }
  {
    const v = (value as any)['logs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logs'] = (v as Uint8Array[]).map(toBase64)
    }
  }
  return out as ApplicationLogDataDto
}

export function fromDto(dto: ApplicationLogDataDto): ApplicationLogData {
  const out: any = {}
  {
    const v = (dto as any)['txid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txid'] = v as any
    }
  }
  {
    const v = (dto as any)['logs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logs'] = (v as string[]).map(fromBase64)
    }
  }
  return out as ApplicationLogData
}

// Msgpack codecs
export function encodeMsgpack(value: ApplicationLogData): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ApplicationLogData {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ApplicationLogData): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ApplicationLogData {
  return fromDto(raw as ApplicationLogDataDto)
}

// Array helpers
export function encodeMsgpackArray(values: ApplicationLogData[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ApplicationLogData[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ApplicationLogData[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ApplicationLogData[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ApplicationLogDataMsgpackDto = {
  txid: string
  logs: Uint8Array[]
}

function toMsgpackDto(value: ApplicationLogData): ApplicationLogDataMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['txid']
    if (v === undefined) {
    } else {
      out['txid'] = v
    }
  }
  {
    const v = (value as any)['logs']
    if (v === undefined) {
    } else {
      out['logs'] = v as any[]
    }
  }
  return out as ApplicationLogDataMsgpackDto
}

function fromMsgpackDto(dto: ApplicationLogDataMsgpackDto): ApplicationLogData {
  const out: any = {}
  {
    const v = (dto as any)['txid']
    if (v === undefined) {
    } else {
      out['txid'] = v
    }
  }
  {
    const v = (dto as any)['logs']
    if (v === undefined) {
    } else {
      out['logs'] = v as any[]
    }
  }
  return out as ApplicationLogData
}

export const ApplicationLogData = {
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
