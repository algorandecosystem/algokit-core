import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AccountStateDelta, AccountStateDeltaDto } from './account-state-delta'
import { AccountStateDelta as AccountStateDeltaModel } from './account-state-delta'
import type { DryrunState, DryrunStateDto } from './dryrun-state'
import { DryrunState as DryrunStateModel } from './dryrun-state'
import type { StateDelta, StateDeltaDto } from './state-delta'
import { StateDelta as StateDeltaModel } from './state-delta'

/**
 * DryrunTxnResult contains any LogicSig or ApplicationCall program debug information and state updates from a dryrun.
 */
export type DryrunTxnResult = {
  /**
   * Disassembled program line by line.
   */
  disassembly: string[]

  /**
   * Disassembled lsig program line by line.
   */
  logicSigDisassembly?: string[]
  logicSigTrace?: DryrunState[]
  logicSigMessages?: string[]
  appCallTrace?: DryrunState[]
  appCallMessages?: string[]
  globalDelta?: StateDelta
  localDeltas?: AccountStateDelta[]
  logs?: Uint8Array[]

  /**
   * Budget added during execution of app call transaction.
   */
  budgetAdded?: bigint

  /**
   * Budget consumed during execution of app call transaction.
   */
  budgetConsumed?: bigint
}

// JSON DTO shape for DryrunTxnResult with wire keys and JSON-safe primitives
export type DryrunTxnResultDto = {
  disassembly: string[]
  'logic-sig-disassembly'?: string[]
  'logic-sig-trace'?: DryrunStateDto[]
  'logic-sig-messages'?: string[]
  'app-call-trace'?: DryrunStateDto[]
  'app-call-messages'?: string[]
  'global-delta'?: StateDeltaDto
  'local-deltas'?: AccountStateDeltaDto[]
  logs?: string[]
  'budget-added'?: bigint
  'budget-consumed'?: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: DryrunTxnResult): DryrunTxnResultDto {
  const out: any = {}
  {
    const v = (value as any)['disassembly']
    if (v === undefined) {
      // omit undefined
    } else {
      out['disassembly'] = v as any[]
    }
  }
  {
    const v = (value as any)['logicSigDisassembly']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logic-sig-disassembly'] = v as any[]
    }
  }
  {
    const v = (value as any)['logicSigTrace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logic-sig-trace'] = (v as any[]).map((item) => DryrunStateModel.toDto(item))
    }
  }
  {
    const v = (value as any)['logicSigMessages']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logic-sig-messages'] = v as any[]
    }
  }
  {
    const v = (value as any)['appCallTrace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-call-trace'] = (v as any[]).map((item) => DryrunStateModel.toDto(item))
    }
  }
  {
    const v = (value as any)['appCallMessages']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-call-messages'] = v as any[]
    }
  }
  {
    const v = (value as any)['globalDelta']
    if (v === undefined) {
      // omit undefined
    } else {
      out['global-delta'] = v === undefined ? v : StateDeltaModel.toDto(v)
    }
  }
  {
    const v = (value as any)['localDeltas']
    if (v === undefined) {
      // omit undefined
    } else {
      out['local-deltas'] = (v as any[]).map((item) => AccountStateDeltaModel.toDto(item))
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
  {
    const v = (value as any)['budgetAdded']
    if (v === undefined) {
      // omit undefined
    } else {
      out['budget-added'] = v
    }
  }
  {
    const v = (value as any)['budgetConsumed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['budget-consumed'] = v
    }
  }
  return out as DryrunTxnResultDto
}

export function fromDto(dto: DryrunTxnResultDto): DryrunTxnResult {
  const out: any = {}
  {
    const v = (dto as any)['disassembly']
    if (v === undefined) {
      // omit undefined
    } else {
      out['disassembly'] = v as any[]
    }
  }
  {
    const v = (dto as any)['logic-sig-disassembly']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logicSigDisassembly'] = v as any[]
    }
  }
  {
    const v = (dto as any)['logic-sig-trace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logicSigTrace'] = (v as any[]).map((item) => DryrunStateModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['logic-sig-messages']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logicSigMessages'] = v as any[]
    }
  }
  {
    const v = (dto as any)['app-call-trace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appCallTrace'] = (v as any[]).map((item) => DryrunStateModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['app-call-messages']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appCallMessages'] = v as any[]
    }
  }
  {
    const v = (dto as any)['global-delta']
    if (v === undefined) {
      // omit undefined
    } else {
      out['globalDelta'] = v === undefined ? v : StateDeltaModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['local-deltas']
    if (v === undefined) {
      // omit undefined
    } else {
      out['localDeltas'] = (v as any[]).map((item) => AccountStateDeltaModel.fromDto(item))
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
  {
    const v = (dto as any)['budget-added']
    if (v === undefined) {
      // omit undefined
    } else {
      out['budgetAdded'] = v as any
    }
  }
  {
    const v = (dto as any)['budget-consumed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['budgetConsumed'] = v as any
    }
  }
  return out as DryrunTxnResult
}

// Msgpack codecs
export function encodeMsgpack(value: DryrunTxnResult): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): DryrunTxnResult {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: DryrunTxnResult): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): DryrunTxnResult {
  return fromDto(raw as DryrunTxnResultDto)
}

// Array helpers
export function encodeMsgpackArray(values: DryrunTxnResult[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): DryrunTxnResult[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: DryrunTxnResult[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): DryrunTxnResult[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type DryrunTxnResultMsgpackDto = {
  disassembly: string[]
  'logic-sig-disassembly'?: string[]
  'logic-sig-trace'?: ReturnType<(typeof DryrunStateModel)['toMsgpackDto']>[]
  'logic-sig-messages'?: string[]
  'app-call-trace'?: ReturnType<(typeof DryrunStateModel)['toMsgpackDto']>[]
  'app-call-messages'?: string[]
  'global-delta'?: ReturnType<(typeof StateDeltaModel)['toMsgpackDto']>
  'local-deltas'?: ReturnType<(typeof AccountStateDeltaModel)['toMsgpackDto']>[]
  logs?: Uint8Array[]
  'budget-added'?: bigint
  'budget-consumed'?: bigint
}

function toMsgpackDto(value: DryrunTxnResult): DryrunTxnResultMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['disassembly']
    if (v === undefined) {
    } else {
      out['disassembly'] = v as any[]
    }
  }
  {
    const v = (value as any)['logicSigDisassembly']
    if (v === undefined) {
    } else {
      out['logic-sig-disassembly'] = v as any[]
    }
  }
  {
    const v = (value as any)['logicSigTrace']
    if (v === undefined) {
    } else {
      out['logic-sig-trace'] = (v as any[]).map((item) => DryrunStateModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['logicSigMessages']
    if (v === undefined) {
    } else {
      out['logic-sig-messages'] = v as any[]
    }
  }
  {
    const v = (value as any)['appCallTrace']
    if (v === undefined) {
    } else {
      out['app-call-trace'] = (v as any[]).map((item) => DryrunStateModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['appCallMessages']
    if (v === undefined) {
    } else {
      out['app-call-messages'] = v as any[]
    }
  }
  {
    const v = (value as any)['globalDelta']
    if (v === undefined) {
    } else {
      out['global-delta'] = StateDeltaModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['localDeltas']
    if (v === undefined) {
    } else {
      out['local-deltas'] = (v as any[]).map((item) => AccountStateDeltaModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['logs']
    if (v === undefined) {
    } else {
      out['logs'] = v as any[]
    }
  }
  {
    const v = (value as any)['budgetAdded']
    if (v === undefined) {
    } else {
      out['budget-added'] = v
    }
  }
  {
    const v = (value as any)['budgetConsumed']
    if (v === undefined) {
    } else {
      out['budget-consumed'] = v
    }
  }
  return out as DryrunTxnResultMsgpackDto
}

function fromMsgpackDto(dto: DryrunTxnResultMsgpackDto): DryrunTxnResult {
  const out: any = {}
  {
    const v = (dto as any)['disassembly']
    if (v === undefined) {
    } else {
      out['disassembly'] = v as any[]
    }
  }
  {
    const v = (dto as any)['logic-sig-disassembly']
    if (v === undefined) {
    } else {
      out['logicSigDisassembly'] = v as any[]
    }
  }
  {
    const v = (dto as any)['logic-sig-trace']
    if (v === undefined) {
    } else {
      out['logicSigTrace'] = (v as any[]).map((item) => DryrunStateModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['logic-sig-messages']
    if (v === undefined) {
    } else {
      out['logicSigMessages'] = v as any[]
    }
  }
  {
    const v = (dto as any)['app-call-trace']
    if (v === undefined) {
    } else {
      out['appCallTrace'] = (v as any[]).map((item) => DryrunStateModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['app-call-messages']
    if (v === undefined) {
    } else {
      out['appCallMessages'] = v as any[]
    }
  }
  {
    const v = (dto as any)['global-delta']
    if (v === undefined) {
    } else {
      out['globalDelta'] = StateDeltaModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['local-deltas']
    if (v === undefined) {
    } else {
      out['localDeltas'] = (v as any[]).map((item) => AccountStateDeltaModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['logs']
    if (v === undefined) {
    } else {
      out['logs'] = v as any[]
    }
  }
  {
    const v = (dto as any)['budget-added']
    if (v === undefined) {
    } else {
      out['budgetAdded'] = v
    }
  }
  {
    const v = (dto as any)['budget-consumed']
    if (v === undefined) {
    } else {
      out['budgetConsumed'] = v
    }
  }
  return out as DryrunTxnResult
}

export const DryrunTxnResult = {
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
