import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * An object that configures simulation execution trace.
 */
export type SimulateTraceConfig = {
  /**
   * A boolean option for opting in execution trace features simulation endpoint.
   */
  enable?: boolean

  /**
   * A boolean option enabling returning stack changes together with execution trace during simulation.
   */
  stackChange?: boolean

  /**
   * A boolean option enabling returning scratch slot changes together with execution trace during simulation.
   */
  scratchChange?: boolean

  /**
   * A boolean option enabling returning application state changes (global, local, and box changes) with the execution trace during simulation.
   */
  stateChange?: boolean
}

// JSON DTO shape for SimulateTraceConfig with wire keys and JSON-safe primitives
export type SimulateTraceConfigDto = {
  enable?: boolean
  'stack-change'?: boolean
  'scratch-change'?: boolean
  'state-change'?: boolean
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulateTraceConfig): SimulateTraceConfigDto {
  const out: any = {}
  {
    const v = (value as any)['enable']
    if (v === undefined) {
      // omit undefined
    } else {
      out['enable'] = v
    }
  }
  {
    const v = (value as any)['stackChange']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stack-change'] = v
    }
  }
  {
    const v = (value as any)['scratchChange']
    if (v === undefined) {
      // omit undefined
    } else {
      out['scratch-change'] = v
    }
  }
  {
    const v = (value as any)['stateChange']
    if (v === undefined) {
      // omit undefined
    } else {
      out['state-change'] = v
    }
  }
  return out as SimulateTraceConfigDto
}

export function fromDto(dto: SimulateTraceConfigDto): SimulateTraceConfig {
  const out: any = {}
  {
    const v = (dto as any)['enable']
    if (v === undefined) {
      // omit undefined
    } else {
      out['enable'] = v as any
    }
  }
  {
    const v = (dto as any)['stack-change']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stackChange'] = v as any
    }
  }
  {
    const v = (dto as any)['scratch-change']
    if (v === undefined) {
      // omit undefined
    } else {
      out['scratchChange'] = v as any
    }
  }
  {
    const v = (dto as any)['state-change']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stateChange'] = v as any
    }
  }
  return out as SimulateTraceConfig
}

// Msgpack codecs
export function encodeMsgpack(value: SimulateTraceConfig): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulateTraceConfig {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulateTraceConfig): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulateTraceConfig {
  return fromDto(raw as SimulateTraceConfigDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulateTraceConfig[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulateTraceConfig[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulateTraceConfig[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulateTraceConfig[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulateTraceConfigMsgpackDto = {
  enable?: boolean
  'stack-change'?: boolean
  'scratch-change'?: boolean
  'state-change'?: boolean
}

function toMsgpackDto(value: SimulateTraceConfig): SimulateTraceConfigMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['enable']
    if (v === undefined) {
    } else {
      out['enable'] = v
    }
  }
  {
    const v = (value as any)['stackChange']
    if (v === undefined) {
    } else {
      out['stack-change'] = v
    }
  }
  {
    const v = (value as any)['scratchChange']
    if (v === undefined) {
    } else {
      out['scratch-change'] = v
    }
  }
  {
    const v = (value as any)['stateChange']
    if (v === undefined) {
    } else {
      out['state-change'] = v
    }
  }
  return out as SimulateTraceConfigMsgpackDto
}

function fromMsgpackDto(dto: SimulateTraceConfigMsgpackDto): SimulateTraceConfig {
  const out: any = {}
  {
    const v = (dto as any)['enable']
    if (v === undefined) {
    } else {
      out['enable'] = v
    }
  }
  {
    const v = (dto as any)['stack-change']
    if (v === undefined) {
    } else {
      out['stackChange'] = v
    }
  }
  {
    const v = (dto as any)['scratch-change']
    if (v === undefined) {
    } else {
      out['scratchChange'] = v
    }
  }
  {
    const v = (dto as any)['state-change']
    if (v === undefined) {
    } else {
      out['stateChange'] = v
    }
  }
  return out as SimulateTraceConfig
}

export const SimulateTraceConfig = {
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
