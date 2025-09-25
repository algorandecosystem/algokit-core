import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { SimulationOpcodeTraceUnit, SimulationOpcodeTraceUnitDto } from './simulation-opcode-trace-unit'
import { SimulationOpcodeTraceUnit as SimulationOpcodeTraceUnitModel } from './simulation-opcode-trace-unit'

/**
 * The execution trace of calling an app or a logic sig, containing the inner app call trace in a recursive way.
 */
export type SimulationTransactionExecTrace = {
  /**
   * Program trace that contains a trace of opcode effects in an approval program.
   */
  approvalProgramTrace?: SimulationOpcodeTraceUnit[]

  /**
   * SHA512_256 hash digest of the approval program executed in transaction.
   */
  approvalProgramHash?: Uint8Array

  /**
   * Program trace that contains a trace of opcode effects in a clear state program.
   */
  clearStateProgramTrace?: SimulationOpcodeTraceUnit[]

  /**
   * SHA512_256 hash digest of the clear state program executed in transaction.
   */
  clearStateProgramHash?: Uint8Array

  /**
   * If true, indicates that the clear state program failed and any persistent state changes it produced should be reverted once the program exits.
   */
  clearStateRollback?: boolean

  /**
   * The error message explaining why the clear state program failed. This field will only be populated if clear-state-rollback is true and the failure was due to an execution error.
   */
  clearStateRollbackError?: string

  /**
   * Program trace that contains a trace of opcode effects in a logic sig.
   */
  logicSigTrace?: SimulationOpcodeTraceUnit[]

  /**
   * SHA512_256 hash digest of the logic sig executed in transaction.
   */
  logicSigHash?: Uint8Array

  /**
   * An array of SimulationTransactionExecTrace representing the execution trace of any inner transactions executed.
   */
  innerTrace?: SimulationTransactionExecTrace[]
}

// JSON DTO shape for SimulationTransactionExecTrace with wire keys and JSON-safe primitives
export type SimulationTransactionExecTraceDto = {
  'approval-program-trace'?: SimulationOpcodeTraceUnitDto[]
  'approval-program-hash'?: string
  'clear-state-program-trace'?: SimulationOpcodeTraceUnitDto[]
  'clear-state-program-hash'?: string
  'clear-state-rollback'?: boolean
  'clear-state-rollback-error'?: string
  'logic-sig-trace'?: SimulationOpcodeTraceUnitDto[]
  'logic-sig-hash'?: string
  'inner-trace'?: SimulationTransactionExecTraceDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulationTransactionExecTrace): SimulationTransactionExecTraceDto {
  const out: any = {}
  {
    const v = (value as any)['approvalProgramTrace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['approval-program-trace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.toDto(item))
    }
  }
  {
    const v = (value as any)['approvalProgramHash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['approval-program-hash'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['clearStateProgramTrace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clear-state-program-trace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.toDto(item))
    }
  }
  {
    const v = (value as any)['clearStateProgramHash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clear-state-program-hash'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['clearStateRollback']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clear-state-rollback'] = v
    }
  }
  {
    const v = (value as any)['clearStateRollbackError']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clear-state-rollback-error'] = v
    }
  }
  {
    const v = (value as any)['logicSigTrace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logic-sig-trace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.toDto(item))
    }
  }
  {
    const v = (value as any)['logicSigHash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logic-sig-hash'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['innerTrace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['inner-trace'] = (v as any[]).map((item) => SimulationTransactionExecTraceModel.toDto(item))
    }
  }
  return out as SimulationTransactionExecTraceDto
}

export function fromDto(dto: SimulationTransactionExecTraceDto): SimulationTransactionExecTrace {
  const out: any = {}
  {
    const v = (dto as any)['approval-program-trace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['approvalProgramTrace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['approval-program-hash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['approvalProgramHash'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['clear-state-program-trace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clearStateProgramTrace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['clear-state-program-hash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clearStateProgramHash'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['clear-state-rollback']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clearStateRollback'] = v as any
    }
  }
  {
    const v = (dto as any)['clear-state-rollback-error']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clearStateRollbackError'] = v as any
    }
  }
  {
    const v = (dto as any)['logic-sig-trace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logicSigTrace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['logic-sig-hash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logicSigHash'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['inner-trace']
    if (v === undefined) {
      // omit undefined
    } else {
      out['innerTrace'] = (v as any[]).map((item) => SimulationTransactionExecTraceModel.fromDto(item))
    }
  }
  return out as SimulationTransactionExecTrace
}

// Msgpack codecs
export function encodeMsgpack(value: SimulationTransactionExecTrace): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulationTransactionExecTrace {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulationTransactionExecTrace): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulationTransactionExecTrace {
  return fromDto(raw as SimulationTransactionExecTraceDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulationTransactionExecTrace[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulationTransactionExecTrace[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulationTransactionExecTrace[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulationTransactionExecTrace[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulationTransactionExecTraceMsgpackDto = {
  'approval-program-trace'?: ReturnType<(typeof SimulationOpcodeTraceUnitModel)['toMsgpackDto']>[]
  'approval-program-hash'?: Uint8Array
  'clear-state-program-trace'?: ReturnType<(typeof SimulationOpcodeTraceUnitModel)['toMsgpackDto']>[]
  'clear-state-program-hash'?: Uint8Array
  'clear-state-rollback'?: boolean
  'clear-state-rollback-error'?: string
  'logic-sig-trace'?: ReturnType<(typeof SimulationOpcodeTraceUnitModel)['toMsgpackDto']>[]
  'logic-sig-hash'?: Uint8Array
  'inner-trace'?: ReturnType<(typeof SimulationTransactionExecTraceModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: SimulationTransactionExecTrace): SimulationTransactionExecTraceMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['approvalProgramTrace']
    if (v === undefined) {
    } else {
      out['approval-program-trace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['approvalProgramHash']
    if (v === undefined) {
    } else {
      out['approval-program-hash'] = v
    }
  }
  {
    const v = (value as any)['clearStateProgramTrace']
    if (v === undefined) {
    } else {
      out['clear-state-program-trace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['clearStateProgramHash']
    if (v === undefined) {
    } else {
      out['clear-state-program-hash'] = v
    }
  }
  {
    const v = (value as any)['clearStateRollback']
    if (v === undefined) {
    } else {
      out['clear-state-rollback'] = v
    }
  }
  {
    const v = (value as any)['clearStateRollbackError']
    if (v === undefined) {
    } else {
      out['clear-state-rollback-error'] = v
    }
  }
  {
    const v = (value as any)['logicSigTrace']
    if (v === undefined) {
    } else {
      out['logic-sig-trace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['logicSigHash']
    if (v === undefined) {
    } else {
      out['logic-sig-hash'] = v
    }
  }
  {
    const v = (value as any)['innerTrace']
    if (v === undefined) {
    } else {
      out['inner-trace'] = (v as any[]).map((item) => SimulationTransactionExecTraceModel.toMsgpackDto(item))
    }
  }
  return out as SimulationTransactionExecTraceMsgpackDto
}

function fromMsgpackDto(dto: SimulationTransactionExecTraceMsgpackDto): SimulationTransactionExecTrace {
  const out: any = {}
  {
    const v = (dto as any)['approval-program-trace']
    if (v === undefined) {
    } else {
      out['approvalProgramTrace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['approval-program-hash']
    if (v === undefined) {
    } else {
      out['approvalProgramHash'] = v
    }
  }
  {
    const v = (dto as any)['clear-state-program-trace']
    if (v === undefined) {
    } else {
      out['clearStateProgramTrace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['clear-state-program-hash']
    if (v === undefined) {
    } else {
      out['clearStateProgramHash'] = v
    }
  }
  {
    const v = (dto as any)['clear-state-rollback']
    if (v === undefined) {
    } else {
      out['clearStateRollback'] = v
    }
  }
  {
    const v = (dto as any)['clear-state-rollback-error']
    if (v === undefined) {
    } else {
      out['clearStateRollbackError'] = v
    }
  }
  {
    const v = (dto as any)['logic-sig-trace']
    if (v === undefined) {
    } else {
      out['logicSigTrace'] = (v as any[]).map((item) => SimulationOpcodeTraceUnitModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['logic-sig-hash']
    if (v === undefined) {
    } else {
      out['logicSigHash'] = v
    }
  }
  {
    const v = (dto as any)['inner-trace']
    if (v === undefined) {
    } else {
      out['innerTrace'] = (v as any[]).map((item) => SimulationTransactionExecTraceModel.fromMsgpackDto(item))
    }
  }
  return out as SimulationTransactionExecTrace
}

export const SimulationTransactionExecTrace = {
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
