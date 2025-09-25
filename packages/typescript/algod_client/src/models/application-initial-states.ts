import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { ApplicationKvstorage, ApplicationKvstorageDto } from './application-kvstorage'
import { ApplicationKvstorage as ApplicationKvstorageModel } from './application-kvstorage'

/**
 * An application's initial global/local/box states that were accessed during simulation.
 */
export type ApplicationInitialStates = {
  /**
   * Application index.
   */
  id: bigint

  /**
   * An application's initial local states tied to different accounts.
   */
  appLocals?: ApplicationKvstorage[]
  appGlobals?: ApplicationKvstorage
  appBoxes?: ApplicationKvstorage
}

// JSON DTO shape for ApplicationInitialStates with wire keys and JSON-safe primitives
export type ApplicationInitialStatesDto = {
  id: string
  'app-locals'?: ApplicationKvstorageDto[]
  'app-globals'?: ApplicationKvstorageDto
  'app-boxes'?: ApplicationKvstorageDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ApplicationInitialStates): ApplicationInitialStatesDto {
  const out: any = {}
  {
    const v = (value as any)['id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['id'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['appLocals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-locals'] = (v as any[]).map((item) => ApplicationKvstorageModel.toDto(item))
    }
  }
  {
    const v = (value as any)['appGlobals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-globals'] = v === undefined ? v : ApplicationKvstorageModel.toDto(v)
    }
  }
  {
    const v = (value as any)['appBoxes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-boxes'] = v === undefined ? v : ApplicationKvstorageModel.toDto(v)
    }
  }
  return out as ApplicationInitialStatesDto
}

export function fromDto(dto: ApplicationInitialStatesDto): ApplicationInitialStates {
  const out: any = {}
  {
    const v = (dto as any)['id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['id'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['app-locals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appLocals'] = (v as any[]).map((item) => ApplicationKvstorageModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['app-globals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appGlobals'] = v === undefined ? v : ApplicationKvstorageModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['app-boxes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appBoxes'] = v === undefined ? v : ApplicationKvstorageModel.fromDto(v)
    }
  }
  return out as ApplicationInitialStates
}

// Msgpack codecs
export function encodeMsgpack(value: ApplicationInitialStates): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ApplicationInitialStates {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ApplicationInitialStates): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ApplicationInitialStates {
  return fromDto(raw as ApplicationInitialStatesDto)
}

// Array helpers
export function encodeMsgpackArray(values: ApplicationInitialStates[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ApplicationInitialStates[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ApplicationInitialStates[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ApplicationInitialStates[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ApplicationInitialStatesMsgpackDto = {
  id: bigint
  'app-locals'?: ReturnType<(typeof ApplicationKvstorageModel)['toMsgpackDto']>[]
  'app-globals'?: ReturnType<(typeof ApplicationKvstorageModel)['toMsgpackDto']>
  'app-boxes'?: ReturnType<(typeof ApplicationKvstorageModel)['toMsgpackDto']>
}

function toMsgpackDto(value: ApplicationInitialStates): ApplicationInitialStatesMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['id']
    if (v === undefined) {
    } else {
      out['id'] = v
    }
  }
  {
    const v = (value as any)['appLocals']
    if (v === undefined) {
    } else {
      out['app-locals'] = (v as any[]).map((item) => ApplicationKvstorageModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['appGlobals']
    if (v === undefined) {
    } else {
      out['app-globals'] = ApplicationKvstorageModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['appBoxes']
    if (v === undefined) {
    } else {
      out['app-boxes'] = ApplicationKvstorageModel.toMsgpackDto(v)
    }
  }
  return out as ApplicationInitialStatesMsgpackDto
}

function fromMsgpackDto(dto: ApplicationInitialStatesMsgpackDto): ApplicationInitialStates {
  const out: any = {}
  {
    const v = (dto as any)['id']
    if (v === undefined) {
    } else {
      out['id'] = v
    }
  }
  {
    const v = (dto as any)['app-locals']
    if (v === undefined) {
    } else {
      out['appLocals'] = (v as any[]).map((item) => ApplicationKvstorageModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['app-globals']
    if (v === undefined) {
    } else {
      out['appGlobals'] = ApplicationKvstorageModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['app-boxes']
    if (v === undefined) {
    } else {
      out['appBoxes'] = ApplicationKvstorageModel.fromMsgpackDto(v)
    }
  }
  return out as ApplicationInitialStates
}

export const ApplicationInitialStates = {
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
