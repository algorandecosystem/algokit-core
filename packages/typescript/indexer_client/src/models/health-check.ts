import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * A health check response.
 */
export type HealthCheck = {
  /**
   * Current version.
   */
  version: string
  data?: {}
  round: bigint
  isMigrating: boolean
  dbAvailable: boolean
  message: string
  errors?: string[]
}

// JSON DTO shape for HealthCheck with wire keys and JSON-safe primitives
export type HealthCheckDto = {
  version: string
  data?: {}
  round: string
  'is-migrating': boolean
  'db-available': boolean
  message: string
  errors?: string[][]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: HealthCheck): HealthCheckDto {
  const out: any = {}
  {
    const v = (value as any)['version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['version'] = v
    }
  }
  {
    const v = (value as any)['data']
    if (v === undefined) {
      // omit undefined
    } else {
      out['data'] = v
    }
  }
  {
    const v = (value as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['isMigrating']
    if (v === undefined) {
      // omit undefined
    } else {
      out['is-migrating'] = v
    }
  }
  {
    const v = (value as any)['dbAvailable']
    if (v === undefined) {
      // omit undefined
    } else {
      out['db-available'] = v
    }
  }
  {
    const v = (value as any)['message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['message'] = v
    }
  }
  {
    const v = (value as any)['errors']
    if (v === undefined) {
      // omit undefined
    } else {
      out['errors'] = v as any[]
    }
  }
  return out as HealthCheckDto
}

export function fromDto(dto: HealthCheckDto): HealthCheck {
  const out: any = {}
  {
    const v = (dto as any)['version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['version'] = v as any
    }
  }
  {
    const v = (dto as any)['data']
    if (v === undefined) {
      // omit undefined
    } else {
      out['data'] = v as any
    }
  }
  {
    const v = (dto as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['is-migrating']
    if (v === undefined) {
      // omit undefined
    } else {
      out['isMigrating'] = v as any
    }
  }
  {
    const v = (dto as any)['db-available']
    if (v === undefined) {
      // omit undefined
    } else {
      out['dbAvailable'] = v as any
    }
  }
  {
    const v = (dto as any)['message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['message'] = v as any
    }
  }
  {
    const v = (dto as any)['errors']
    if (v === undefined) {
      // omit undefined
    } else {
      out['errors'] = v as any[]
    }
  }
  return out as HealthCheck
}

// Msgpack codecs
export function encodeMsgpack(value: HealthCheck): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): HealthCheck {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: HealthCheck): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): HealthCheck {
  return fromDto(raw as HealthCheckDto)
}

// Array helpers
export function encodeMsgpackArray(values: HealthCheck[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): HealthCheck[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: HealthCheck[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): HealthCheck[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type HealthCheckMsgpackDto = {
  version: string
  data?: {}
  round: bigint
  'is-migrating': boolean
  'db-available': boolean
  message: string
  errors?: string[][]
}

function toMsgpackDto(value: HealthCheck): HealthCheckMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['version']
    if (v === undefined) {
    } else {
      out['version'] = v
    }
  }
  {
    const v = (value as any)['data']
    if (v === undefined) {
    } else {
      out['data'] = v
    }
  }
  {
    const v = (value as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (value as any)['isMigrating']
    if (v === undefined) {
    } else {
      out['is-migrating'] = v
    }
  }
  {
    const v = (value as any)['dbAvailable']
    if (v === undefined) {
    } else {
      out['db-available'] = v
    }
  }
  {
    const v = (value as any)['message']
    if (v === undefined) {
    } else {
      out['message'] = v
    }
  }
  {
    const v = (value as any)['errors']
    if (v === undefined) {
    } else {
      out['errors'] = v as any[]
    }
  }
  return out as HealthCheckMsgpackDto
}

function fromMsgpackDto(dto: HealthCheckMsgpackDto): HealthCheck {
  const out: any = {}
  {
    const v = (dto as any)['version']
    if (v === undefined) {
    } else {
      out['version'] = v
    }
  }
  {
    const v = (dto as any)['data']
    if (v === undefined) {
    } else {
      out['data'] = v
    }
  }
  {
    const v = (dto as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (dto as any)['is-migrating']
    if (v === undefined) {
    } else {
      out['isMigrating'] = v
    }
  }
  {
    const v = (dto as any)['db-available']
    if (v === undefined) {
    } else {
      out['dbAvailable'] = v
    }
  }
  {
    const v = (dto as any)['message']
    if (v === undefined) {
    } else {
      out['message'] = v
    }
  }
  {
    const v = (dto as any)['errors']
    if (v === undefined) {
    } else {
      out['errors'] = v as any[]
    }
  }
  return out as HealthCheck
}

export const HealthCheck = {
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
