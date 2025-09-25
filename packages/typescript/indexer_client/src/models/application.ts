import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { ApplicationParams, ApplicationParamsDto } from './application-params'
import { ApplicationParams as ApplicationParamsModel } from './application-params'

/**
 * Application index and its parameters
 */
export type Application = {
  /**
   * application index.
   */
  id: bigint

  /**
   * Whether or not this application is currently deleted.
   */
  deleted?: boolean

  /**
   * Round when this application was created.
   */
  createdAtRound?: bigint

  /**
   * Round when this application was deleted.
   */
  deletedAtRound?: bigint
  params: ApplicationParams
}

// JSON DTO shape for Application with wire keys and JSON-safe primitives
export type ApplicationDto = {
  id: string
  deleted?: boolean
  'created-at-round'?: string
  'deleted-at-round'?: string
  params: ApplicationParamsDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: Application): ApplicationDto {
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
    const v = (value as any)['createdAtRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['created-at-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['deletedAtRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['deleted-at-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['params']
    if (v === undefined) {
      // omit undefined
    } else {
      out['params'] = v === undefined ? v : ApplicationParamsModel.toDto(v)
    }
  }
  return out as ApplicationDto
}

export function fromDto(dto: ApplicationDto): Application {
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
    const v = (dto as any)['created-at-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['createdAtRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['deleted-at-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['deletedAtRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['params']
    if (v === undefined) {
      // omit undefined
    } else {
      out['params'] = v === undefined ? v : ApplicationParamsModel.fromDto(v)
    }
  }
  return out as Application
}

// Msgpack codecs
export function encodeMsgpack(value: Application): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): Application {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: Application): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): Application {
  return fromDto(raw as ApplicationDto)
}

// Array helpers
export function encodeMsgpackArray(values: Application[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): Application[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: Application[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): Application[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ApplicationMsgpackDto = {
  id: bigint
  deleted?: boolean
  'created-at-round'?: bigint
  'deleted-at-round'?: bigint
  params: ReturnType<(typeof ApplicationParamsModel)['toMsgpackDto']>
}

function toMsgpackDto(value: Application): ApplicationMsgpackDto {
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
    const v = (value as any)['createdAtRound']
    if (v === undefined) {
    } else {
      out['created-at-round'] = v
    }
  }
  {
    const v = (value as any)['deletedAtRound']
    if (v === undefined) {
    } else {
      out['deleted-at-round'] = v
    }
  }
  {
    const v = (value as any)['params']
    if (v === undefined) {
    } else {
      out['params'] = ApplicationParamsModel.toMsgpackDto(v)
    }
  }
  return out as ApplicationMsgpackDto
}

function fromMsgpackDto(dto: ApplicationMsgpackDto): Application {
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
    const v = (dto as any)['created-at-round']
    if (v === undefined) {
    } else {
      out['createdAtRound'] = v
    }
  }
  {
    const v = (dto as any)['deleted-at-round']
    if (v === undefined) {
    } else {
      out['deletedAtRound'] = v
    }
  }
  {
    const v = (dto as any)['params']
    if (v === undefined) {
    } else {
      out['params'] = ApplicationParamsModel.fromMsgpackDto(v)
    }
  }
  return out as Application
}

export const Application = {
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
