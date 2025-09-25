import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type BuildVersion = {
  branch: string
  buildNumber: bigint
  channel: string
  commitHash: string
  major: bigint
  minor: bigint
}

// JSON DTO shape for BuildVersion with wire keys and JSON-safe primitives
export type BuildVersionDto = {
  branch: string
  build_number: bigint
  channel: string
  commit_hash: string
  major: bigint
  minor: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: BuildVersion): BuildVersionDto {
  const out: any = {}
  {
    const v = (value as any)['branch']
    if (v === undefined) {
      // omit undefined
    } else {
      out['branch'] = v
    }
  }
  {
    const v = (value as any)['buildNumber']
    if (v === undefined) {
      // omit undefined
    } else {
      out['build_number'] = v
    }
  }
  {
    const v = (value as any)['channel']
    if (v === undefined) {
      // omit undefined
    } else {
      out['channel'] = v
    }
  }
  {
    const v = (value as any)['commitHash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['commit_hash'] = v
    }
  }
  {
    const v = (value as any)['major']
    if (v === undefined) {
      // omit undefined
    } else {
      out['major'] = v
    }
  }
  {
    const v = (value as any)['minor']
    if (v === undefined) {
      // omit undefined
    } else {
      out['minor'] = v
    }
  }
  return out as BuildVersionDto
}

export function fromDto(dto: BuildVersionDto): BuildVersion {
  const out: any = {}
  {
    const v = (dto as any)['branch']
    if (v === undefined) {
      // omit undefined
    } else {
      out['branch'] = v as any
    }
  }
  {
    const v = (dto as any)['build_number']
    if (v === undefined) {
      // omit undefined
    } else {
      out['buildNumber'] = v as any
    }
  }
  {
    const v = (dto as any)['channel']
    if (v === undefined) {
      // omit undefined
    } else {
      out['channel'] = v as any
    }
  }
  {
    const v = (dto as any)['commit_hash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['commitHash'] = v as any
    }
  }
  {
    const v = (dto as any)['major']
    if (v === undefined) {
      // omit undefined
    } else {
      out['major'] = v as any
    }
  }
  {
    const v = (dto as any)['minor']
    if (v === undefined) {
      // omit undefined
    } else {
      out['minor'] = v as any
    }
  }
  return out as BuildVersion
}

// Msgpack codecs
export function encodeMsgpack(value: BuildVersion): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): BuildVersion {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: BuildVersion): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): BuildVersion {
  return fromDto(raw as BuildVersionDto)
}

// Array helpers
export function encodeMsgpackArray(values: BuildVersion[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): BuildVersion[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: BuildVersion[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): BuildVersion[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type BuildVersionMsgpackDto = {
  branch: string
  build_number: bigint
  channel: string
  commit_hash: string
  major: bigint
  minor: bigint
}

function toMsgpackDto(value: BuildVersion): BuildVersionMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['branch']
    if (v === undefined) {
    } else {
      out['branch'] = v
    }
  }
  {
    const v = (value as any)['buildNumber']
    if (v === undefined) {
    } else {
      out['build_number'] = v
    }
  }
  {
    const v = (value as any)['channel']
    if (v === undefined) {
    } else {
      out['channel'] = v
    }
  }
  {
    const v = (value as any)['commitHash']
    if (v === undefined) {
    } else {
      out['commit_hash'] = v
    }
  }
  {
    const v = (value as any)['major']
    if (v === undefined) {
    } else {
      out['major'] = v
    }
  }
  {
    const v = (value as any)['minor']
    if (v === undefined) {
    } else {
      out['minor'] = v
    }
  }
  return out as BuildVersionMsgpackDto
}

function fromMsgpackDto(dto: BuildVersionMsgpackDto): BuildVersion {
  const out: any = {}
  {
    const v = (dto as any)['branch']
    if (v === undefined) {
    } else {
      out['branch'] = v
    }
  }
  {
    const v = (dto as any)['build_number']
    if (v === undefined) {
    } else {
      out['buildNumber'] = v
    }
  }
  {
    const v = (dto as any)['channel']
    if (v === undefined) {
    } else {
      out['channel'] = v
    }
  }
  {
    const v = (dto as any)['commit_hash']
    if (v === undefined) {
    } else {
      out['commitHash'] = v
    }
  }
  {
    const v = (dto as any)['major']
    if (v === undefined) {
    } else {
      out['major'] = v
    }
  }
  {
    const v = (dto as any)['minor']
    if (v === undefined) {
    } else {
      out['minor'] = v
    }
  }
  return out as BuildVersion
}

export const BuildVersion = {
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
