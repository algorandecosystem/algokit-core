import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { SimulateTransactionResult, SimulateTransactionResultDto } from './simulate-transaction-result'
import { SimulateTransactionResult as SimulateTransactionResultModel } from './simulate-transaction-result'
import type { SimulateUnnamedResourcesAccessed, SimulateUnnamedResourcesAccessedDto } from './simulate-unnamed-resources-accessed'
import { SimulateUnnamedResourcesAccessed as SimulateUnnamedResourcesAccessedModel } from './simulate-unnamed-resources-accessed'

/**
 * Simulation result for an atomic transaction group
 */
export type SimulateTransactionGroupResult = {
  /**
   * Simulation result for individual transactions
   */
  txnResults: SimulateTransactionResult[]

  /**
   * If present, indicates that the transaction group failed and specifies why that happened
   */
  failureMessage?: string

  /**
   * If present, indicates which transaction in this group caused the failure. This array represents the path to the failing transaction. Indexes are zero based, the first element indicates the top-level transaction, and successive elements indicate deeper inner transactions.
   */
  failedAt?: bigint[]

  /**
   * Total budget added during execution of app calls in the transaction group.
   */
  appBudgetAdded?: bigint

  /**
   * Total budget consumed during execution of app calls in the transaction group.
   */
  appBudgetConsumed?: bigint
  unnamedResourcesAccessed?: SimulateUnnamedResourcesAccessed
}

// JSON DTO shape for SimulateTransactionGroupResult with wire keys and JSON-safe primitives
export type SimulateTransactionGroupResultDto = {
  'txn-results': SimulateTransactionResultDto[]
  'failure-message'?: string
  'failed-at'?: bigint[]
  'app-budget-added'?: bigint
  'app-budget-consumed'?: bigint
  'unnamed-resources-accessed'?: SimulateUnnamedResourcesAccessedDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulateTransactionGroupResult): SimulateTransactionGroupResultDto {
  const out: any = {}
  {
    const v = (value as any)['txnResults']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txn-results'] = (v as any[]).map((item) => SimulateTransactionResultModel.toDto(item))
    }
  }
  {
    const v = (value as any)['failureMessage']
    if (v === undefined) {
      // omit undefined
    } else {
      out['failure-message'] = v
    }
  }
  {
    const v = (value as any)['failedAt']
    if (v === undefined) {
      // omit undefined
    } else {
      out['failed-at'] = v as any[]
    }
  }
  {
    const v = (value as any)['appBudgetAdded']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-budget-added'] = v
    }
  }
  {
    const v = (value as any)['appBudgetConsumed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-budget-consumed'] = v
    }
  }
  {
    const v = (value as any)['unnamedResourcesAccessed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['unnamed-resources-accessed'] = v === undefined ? v : SimulateUnnamedResourcesAccessedModel.toDto(v)
    }
  }
  return out as SimulateTransactionGroupResultDto
}

export function fromDto(dto: SimulateTransactionGroupResultDto): SimulateTransactionGroupResult {
  const out: any = {}
  {
    const v = (dto as any)['txn-results']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txnResults'] = (v as any[]).map((item) => SimulateTransactionResultModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['failure-message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['failureMessage'] = v as any
    }
  }
  {
    const v = (dto as any)['failed-at']
    if (v === undefined) {
      // omit undefined
    } else {
      out['failedAt'] = v as any[]
    }
  }
  {
    const v = (dto as any)['app-budget-added']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appBudgetAdded'] = v as any
    }
  }
  {
    const v = (dto as any)['app-budget-consumed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appBudgetConsumed'] = v as any
    }
  }
  {
    const v = (dto as any)['unnamed-resources-accessed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['unnamedResourcesAccessed'] = v === undefined ? v : SimulateUnnamedResourcesAccessedModel.fromDto(v)
    }
  }
  return out as SimulateTransactionGroupResult
}

// Msgpack codecs
export function encodeMsgpack(value: SimulateTransactionGroupResult): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulateTransactionGroupResult {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulateTransactionGroupResult): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulateTransactionGroupResult {
  return fromDto(raw as SimulateTransactionGroupResultDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulateTransactionGroupResult[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulateTransactionGroupResult[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulateTransactionGroupResult[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulateTransactionGroupResult[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulateTransactionGroupResultMsgpackDto = {
  'txn-results': ReturnType<(typeof SimulateTransactionResultModel)['toMsgpackDto']>[]
  'failure-message'?: string
  'failed-at'?: bigint[]
  'app-budget-added'?: bigint
  'app-budget-consumed'?: bigint
  'unnamed-resources-accessed'?: ReturnType<(typeof SimulateUnnamedResourcesAccessedModel)['toMsgpackDto']>
}

function toMsgpackDto(value: SimulateTransactionGroupResult): SimulateTransactionGroupResultMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['txnResults']
    if (v === undefined) {
    } else {
      out['txn-results'] = (v as any[]).map((item) => SimulateTransactionResultModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['failureMessage']
    if (v === undefined) {
    } else {
      out['failure-message'] = v
    }
  }
  {
    const v = (value as any)['failedAt']
    if (v === undefined) {
    } else {
      out['failed-at'] = v as any[]
    }
  }
  {
    const v = (value as any)['appBudgetAdded']
    if (v === undefined) {
    } else {
      out['app-budget-added'] = v
    }
  }
  {
    const v = (value as any)['appBudgetConsumed']
    if (v === undefined) {
    } else {
      out['app-budget-consumed'] = v
    }
  }
  {
    const v = (value as any)['unnamedResourcesAccessed']
    if (v === undefined) {
    } else {
      out['unnamed-resources-accessed'] = SimulateUnnamedResourcesAccessedModel.toMsgpackDto(v)
    }
  }
  return out as SimulateTransactionGroupResultMsgpackDto
}

function fromMsgpackDto(dto: SimulateTransactionGroupResultMsgpackDto): SimulateTransactionGroupResult {
  const out: any = {}
  {
    const v = (dto as any)['txn-results']
    if (v === undefined) {
    } else {
      out['txnResults'] = (v as any[]).map((item) => SimulateTransactionResultModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['failure-message']
    if (v === undefined) {
    } else {
      out['failureMessage'] = v
    }
  }
  {
    const v = (dto as any)['failed-at']
    if (v === undefined) {
    } else {
      out['failedAt'] = v as any[]
    }
  }
  {
    const v = (dto as any)['app-budget-added']
    if (v === undefined) {
    } else {
      out['appBudgetAdded'] = v
    }
  }
  {
    const v = (dto as any)['app-budget-consumed']
    if (v === undefined) {
    } else {
      out['appBudgetConsumed'] = v
    }
  }
  {
    const v = (dto as any)['unnamed-resources-accessed']
    if (v === undefined) {
    } else {
      out['unnamedResourcesAccessed'] = SimulateUnnamedResourcesAccessedModel.fromMsgpackDto(v)
    }
  }
  return out as SimulateTransactionGroupResult
}

export const SimulateTransactionGroupResult = {
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
