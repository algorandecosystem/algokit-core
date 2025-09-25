import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type {
  LedgerStateDeltaForTransactionGroup,
  LedgerStateDeltaForTransactionGroupDto,
} from './ledger-state-delta-for-transaction-group'
import { LedgerStateDeltaForTransactionGroup as LedgerStateDeltaForTransactionGroupModel } from './ledger-state-delta-for-transaction-group'

export type GetTransactionGroupLedgerStateDeltasForRound = {
  deltas: LedgerStateDeltaForTransactionGroup[]
}

// JSON DTO shape for GetTransactionGroupLedgerStateDeltasForRound with wire keys and JSON-safe primitives
export type GetTransactionGroupLedgerStateDeltasForRoundDto = {
  Deltas: LedgerStateDeltaForTransactionGroupDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetTransactionGroupLedgerStateDeltasForRound): GetTransactionGroupLedgerStateDeltasForRoundDto {
  const out: any = {}
  {
    const v = (value as any)['deltas']
    if (v === undefined) {
      // omit undefined
    } else {
      out['Deltas'] = (v as any[]).map((item) => LedgerStateDeltaForTransactionGroupModel.toDto(item))
    }
  }
  return out as GetTransactionGroupLedgerStateDeltasForRoundDto
}

export function fromDto(dto: GetTransactionGroupLedgerStateDeltasForRoundDto): GetTransactionGroupLedgerStateDeltasForRound {
  const out: any = {}
  {
    const v = (dto as any)['Deltas']
    if (v === undefined) {
      // omit undefined
    } else {
      out['deltas'] = (v as any[]).map((item) => LedgerStateDeltaForTransactionGroupModel.fromDto(item))
    }
  }
  return out as GetTransactionGroupLedgerStateDeltasForRound
}

// Msgpack codecs
export function encodeMsgpack(value: GetTransactionGroupLedgerStateDeltasForRound): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetTransactionGroupLedgerStateDeltasForRound {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetTransactionGroupLedgerStateDeltasForRound): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetTransactionGroupLedgerStateDeltasForRound {
  return fromDto(raw as GetTransactionGroupLedgerStateDeltasForRoundDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetTransactionGroupLedgerStateDeltasForRound[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetTransactionGroupLedgerStateDeltasForRound[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetTransactionGroupLedgerStateDeltasForRound[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetTransactionGroupLedgerStateDeltasForRound[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetTransactionGroupLedgerStateDeltasForRoundMsgpackDto = {
  Deltas: ReturnType<(typeof LedgerStateDeltaForTransactionGroupModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: GetTransactionGroupLedgerStateDeltasForRound): GetTransactionGroupLedgerStateDeltasForRoundMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['deltas']
    if (v === undefined) {
    } else {
      out['Deltas'] = (v as any[]).map((item) => LedgerStateDeltaForTransactionGroupModel.toMsgpackDto(item))
    }
  }
  return out as GetTransactionGroupLedgerStateDeltasForRoundMsgpackDto
}

function fromMsgpackDto(dto: GetTransactionGroupLedgerStateDeltasForRoundMsgpackDto): GetTransactionGroupLedgerStateDeltasForRound {
  const out: any = {}
  {
    const v = (dto as any)['Deltas']
    if (v === undefined) {
    } else {
      out['deltas'] = (v as any[]).map((item) => LedgerStateDeltaForTransactionGroupModel.fromMsgpackDto(item))
    }
  }
  return out as GetTransactionGroupLedgerStateDeltasForRound
}

export const GetTransactionGroupLedgerStateDeltasForRound = {
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
