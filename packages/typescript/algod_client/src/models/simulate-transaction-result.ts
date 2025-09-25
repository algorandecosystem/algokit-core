import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { PendingTransactionResponse, PendingTransactionResponseDto } from './pending-transaction-response'
import { PendingTransactionResponse as PendingTransactionResponseModel } from './pending-transaction-response'
import type { SimulateUnnamedResourcesAccessed, SimulateUnnamedResourcesAccessedDto } from './simulate-unnamed-resources-accessed'
import { SimulateUnnamedResourcesAccessed as SimulateUnnamedResourcesAccessedModel } from './simulate-unnamed-resources-accessed'
import type { SimulationTransactionExecTrace, SimulationTransactionExecTraceDto } from './simulation-transaction-exec-trace'
import { SimulationTransactionExecTrace as SimulationTransactionExecTraceModel } from './simulation-transaction-exec-trace'

/**
 * Simulation result for an individual transaction
 */
export type SimulateTransactionResult = {
  txnResult: PendingTransactionResponse

  /**
   * Budget used during execution of an app call transaction. This value includes budged used by inner app calls spawned by this transaction.
   */
  appBudgetConsumed?: bigint

  /**
   * Budget used during execution of a logic sig transaction.
   */
  logicSigBudgetConsumed?: bigint
  execTrace?: SimulationTransactionExecTrace
  unnamedResourcesAccessed?: SimulateUnnamedResourcesAccessed

  /**
   * The account that needed to sign this transaction when no signature was provided and the provided signer was incorrect.
   */
  fixedSigner?: string
}

// JSON DTO shape for SimulateTransactionResult with wire keys and JSON-safe primitives
export type SimulateTransactionResultDto = {
  'txn-result': PendingTransactionResponseDto
  'app-budget-consumed'?: bigint
  'logic-sig-budget-consumed'?: bigint
  'exec-trace'?: SimulationTransactionExecTraceDto
  'unnamed-resources-accessed'?: SimulateUnnamedResourcesAccessedDto
  'fixed-signer'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulateTransactionResult): SimulateTransactionResultDto {
  const out: any = {}
  {
    const v = (value as any)['txnResult']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txn-result'] = v === undefined ? v : PendingTransactionResponseModel.toDto(v)
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
    const v = (value as any)['logicSigBudgetConsumed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logic-sig-budget-consumed'] = v
    }
  }
  {
    const v = (value as any)['execTrace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['exec-trace'] = v === undefined ? v : SimulationTransactionExecTraceModel.toDto(v)
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
  {
    const v = (value as any)['fixedSigner']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fixed-signer'] = v
    }
  }
  return out as SimulateTransactionResultDto
}

export function fromDto(dto: SimulateTransactionResultDto): SimulateTransactionResult {
  const out: any = {}
  {
    const v = (dto as any)['txn-result']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txnResult'] = v === undefined ? v : PendingTransactionResponseModel.fromDto(v)
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
    const v = (dto as any)['logic-sig-budget-consumed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logicSigBudgetConsumed'] = v as any
    }
  }
  {
    const v = (dto as any)['exec-trace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['execTrace'] = v === undefined ? v : SimulationTransactionExecTraceModel.fromDto(v)
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
  {
    const v = (dto as any)['fixed-signer']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fixedSigner'] = v as any
    }
  }
  return out as SimulateTransactionResult
}

// Msgpack codecs
export function encodeMsgpack(value: SimulateTransactionResult): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulateTransactionResult {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulateTransactionResult): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulateTransactionResult {
  return fromDto(raw as SimulateTransactionResultDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulateTransactionResult[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulateTransactionResult[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulateTransactionResult[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulateTransactionResult[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulateTransactionResultMsgpackDto = {
  'txn-result': ReturnType<(typeof PendingTransactionResponseModel)['toMsgpackDto']>
  'app-budget-consumed'?: bigint
  'logic-sig-budget-consumed'?: bigint
  'exec-trace'?: ReturnType<(typeof SimulationTransactionExecTraceModel)['toMsgpackDto']>
  'unnamed-resources-accessed'?: ReturnType<(typeof SimulateUnnamedResourcesAccessedModel)['toMsgpackDto']>
  'fixed-signer'?: string
}

function toMsgpackDto(value: SimulateTransactionResult): SimulateTransactionResultMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['txnResult']
    if (v === undefined) {
    } else {
      out['txn-result'] = PendingTransactionResponseModel.toMsgpackDto(v)
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
    const v = (value as any)['logicSigBudgetConsumed']
    if (v === undefined) {
    } else {
      out['logic-sig-budget-consumed'] = v
    }
  }
  {
    const v = (value as any)['execTrace']
    if (v === undefined) {
    } else {
      out['exec-trace'] = SimulationTransactionExecTraceModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['unnamedResourcesAccessed']
    if (v === undefined) {
    } else {
      out['unnamed-resources-accessed'] = SimulateUnnamedResourcesAccessedModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['fixedSigner']
    if (v === undefined) {
    } else {
      out['fixed-signer'] = v
    }
  }
  return out as SimulateTransactionResultMsgpackDto
}

function fromMsgpackDto(dto: SimulateTransactionResultMsgpackDto): SimulateTransactionResult {
  const out: any = {}
  {
    const v = (dto as any)['txn-result']
    if (v === undefined) {
    } else {
      out['txnResult'] = PendingTransactionResponseModel.fromMsgpackDto(v)
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
    const v = (dto as any)['logic-sig-budget-consumed']
    if (v === undefined) {
    } else {
      out['logicSigBudgetConsumed'] = v
    }
  }
  {
    const v = (dto as any)['exec-trace']
    if (v === undefined) {
    } else {
      out['execTrace'] = SimulationTransactionExecTraceModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['unnamed-resources-accessed']
    if (v === undefined) {
    } else {
      out['unnamedResourcesAccessed'] = SimulateUnnamedResourcesAccessedModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['fixed-signer']
    if (v === undefined) {
    } else {
      out['fixedSigner'] = v
    }
  }
  return out as SimulateTransactionResult
}

export const SimulateTransactionResult = {
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
