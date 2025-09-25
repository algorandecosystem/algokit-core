import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AvmKeyValue, AvmKeyValueDto } from './avm-key-value'
import { AvmKeyValue as AvmKeyValueModel } from './avm-key-value'

/**
 * An application's global/local/box state.
 */
export type ApplicationKvstorage = {
  /**
   * Key-Value pairs representing application states.
   */
  kvs: AvmKeyValue[]

  /**
   * The address of the account associated with the local state.
   */
  account?: string
}

// JSON DTO shape for ApplicationKvstorage with wire keys and JSON-safe primitives
export type ApplicationKvstorageDto = {
  kvs: AvmKeyValueDto[]
  account?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ApplicationKvstorage): ApplicationKvstorageDto {
  const out: any = {}
  {
    const v = (value as any)['kvs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['kvs'] = (v as any[]).map((item) => AvmKeyValueModel.toDto(item))
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
  return out as ApplicationKvstorageDto
}

export function fromDto(dto: ApplicationKvstorageDto): ApplicationKvstorage {
  const out: any = {}
  {
    const v = (dto as any)['kvs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['kvs'] = (v as any[]).map((item) => AvmKeyValueModel.fromDto(item))
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
  return out as ApplicationKvstorage
}

// Msgpack codecs
export function encodeMsgpack(value: ApplicationKvstorage): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ApplicationKvstorage {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ApplicationKvstorage): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ApplicationKvstorage {
  return fromDto(raw as ApplicationKvstorageDto)
}

// Array helpers
export function encodeMsgpackArray(values: ApplicationKvstorage[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ApplicationKvstorage[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ApplicationKvstorage[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ApplicationKvstorage[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ApplicationKvstorageMsgpackDto = {
  kvs: ReturnType<(typeof AvmKeyValueModel)['toMsgpackDto']>[]
  account?: string
}

function toMsgpackDto(value: ApplicationKvstorage): ApplicationKvstorageMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['kvs']
    if (v === undefined) {
    } else {
      out['kvs'] = (v as any[]).map((item) => AvmKeyValueModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['account']
    if (v === undefined) {
    } else {
      out['account'] = v
    }
  }
  return out as ApplicationKvstorageMsgpackDto
}

function fromMsgpackDto(dto: ApplicationKvstorageMsgpackDto): ApplicationKvstorage {
  const out: any = {}
  {
    const v = (dto as any)['kvs']
    if (v === undefined) {
    } else {
      out['kvs'] = (v as any[]).map((item) => AvmKeyValueModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['account']
    if (v === undefined) {
    } else {
      out['account'] = v
    }
  }
  return out as ApplicationKvstorage
}

export const ApplicationKvstorage = {
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
