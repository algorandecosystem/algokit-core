import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * algod mutex and blocking profiling state.
 */
export type DebugSettingsProf = {
  /**
   * The rate of blocking events. The profiler aims to sample an average of one blocking event per rate nanoseconds spent blocked. To turn off profiling entirely, pass rate 0.
   */
  blockRate?: bigint

  /**
   * The rate of mutex events. On average 1/rate events are reported. To turn off profiling entirely, pass rate 0
   */
  mutexRate?: bigint
}

// JSON DTO shape for DebugSettingsProf with wire keys and JSON-safe primitives
export type DebugSettingsProfDto = {
  'block-rate'?: string
  'mutex-rate'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: DebugSettingsProf): DebugSettingsProfDto {
  const out: any = {}
  {
    const v = (value as any)['blockRate']
    if (v === undefined) {
      // omit undefined
    } else {
      out['block-rate'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['mutexRate']
    if (v === undefined) {
      // omit undefined
    } else {
      out['mutex-rate'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as DebugSettingsProfDto
}

export function fromDto(dto: DebugSettingsProfDto): DebugSettingsProf {
  const out: any = {}
  {
    const v = (dto as any)['block-rate']
    if (v === undefined) {
      // omit undefined
    } else {
      out['blockRate'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['mutex-rate']
    if (v === undefined) {
      // omit undefined
    } else {
      out['mutexRate'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as DebugSettingsProf
}

// Msgpack codecs
export function encodeMsgpack(value: DebugSettingsProf): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): DebugSettingsProf {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: DebugSettingsProf): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): DebugSettingsProf {
  return fromDto(raw as DebugSettingsProfDto)
}

// Array helpers
export function encodeMsgpackArray(values: DebugSettingsProf[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): DebugSettingsProf[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: DebugSettingsProf[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): DebugSettingsProf[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type DebugSettingsProfMsgpackDto = {
  'block-rate'?: bigint
  'mutex-rate'?: bigint
}

function toMsgpackDto(value: DebugSettingsProf): DebugSettingsProfMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['blockRate']
    if (v === undefined) {
    } else {
      out['block-rate'] = v
    }
  }
  {
    const v = (value as any)['mutexRate']
    if (v === undefined) {
    } else {
      out['mutex-rate'] = v
    }
  }
  return out as DebugSettingsProfMsgpackDto
}

function fromMsgpackDto(dto: DebugSettingsProfMsgpackDto): DebugSettingsProf {
  const out: any = {}
  {
    const v = (dto as any)['block-rate']
    if (v === undefined) {
    } else {
      out['blockRate'] = v
    }
  }
  {
    const v = (dto as any)['mutex-rate']
    if (v === undefined) {
    } else {
      out['mutexRate'] = v
    }
  }
  return out as DebugSettingsProf
}

export const DebugSettingsProf = {
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
