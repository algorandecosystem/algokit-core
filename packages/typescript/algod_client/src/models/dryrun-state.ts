import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { TealValue, TealValueDto } from './teal-value'
import { TealValue as TealValueModel } from './teal-value'

/**
 * Stores the TEAL eval step data
 */
export type DryrunState = {
  /**
   * Line number
   */
  line: bigint

  /**
   * Program counter
   */
  pc: bigint
  stack: TealValue[]
  scratch?: TealValue[]

  /**
   * Evaluation error if any
   */
  error?: string
}

// JSON DTO shape for DryrunState with wire keys and JSON-safe primitives
export type DryrunStateDto = {
  line: bigint
  pc: bigint
  stack: TealValueDto[]
  scratch?: TealValueDto[]
  error?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: DryrunState): DryrunStateDto {
  const out: any = {}
  {
    const v = (value as any)['line']
    if (v === undefined) {
      // omit undefined
    } else {
      out['line'] = v
    }
  }
  {
    const v = (value as any)['pc']
    if (v === undefined) {
      // omit undefined
    } else {
      out['pc'] = v
    }
  }
  {
    const v = (value as any)['stack']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stack'] = (v as any[]).map((item) => TealValueModel.toDto(item))
    }
  }
  {
    const v = (value as any)['scratch']
    if (v === undefined) {
      // omit undefined
    } else {
      out['scratch'] = (v as any[]).map((item) => TealValueModel.toDto(item))
    }
  }
  {
    const v = (value as any)['error']
    if (v === undefined) {
      // omit undefined
    } else {
      out['error'] = v
    }
  }
  return out as DryrunStateDto
}

export function fromDto(dto: DryrunStateDto): DryrunState {
  const out: any = {}
  {
    const v = (dto as any)['line']
    if (v === undefined) {
      // omit undefined
    } else {
      out['line'] = v as any
    }
  }
  {
    const v = (dto as any)['pc']
    if (v === undefined) {
      // omit undefined
    } else {
      out['pc'] = v as any
    }
  }
  {
    const v = (dto as any)['stack']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stack'] = (v as any[]).map((item) => TealValueModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['scratch']
    if (v === undefined) {
      // omit undefined
    } else {
      out['scratch'] = (v as any[]).map((item) => TealValueModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['error']
    if (v === undefined) {
      // omit undefined
    } else {
      out['error'] = v as any
    }
  }
  return out as DryrunState
}

// Msgpack codecs
export function encodeMsgpack(value: DryrunState): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): DryrunState {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: DryrunState): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): DryrunState {
  return fromDto(raw as DryrunStateDto)
}

// Array helpers
export function encodeMsgpackArray(values: DryrunState[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): DryrunState[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: DryrunState[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): DryrunState[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type DryrunStateMsgpackDto = {
  line: bigint
  pc: bigint
  stack: ReturnType<(typeof TealValueModel)['toMsgpackDto']>[]
  scratch?: ReturnType<(typeof TealValueModel)['toMsgpackDto']>[]
  error?: string
}

function toMsgpackDto(value: DryrunState): DryrunStateMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['line']
    if (v === undefined) {
    } else {
      out['line'] = v
    }
  }
  {
    const v = (value as any)['pc']
    if (v === undefined) {
    } else {
      out['pc'] = v
    }
  }
  {
    const v = (value as any)['stack']
    if (v === undefined) {
    } else {
      out['stack'] = (v as any[]).map((item) => TealValueModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['scratch']
    if (v === undefined) {
    } else {
      out['scratch'] = (v as any[]).map((item) => TealValueModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['error']
    if (v === undefined) {
    } else {
      out['error'] = v
    }
  }
  return out as DryrunStateMsgpackDto
}

function fromMsgpackDto(dto: DryrunStateMsgpackDto): DryrunState {
  const out: any = {}
  {
    const v = (dto as any)['line']
    if (v === undefined) {
    } else {
      out['line'] = v
    }
  }
  {
    const v = (dto as any)['pc']
    if (v === undefined) {
    } else {
      out['pc'] = v
    }
  }
  {
    const v = (dto as any)['stack']
    if (v === undefined) {
    } else {
      out['stack'] = (v as any[]).map((item) => TealValueModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['scratch']
    if (v === undefined) {
    } else {
      out['scratch'] = (v as any[]).map((item) => TealValueModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['error']
    if (v === undefined) {
    } else {
      out['error'] = v
    }
  }
  return out as DryrunState
}

export const DryrunState = {
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
