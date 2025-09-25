import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { BuildVersion, BuildVersionDto } from './build-version'
import { BuildVersion as BuildVersionModel } from './build-version'

/**
 * algod version information.
 */
export type Version = {
  build: BuildVersion
  genesisHashB64: Uint8Array
  genesisId: string
  versions: string[]
}

// JSON DTO shape for Version with wire keys and JSON-safe primitives
export type VersionDto = {
  build: BuildVersionDto
  genesis_hash_b64: string
  genesis_id: string
  versions: string[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: Version): VersionDto {
  const out: any = {}
  {
    const v = (value as any)['build']
    if (v === undefined) {
      // omit undefined
    } else {
      out['build'] = v === undefined ? v : BuildVersionModel.toDto(v)
    }
  }
  {
    const v = (value as any)['genesisHashB64']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesis_hash_b64'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['genesisId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesis_id'] = v
    }
  }
  {
    const v = (value as any)['versions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['versions'] = v as any[]
    }
  }
  return out as VersionDto
}

export function fromDto(dto: VersionDto): Version {
  const out: any = {}
  {
    const v = (dto as any)['build']
    if (v === undefined) {
      // omit undefined
    } else {
      out['build'] = v === undefined ? v : BuildVersionModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['genesis_hash_b64']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesisHashB64'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['genesis_id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesisId'] = v as any
    }
  }
  {
    const v = (dto as any)['versions']
    if (v === undefined) {
      // omit undefined
    } else {
      out['versions'] = v as any[]
    }
  }
  return out as Version
}

// Msgpack codecs
export function encodeMsgpack(value: Version): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): Version {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: Version): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): Version {
  return fromDto(raw as VersionDto)
}

// Array helpers
export function encodeMsgpackArray(values: Version[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): Version[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: Version[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): Version[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type VersionMsgpackDto = {
  build: ReturnType<(typeof BuildVersionModel)['toMsgpackDto']>
  genesis_hash_b64: Uint8Array
  genesis_id: string
  versions: string[]
}

function toMsgpackDto(value: Version): VersionMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['build']
    if (v === undefined) {
    } else {
      out['build'] = BuildVersionModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['genesisHashB64']
    if (v === undefined) {
    } else {
      out['genesis_hash_b64'] = v
    }
  }
  {
    const v = (value as any)['genesisId']
    if (v === undefined) {
    } else {
      out['genesis_id'] = v
    }
  }
  {
    const v = (value as any)['versions']
    if (v === undefined) {
    } else {
      out['versions'] = v as any[]
    }
  }
  return out as VersionMsgpackDto
}

function fromMsgpackDto(dto: VersionMsgpackDto): Version {
  const out: any = {}
  {
    const v = (dto as any)['build']
    if (v === undefined) {
    } else {
      out['build'] = BuildVersionModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['genesis_hash_b64']
    if (v === undefined) {
    } else {
      out['genesisHashB64'] = v
    }
  }
  {
    const v = (dto as any)['genesis_id']
    if (v === undefined) {
    } else {
      out['genesisId'] = v
    }
  }
  {
    const v = (dto as any)['versions']
    if (v === undefined) {
    } else {
      out['versions'] = v as any[]
    }
  }
  return out as Version
}

export const Version = {
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
