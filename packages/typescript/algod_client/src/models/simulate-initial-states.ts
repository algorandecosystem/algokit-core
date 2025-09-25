import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { ApplicationInitialStates, ApplicationInitialStatesDto } from './application-initial-states'
import { ApplicationInitialStates as ApplicationInitialStatesModel } from './application-initial-states'

/**
 * Initial states of resources that were accessed during simulation.
 */
export type SimulateInitialStates = {
  /**
   * The initial states of accessed application before simulation. The order of this array is arbitrary.
   */
  appInitialStates?: ApplicationInitialStates[]
}

// JSON DTO shape for SimulateInitialStates with wire keys and JSON-safe primitives
export type SimulateInitialStatesDto = {
  'app-initial-states'?: ApplicationInitialStatesDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulateInitialStates): SimulateInitialStatesDto {
  const out: any = {}
  {
    const v = (value as any)['appInitialStates']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-initial-states'] = (v as any[]).map((item) => ApplicationInitialStatesModel.toDto(item))
    }
  }
  return out as SimulateInitialStatesDto
}

export function fromDto(dto: SimulateInitialStatesDto): SimulateInitialStates {
  const out: any = {}
  {
    const v = (dto as any)['app-initial-states']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appInitialStates'] = (v as any[]).map((item) => ApplicationInitialStatesModel.fromDto(item))
    }
  }
  return out as SimulateInitialStates
}

// Msgpack codecs
export function encodeMsgpack(value: SimulateInitialStates): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulateInitialStates {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulateInitialStates): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulateInitialStates {
  return fromDto(raw as SimulateInitialStatesDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulateInitialStates[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulateInitialStates[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulateInitialStates[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulateInitialStates[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulateInitialStatesMsgpackDto = {
  'app-initial-states'?: ReturnType<(typeof ApplicationInitialStatesModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: SimulateInitialStates): SimulateInitialStatesMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['appInitialStates']
    if (v === undefined) {
    } else {
      out['app-initial-states'] = (v as any[]).map((item) => ApplicationInitialStatesModel.toMsgpackDto(item))
    }
  }
  return out as SimulateInitialStatesMsgpackDto
}

function fromMsgpackDto(dto: SimulateInitialStatesMsgpackDto): SimulateInitialStates {
  const out: any = {}
  {
    const v = (dto as any)['app-initial-states']
    if (v === undefined) {
    } else {
      out['appInitialStates'] = (v as any[]).map((item) => ApplicationInitialStatesModel.fromMsgpackDto(item))
    }
  }
  return out as SimulateInitialStates
}

export const SimulateInitialStates = {
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
