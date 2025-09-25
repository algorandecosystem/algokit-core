import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { ApplicationStateSchema, ApplicationStateSchemaDto } from './application-state-schema'
import { ApplicationStateSchema as ApplicationStateSchemaModel } from './application-state-schema'
import type { TealKeyValueStore, TealKeyValueStoreDto } from './teal-key-value-store'
import { TealKeyValueStore as TealKeyValueStoreModel } from './teal-key-value-store'

/**
 * Stores local state associated with an application.
 */
export type ApplicationLocalState = {
  /**
   * The application which this local state is for.
   */
  id: bigint

  /**
   * Whether or not the application local state is currently deleted from its account.
   */
  deleted?: boolean

  /**
   * Round when the account opted into the application.
   */
  optedInAtRound?: bigint

  /**
   * Round when account closed out of the application.
   */
  closedOutAtRound?: bigint
  schema: ApplicationStateSchema
  keyValue?: TealKeyValueStore
}

// JSON DTO shape for ApplicationLocalState with wire keys and JSON-safe primitives
export type ApplicationLocalStateDto = {
  id: string
  deleted?: boolean
  'opted-in-at-round'?: string
  'closed-out-at-round'?: string
  schema: ApplicationStateSchemaDto
  'key-value'?: TealKeyValueStoreDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ApplicationLocalState): ApplicationLocalStateDto {
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
    const v = (value as any)['deleted']
    if (v === undefined) {
      // omit undefined
    } else {
      out['deleted'] = v
    }
  }
  {
    const v = (value as any)['optedInAtRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['opted-in-at-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['closedOutAtRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closed-out-at-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['schema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['schema'] = v === undefined ? v : ApplicationStateSchemaModel.toDto(v)
    }
  }
  {
    const v = (value as any)['keyValue']
    if (v === undefined) {
      // omit undefined
    } else {
      out['key-value'] = v === undefined ? v : TealKeyValueStoreModel.toDto(v)
    }
  }
  return out as ApplicationLocalStateDto
}

export function fromDto(dto: ApplicationLocalStateDto): ApplicationLocalState {
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
    const v = (dto as any)['deleted']
    if (v === undefined) {
      // omit undefined
    } else {
      out['deleted'] = v as any
    }
  }
  {
    const v = (dto as any)['opted-in-at-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['optedInAtRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['closed-out-at-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closedOutAtRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['schema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['schema'] = v === undefined ? v : ApplicationStateSchemaModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['key-value']
    if (v === undefined) {
      // omit undefined
    } else {
      out['keyValue'] = v === undefined ? v : TealKeyValueStoreModel.fromDto(v)
    }
  }
  return out as ApplicationLocalState
}

// Msgpack codecs
export function encodeMsgpack(value: ApplicationLocalState): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ApplicationLocalState {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ApplicationLocalState): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ApplicationLocalState {
  return fromDto(raw as ApplicationLocalStateDto)
}

// Array helpers
export function encodeMsgpackArray(values: ApplicationLocalState[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ApplicationLocalState[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ApplicationLocalState[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ApplicationLocalState[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ApplicationLocalStateMsgpackDto = {
  id: bigint
  deleted?: boolean
  'opted-in-at-round'?: bigint
  'closed-out-at-round'?: bigint
  schema: ReturnType<(typeof ApplicationStateSchemaModel)['toMsgpackDto']>
  'key-value'?: ReturnType<(typeof TealKeyValueStoreModel)['toMsgpackDto']>
}

function toMsgpackDto(value: ApplicationLocalState): ApplicationLocalStateMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['id']
    if (v === undefined) {
    } else {
      out['id'] = v
    }
  }
  {
    const v = (value as any)['deleted']
    if (v === undefined) {
    } else {
      out['deleted'] = v
    }
  }
  {
    const v = (value as any)['optedInAtRound']
    if (v === undefined) {
    } else {
      out['opted-in-at-round'] = v
    }
  }
  {
    const v = (value as any)['closedOutAtRound']
    if (v === undefined) {
    } else {
      out['closed-out-at-round'] = v
    }
  }
  {
    const v = (value as any)['schema']
    if (v === undefined) {
    } else {
      out['schema'] = ApplicationStateSchemaModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['keyValue']
    if (v === undefined) {
    } else {
      out['key-value'] = TealKeyValueStoreModel.toMsgpackDto(v)
    }
  }
  return out as ApplicationLocalStateMsgpackDto
}

function fromMsgpackDto(dto: ApplicationLocalStateMsgpackDto): ApplicationLocalState {
  const out: any = {}
  {
    const v = (dto as any)['id']
    if (v === undefined) {
    } else {
      out['id'] = v
    }
  }
  {
    const v = (dto as any)['deleted']
    if (v === undefined) {
    } else {
      out['deleted'] = v
    }
  }
  {
    const v = (dto as any)['opted-in-at-round']
    if (v === undefined) {
    } else {
      out['optedInAtRound'] = v
    }
  }
  {
    const v = (dto as any)['closed-out-at-round']
    if (v === undefined) {
    } else {
      out['closedOutAtRound'] = v
    }
  }
  {
    const v = (dto as any)['schema']
    if (v === undefined) {
    } else {
      out['schema'] = ApplicationStateSchemaModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['key-value']
    if (v === undefined) {
    } else {
      out['keyValue'] = TealKeyValueStoreModel.fromMsgpackDto(v)
    }
  }
  return out as ApplicationLocalState
}

export const ApplicationLocalState = {
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
