import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { SimulateInitialStates, SimulateInitialStatesDto } from './simulate-initial-states'
import { SimulateInitialStates as SimulateInitialStatesModel } from './simulate-initial-states'
import type { SimulateTraceConfig, SimulateTraceConfigDto } from './simulate-trace-config'
import { SimulateTraceConfig as SimulateTraceConfigModel } from './simulate-trace-config'
import type { SimulateTransactionGroupResult, SimulateTransactionGroupResultDto } from './simulate-transaction-group-result'
import { SimulateTransactionGroupResult as SimulateTransactionGroupResultModel } from './simulate-transaction-group-result'
import type { SimulationEvalOverrides, SimulationEvalOverridesDto } from './simulation-eval-overrides'
import { SimulationEvalOverrides as SimulationEvalOverridesModel } from './simulation-eval-overrides'

export type SimulateTransaction = {
  /**
   * The version of this response object.
   */
  version: bigint

  /**
   * The round immediately preceding this simulation. State changes through this round were used to run this simulation.
   */
  lastRound: bigint

  /**
   * A result object for each transaction group that was simulated.
   */
  txnGroups: SimulateTransactionGroupResult[]
  evalOverrides?: SimulationEvalOverrides
  execTraceConfig?: SimulateTraceConfig
  initialStates?: SimulateInitialStates
}

// JSON DTO shape for SimulateTransaction with wire keys and JSON-safe primitives
export type SimulateTransactionDto = {
  version: string
  'last-round': string
  'txn-groups': SimulateTransactionGroupResultDto[]
  'eval-overrides'?: SimulationEvalOverridesDto
  'exec-trace-config'?: SimulateTraceConfigDto
  'initial-states'?: SimulateInitialStatesDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulateTransaction): SimulateTransactionDto {
  const out: any = {}
  {
    const v = (value as any)['version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['version'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['lastRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['txnGroups']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txn-groups'] = (v as any[]).map((item) => SimulateTransactionGroupResultModel.toDto(item))
    }
  }
  {
    const v = (value as any)['evalOverrides']
    if (v === undefined) {
      // omit undefined
    } else {
      out['eval-overrides'] = v === undefined ? v : SimulationEvalOverridesModel.toDto(v)
    }
  }
  {
    const v = (value as any)['execTraceConfig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['exec-trace-config'] = v === undefined ? v : SimulateTraceConfigModel.toDto(v)
    }
  }
  {
    const v = (value as any)['initialStates']
    if (v === undefined) {
      // omit undefined
    } else {
      out['initial-states'] = v === undefined ? v : SimulateInitialStatesModel.toDto(v)
    }
  }
  return out as SimulateTransactionDto
}

export function fromDto(dto: SimulateTransactionDto): SimulateTransaction {
  const out: any = {}
  {
    const v = (dto as any)['version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['version'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['last-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['txn-groups']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txnGroups'] = (v as any[]).map((item) => SimulateTransactionGroupResultModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['eval-overrides']
    if (v === undefined) {
      // omit undefined
    } else {
      out['evalOverrides'] = v === undefined ? v : SimulationEvalOverridesModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['exec-trace-config']
    if (v === undefined) {
      // omit undefined
    } else {
      out['execTraceConfig'] = v === undefined ? v : SimulateTraceConfigModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['initial-states']
    if (v === undefined) {
      // omit undefined
    } else {
      out['initialStates'] = v === undefined ? v : SimulateInitialStatesModel.fromDto(v)
    }
  }
  return out as SimulateTransaction
}

// Msgpack codecs
export function encodeMsgpack(value: SimulateTransaction): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulateTransaction {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulateTransaction): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulateTransaction {
  return fromDto(raw as SimulateTransactionDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulateTransaction[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulateTransaction[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulateTransaction[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulateTransaction[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulateTransactionMsgpackDto = {
  version: bigint
  'last-round': bigint
  'txn-groups': ReturnType<(typeof SimulateTransactionGroupResultModel)['toMsgpackDto']>[]
  'eval-overrides'?: ReturnType<(typeof SimulationEvalOverridesModel)['toMsgpackDto']>
  'exec-trace-config'?: ReturnType<(typeof SimulateTraceConfigModel)['toMsgpackDto']>
  'initial-states'?: ReturnType<(typeof SimulateInitialStatesModel)['toMsgpackDto']>
}

function toMsgpackDto(value: SimulateTransaction): SimulateTransactionMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['version']
    if (v === undefined) {
    } else {
      out['version'] = v
    }
  }
  {
    const v = (value as any)['lastRound']
    if (v === undefined) {
    } else {
      out['last-round'] = v
    }
  }
  {
    const v = (value as any)['txnGroups']
    if (v === undefined) {
    } else {
      out['txn-groups'] = (v as any[]).map((item) => SimulateTransactionGroupResultModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['evalOverrides']
    if (v === undefined) {
    } else {
      out['eval-overrides'] = SimulationEvalOverridesModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['execTraceConfig']
    if (v === undefined) {
    } else {
      out['exec-trace-config'] = SimulateTraceConfigModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['initialStates']
    if (v === undefined) {
    } else {
      out['initial-states'] = SimulateInitialStatesModel.toMsgpackDto(v)
    }
  }
  return out as SimulateTransactionMsgpackDto
}

function fromMsgpackDto(dto: SimulateTransactionMsgpackDto): SimulateTransaction {
  const out: any = {}
  {
    const v = (dto as any)['version']
    if (v === undefined) {
    } else {
      out['version'] = v
    }
  }
  {
    const v = (dto as any)['last-round']
    if (v === undefined) {
    } else {
      out['lastRound'] = v
    }
  }
  {
    const v = (dto as any)['txn-groups']
    if (v === undefined) {
    } else {
      out['txnGroups'] = (v as any[]).map((item) => SimulateTransactionGroupResultModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['eval-overrides']
    if (v === undefined) {
    } else {
      out['evalOverrides'] = SimulationEvalOverridesModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['exec-trace-config']
    if (v === undefined) {
    } else {
      out['execTraceConfig'] = SimulateTraceConfigModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['initial-states']
    if (v === undefined) {
    } else {
      out['initialStates'] = SimulateInitialStatesModel.fromMsgpackDto(v)
    }
  }
  return out as SimulateTransaction
}

export const SimulateTransaction = {
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
