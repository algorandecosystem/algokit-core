import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * The set of parameters and limits override during simulation. If this set of parameters is present, then evaluation parameters may differ from standard evaluation in certain ways.
 */
export type SimulationEvalOverrides = {
  /**
   * If true, transactions without signatures are allowed and simulated as if they were properly signed.
   */
  allowEmptySignatures?: boolean

  /**
   * If true, allows access to unnamed resources during simulation.
   */
  allowUnnamedResources?: boolean

  /**
   * The maximum log calls one can make during simulation
   */
  maxLogCalls?: bigint

  /**
   * The maximum byte number to log during simulation
   */
  maxLogSize?: bigint

  /**
   * The extra opcode budget added to each transaction group during simulation
   */
  extraOpcodeBudget?: bigint

  /**
   * If true, signers for transactions that are missing signatures will be fixed during evaluation.
   */
  fixSigners?: boolean
}

// JSON DTO shape for SimulationEvalOverrides with wire keys and JSON-safe primitives
export type SimulationEvalOverridesDto = {
  'allow-empty-signatures'?: boolean
  'allow-unnamed-resources'?: boolean
  'max-log-calls'?: bigint
  'max-log-size'?: bigint
  'extra-opcode-budget'?: bigint
  'fix-signers'?: boolean
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulationEvalOverrides): SimulationEvalOverridesDto {
  const out: any = {}
  {
    const v = (value as any)['allowEmptySignatures']
    if (v === undefined) {
      // omit undefined
    } else {
      out['allow-empty-signatures'] = v
    }
  }
  {
    const v = (value as any)['allowUnnamedResources']
    if (v === undefined) {
      // omit undefined
    } else {
      out['allow-unnamed-resources'] = v
    }
  }
  {
    const v = (value as any)['maxLogCalls']
    if (v === undefined) {
      // omit undefined
    } else {
      out['max-log-calls'] = v
    }
  }
  {
    const v = (value as any)['maxLogSize']
    if (v === undefined) {
      // omit undefined
    } else {
      out['max-log-size'] = v
    }
  }
  {
    const v = (value as any)['extraOpcodeBudget']
    if (v === undefined) {
      // omit undefined
    } else {
      out['extra-opcode-budget'] = v
    }
  }
  {
    const v = (value as any)['fixSigners']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fix-signers'] = v
    }
  }
  return out as SimulationEvalOverridesDto
}

export function fromDto(dto: SimulationEvalOverridesDto): SimulationEvalOverrides {
  const out: any = {}
  {
    const v = (dto as any)['allow-empty-signatures']
    if (v === undefined) {
      // omit undefined
    } else {
      out['allowEmptySignatures'] = v as any
    }
  }
  {
    const v = (dto as any)['allow-unnamed-resources']
    if (v === undefined) {
      // omit undefined
    } else {
      out['allowUnnamedResources'] = v as any
    }
  }
  {
    const v = (dto as any)['max-log-calls']
    if (v === undefined) {
      // omit undefined
    } else {
      out['maxLogCalls'] = v as any
    }
  }
  {
    const v = (dto as any)['max-log-size']
    if (v === undefined) {
      // omit undefined
    } else {
      out['maxLogSize'] = v as any
    }
  }
  {
    const v = (dto as any)['extra-opcode-budget']
    if (v === undefined) {
      // omit undefined
    } else {
      out['extraOpcodeBudget'] = v as any
    }
  }
  {
    const v = (dto as any)['fix-signers']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fixSigners'] = v as any
    }
  }
  return out as SimulationEvalOverrides
}

// Msgpack codecs
export function encodeMsgpack(value: SimulationEvalOverrides): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulationEvalOverrides {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulationEvalOverrides): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulationEvalOverrides {
  return fromDto(raw as SimulationEvalOverridesDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulationEvalOverrides[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulationEvalOverrides[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulationEvalOverrides[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulationEvalOverrides[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulationEvalOverridesMsgpackDto = {
  'allow-empty-signatures'?: boolean
  'allow-unnamed-resources'?: boolean
  'max-log-calls'?: bigint
  'max-log-size'?: bigint
  'extra-opcode-budget'?: bigint
  'fix-signers'?: boolean
}

function toMsgpackDto(value: SimulationEvalOverrides): SimulationEvalOverridesMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['allowEmptySignatures']
    if (v === undefined) {
    } else {
      out['allow-empty-signatures'] = v
    }
  }
  {
    const v = (value as any)['allowUnnamedResources']
    if (v === undefined) {
    } else {
      out['allow-unnamed-resources'] = v
    }
  }
  {
    const v = (value as any)['maxLogCalls']
    if (v === undefined) {
    } else {
      out['max-log-calls'] = v
    }
  }
  {
    const v = (value as any)['maxLogSize']
    if (v === undefined) {
    } else {
      out['max-log-size'] = v
    }
  }
  {
    const v = (value as any)['extraOpcodeBudget']
    if (v === undefined) {
    } else {
      out['extra-opcode-budget'] = v
    }
  }
  {
    const v = (value as any)['fixSigners']
    if (v === undefined) {
    } else {
      out['fix-signers'] = v
    }
  }
  return out as SimulationEvalOverridesMsgpackDto
}

function fromMsgpackDto(dto: SimulationEvalOverridesMsgpackDto): SimulationEvalOverrides {
  const out: any = {}
  {
    const v = (dto as any)['allow-empty-signatures']
    if (v === undefined) {
    } else {
      out['allowEmptySignatures'] = v
    }
  }
  {
    const v = (dto as any)['allow-unnamed-resources']
    if (v === undefined) {
    } else {
      out['allowUnnamedResources'] = v
    }
  }
  {
    const v = (dto as any)['max-log-calls']
    if (v === undefined) {
    } else {
      out['maxLogCalls'] = v
    }
  }
  {
    const v = (dto as any)['max-log-size']
    if (v === undefined) {
    } else {
      out['maxLogSize'] = v
    }
  }
  {
    const v = (dto as any)['extra-opcode-budget']
    if (v === undefined) {
    } else {
      out['extraOpcodeBudget'] = v
    }
  }
  {
    const v = (dto as any)['fix-signers']
    if (v === undefined) {
    } else {
      out['fixSigners'] = v
    }
  }
  return out as SimulationEvalOverrides
}

export const SimulationEvalOverrides = {
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
