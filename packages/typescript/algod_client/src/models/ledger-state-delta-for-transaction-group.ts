import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { LedgerStateDelta, LedgerStateDeltaDto } from './ledger-state-delta'
import { LedgerStateDelta as LedgerStateDeltaModel } from './ledger-state-delta'

/**
 * Contains a ledger delta for a single transaction group
 */
export type LedgerStateDeltaForTransactionGroup = {
  delta: LedgerStateDelta
  ids: string[]
}

// JSON DTO shape for LedgerStateDeltaForTransactionGroup with wire keys and JSON-safe primitives
export type LedgerStateDeltaForTransactionGroupDto = {
  Delta: LedgerStateDeltaDto
  Ids: string[][]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LedgerStateDeltaForTransactionGroup): LedgerStateDeltaForTransactionGroupDto {
  const out: any = {}
  {
    const v = (value as any)['delta']
    if (v === undefined) {
      // omit undefined
    } else {
      out['Delta'] = v === undefined ? v : LedgerStateDeltaModel.toDto(v)
    }
  }
  {
    const v = (value as any)['ids']
    if (v === undefined) {
      // omit undefined
    } else {
      out['Ids'] = v as any[]
    }
  }
  return out as LedgerStateDeltaForTransactionGroupDto
}

export function fromDto(dto: LedgerStateDeltaForTransactionGroupDto): LedgerStateDeltaForTransactionGroup {
  const out: any = {}
  {
    const v = (dto as any)['Delta']
    if (v === undefined) {
      // omit undefined
    } else {
      out['delta'] = v === undefined ? v : LedgerStateDeltaModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['Ids']
    if (v === undefined) {
      // omit undefined
    } else {
      out['ids'] = v as any[]
    }
  }
  return out as LedgerStateDeltaForTransactionGroup
}

// Msgpack codecs
export function encodeMsgpack(value: LedgerStateDeltaForTransactionGroup): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LedgerStateDeltaForTransactionGroup {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LedgerStateDeltaForTransactionGroup): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LedgerStateDeltaForTransactionGroup {
  return fromDto(raw as LedgerStateDeltaForTransactionGroupDto)
}

// Array helpers
export function encodeMsgpackArray(values: LedgerStateDeltaForTransactionGroup[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LedgerStateDeltaForTransactionGroup[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LedgerStateDeltaForTransactionGroup[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LedgerStateDeltaForTransactionGroup[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LedgerStateDeltaForTransactionGroupMsgpackDto = {
  Delta: ReturnType<(typeof LedgerStateDeltaModel)['toMsgpackDto']>
  Ids: string[][]
}

function toMsgpackDto(value: LedgerStateDeltaForTransactionGroup): LedgerStateDeltaForTransactionGroupMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['delta']
    if (v === undefined) {
    } else {
      out['Delta'] = LedgerStateDeltaModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['ids']
    if (v === undefined) {
    } else {
      out['Ids'] = v as any[]
    }
  }
  return out as LedgerStateDeltaForTransactionGroupMsgpackDto
}

function fromMsgpackDto(dto: LedgerStateDeltaForTransactionGroupMsgpackDto): LedgerStateDeltaForTransactionGroup {
  const out: any = {}
  {
    const v = (dto as any)['Delta']
    if (v === undefined) {
    } else {
      out['delta'] = LedgerStateDeltaModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['Ids']
    if (v === undefined) {
    } else {
      out['ids'] = v as any[]
    }
  }
  return out as LedgerStateDeltaForTransactionGroup
}

export const LedgerStateDeltaForTransactionGroup = {
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
