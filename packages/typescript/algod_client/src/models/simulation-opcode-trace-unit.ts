import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { ApplicationStateOperation, ApplicationStateOperationDto } from './application-state-operation'
import { ApplicationStateOperation as ApplicationStateOperationModel } from './application-state-operation'
import type { AvmValue, AvmValueDto } from './avm-value'
import { AvmValue as AvmValueModel } from './avm-value'
import type { ScratchChange, ScratchChangeDto } from './scratch-change'
import { ScratchChange as ScratchChangeModel } from './scratch-change'

/**
 * The set of trace information and effect from evaluating a single opcode.
 */
export type SimulationOpcodeTraceUnit = {
  /**
   * The program counter of the current opcode being evaluated.
   */
  pc: bigint

  /**
   * The writes into scratch slots.
   */
  scratchChanges?: ScratchChange[]

  /**
   * The operations against the current application's states.
   */
  stateChanges?: ApplicationStateOperation[]

  /**
   * The indexes of the traces for inner transactions spawned by this opcode, if any.
   */
  spawnedInners?: bigint[]

  /**
   * The number of deleted stack values by this opcode.
   */
  stackPopCount?: bigint

  /**
   * The values added by this opcode to the stack.
   */
  stackAdditions?: AvmValue[]
}

// JSON DTO shape for SimulationOpcodeTraceUnit with wire keys and JSON-safe primitives
export type SimulationOpcodeTraceUnitDto = {
  pc: bigint
  'scratch-changes'?: ScratchChangeDto[]
  'state-changes'?: ApplicationStateOperationDto[]
  'spawned-inners'?: bigint[][]
  'stack-pop-count'?: bigint
  'stack-additions'?: AvmValueDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulationOpcodeTraceUnit): SimulationOpcodeTraceUnitDto {
  const out: any = {}
  {
    const v = (value as any)['pc']
    if (v === undefined) {
      // omit undefined
    } else {
      out['pc'] = v
    }
  }
  {
    const v = (value as any)['scratchChanges']
    if (v === undefined) {
      // omit undefined
    } else {
      out['scratch-changes'] = (v as any[]).map((item) => ScratchChangeModel.toDto(item))
    }
  }
  {
    const v = (value as any)['stateChanges']
    if (v === undefined) {
      // omit undefined
    } else {
      out['state-changes'] = (v as any[]).map((item) => ApplicationStateOperationModel.toDto(item))
    }
  }
  {
    const v = (value as any)['spawnedInners']
    if (v === undefined) {
      // omit undefined
    } else {
      out['spawned-inners'] = v as any[]
    }
  }
  {
    const v = (value as any)['stackPopCount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stack-pop-count'] = v
    }
  }
  {
    const v = (value as any)['stackAdditions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stack-additions'] = (v as any[]).map((item) => AvmValueModel.toDto(item))
    }
  }
  return out as SimulationOpcodeTraceUnitDto
}

export function fromDto(dto: SimulationOpcodeTraceUnitDto): SimulationOpcodeTraceUnit {
  const out: any = {}
  {
    const v = (dto as any)['pc']
    if (v === undefined) {
      // omit undefined
    } else {
      out['pc'] = v as any
    }
  }
  {
    const v = (dto as any)['scratch-changes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['scratchChanges'] = (v as any[]).map((item) => ScratchChangeModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['state-changes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stateChanges'] = (v as any[]).map((item) => ApplicationStateOperationModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['spawned-inners']
    if (v === undefined) {
      // omit undefined
    } else {
      out['spawnedInners'] = v as any[]
    }
  }
  {
    const v = (dto as any)['stack-pop-count']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stackPopCount'] = v as any
    }
  }
  {
    const v = (dto as any)['stack-additions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stackAdditions'] = (v as any[]).map((item) => AvmValueModel.fromDto(item))
    }
  }
  return out as SimulationOpcodeTraceUnit
}

// Msgpack codecs
export function encodeMsgpack(value: SimulationOpcodeTraceUnit): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulationOpcodeTraceUnit {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulationOpcodeTraceUnit): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulationOpcodeTraceUnit {
  return fromDto(raw as SimulationOpcodeTraceUnitDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulationOpcodeTraceUnit[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulationOpcodeTraceUnit[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulationOpcodeTraceUnit[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulationOpcodeTraceUnit[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulationOpcodeTraceUnitMsgpackDto = {
  pc: bigint
  'scratch-changes'?: ReturnType<(typeof ScratchChangeModel)['toMsgpackDto']>[]
  'state-changes'?: ReturnType<(typeof ApplicationStateOperationModel)['toMsgpackDto']>[]
  'spawned-inners'?: bigint[][]
  'stack-pop-count'?: bigint
  'stack-additions'?: ReturnType<(typeof AvmValueModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: SimulationOpcodeTraceUnit): SimulationOpcodeTraceUnitMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['pc']
    if (v === undefined) {
    } else {
      out['pc'] = v
    }
  }
  {
    const v = (value as any)['scratchChanges']
    if (v === undefined) {
    } else {
      out['scratch-changes'] = (v as any[]).map((item) => ScratchChangeModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['stateChanges']
    if (v === undefined) {
    } else {
      out['state-changes'] = (v as any[]).map((item) => ApplicationStateOperationModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['spawnedInners']
    if (v === undefined) {
    } else {
      out['spawned-inners'] = v as any[]
    }
  }
  {
    const v = (value as any)['stackPopCount']
    if (v === undefined) {
    } else {
      out['stack-pop-count'] = v
    }
  }
  {
    const v = (value as any)['stackAdditions']
    if (v === undefined) {
    } else {
      out['stack-additions'] = (v as any[]).map((item) => AvmValueModel.toMsgpackDto(item))
    }
  }
  return out as SimulationOpcodeTraceUnitMsgpackDto
}

function fromMsgpackDto(dto: SimulationOpcodeTraceUnitMsgpackDto): SimulationOpcodeTraceUnit {
  const out: any = {}
  {
    const v = (dto as any)['pc']
    if (v === undefined) {
    } else {
      out['pc'] = v
    }
  }
  {
    const v = (dto as any)['scratch-changes']
    if (v === undefined) {
    } else {
      out['scratchChanges'] = (v as any[]).map((item) => ScratchChangeModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['state-changes']
    if (v === undefined) {
    } else {
      out['stateChanges'] = (v as any[]).map((item) => ApplicationStateOperationModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['spawned-inners']
    if (v === undefined) {
    } else {
      out['spawnedInners'] = v as any[]
    }
  }
  {
    const v = (dto as any)['stack-pop-count']
    if (v === undefined) {
    } else {
      out['stackPopCount'] = v
    }
  }
  {
    const v = (dto as any)['stack-additions']
    if (v === undefined) {
    } else {
      out['stackAdditions'] = (v as any[]).map((item) => AvmValueModel.fromMsgpackDto(item))
    }
  }
  return out as SimulationOpcodeTraceUnit
}

export const SimulationOpcodeTraceUnit = {
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
