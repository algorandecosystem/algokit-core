import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AvmValue, AvmValueDto } from './avm-value'
import { AvmValue as AvmValueModel } from './avm-value'

/**
 * An operation against an application's global/local/box state.
 */
export type ApplicationStateOperation = {
  /**
   * Operation type. Value `w` is **write**, `d` is **delete**.
   */
  operation: string

  /**
   * Type of application state. Value `g` is **global state**, `l` is **local state**, `b` is **boxes**.
   */
  appStateType: string

  /**
   * The key (name) of the global/local/box state.
   */
  key: Uint8Array
  newValue?: AvmValue

  /**
   * For local state changes, the address of the account associated with the local state.
   */
  account?: string
}

// JSON DTO shape for ApplicationStateOperation with wire keys and JSON-safe primitives
export type ApplicationStateOperationDto = {
  operation: string
  'app-state-type': string
  key: string
  'new-value'?: AvmValueDto
  account?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ApplicationStateOperation): ApplicationStateOperationDto {
  const out: any = {}
  {
    const v = (value as any)['operation']
    if (v === undefined) {
      // omit undefined
    } else {
      out['operation'] = v
    }
  }
  {
    const v = (value as any)['appStateType']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-state-type'] = v
    }
  }
  {
    const v = (value as any)['key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['key'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['newValue']
    if (v === undefined) {
      // omit undefined
    } else {
      out['new-value'] = v === undefined ? v : AvmValueModel.toDto(v)
    }
  }
  {
    const v = (value as any)['account']
    if (v === undefined) {
      // omit undefined
    } else {
      out['account'] = v
    }
  }
  return out as ApplicationStateOperationDto
}

export function fromDto(dto: ApplicationStateOperationDto): ApplicationStateOperation {
  const out: any = {}
  {
    const v = (dto as any)['operation']
    if (v === undefined) {
      // omit undefined
    } else {
      out['operation'] = v as any
    }
  }
  {
    const v = (dto as any)['app-state-type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appStateType'] = v as any
    }
  }
  {
    const v = (dto as any)['key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['key'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['new-value']
    if (v === undefined) {
      // omit undefined
    } else {
      out['newValue'] = v === undefined ? v : AvmValueModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['account']
    if (v === undefined) {
      // omit undefined
    } else {
      out['account'] = v as any
    }
  }
  return out as ApplicationStateOperation
}

// Msgpack codecs
export function encodeMsgpack(value: ApplicationStateOperation): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ApplicationStateOperation {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ApplicationStateOperation): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ApplicationStateOperation {
  return fromDto(raw as ApplicationStateOperationDto)
}

// Array helpers
export function encodeMsgpackArray(values: ApplicationStateOperation[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ApplicationStateOperation[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ApplicationStateOperation[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ApplicationStateOperation[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ApplicationStateOperationMsgpackDto = {
  operation: string
  'app-state-type': string
  key: Uint8Array
  'new-value'?: ReturnType<(typeof AvmValueModel)['toMsgpackDto']>
  account?: string
}

function toMsgpackDto(value: ApplicationStateOperation): ApplicationStateOperationMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['operation']
    if (v === undefined) {
    } else {
      out['operation'] = v
    }
  }
  {
    const v = (value as any)['appStateType']
    if (v === undefined) {
    } else {
      out['app-state-type'] = v
    }
  }
  {
    const v = (value as any)['key']
    if (v === undefined) {
    } else {
      out['key'] = v
    }
  }
  {
    const v = (value as any)['newValue']
    if (v === undefined) {
    } else {
      out['new-value'] = AvmValueModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['account']
    if (v === undefined) {
    } else {
      out['account'] = v
    }
  }
  return out as ApplicationStateOperationMsgpackDto
}

function fromMsgpackDto(dto: ApplicationStateOperationMsgpackDto): ApplicationStateOperation {
  const out: any = {}
  {
    const v = (dto as any)['operation']
    if (v === undefined) {
    } else {
      out['operation'] = v
    }
  }
  {
    const v = (dto as any)['app-state-type']
    if (v === undefined) {
    } else {
      out['appStateType'] = v
    }
  }
  {
    const v = (dto as any)['key']
    if (v === undefined) {
    } else {
      out['key'] = v
    }
  }
  {
    const v = (dto as any)['new-value']
    if (v === undefined) {
    } else {
      out['newValue'] = AvmValueModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['account']
    if (v === undefined) {
    } else {
      out['account'] = v
    }
  }
  return out as ApplicationStateOperation
}

export const ApplicationStateOperation = {
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
