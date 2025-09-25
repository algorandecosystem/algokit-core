import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type GenesisAllocation = {
  addr: string
  comment: string
  state: {
    algo: bigint
    onl: bigint
    sel?: string
    stprf?: string
    vote?: string
    voteKd?: bigint
    voteFst?: bigint
    voteLst?: bigint
  }
}

// JSON DTO shape for GenesisAllocation with wire keys and JSON-safe primitives
export type GenesisAllocationDto = {
  addr: string
  comment: string
  state: {
    algo: bigint
    onl: bigint
    sel?: string
    stprf?: string
    vote?: string
    voteKd?: bigint
    voteFst?: bigint
    voteLst?: bigint
  }
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GenesisAllocation): GenesisAllocationDto {
  const out: any = {}
  {
    const v = (value as any)['addr']
    if (v === undefined) {
      // omit undefined
    } else {
      out['addr'] = v
    }
  }
  {
    const v = (value as any)['comment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['comment'] = v
    }
  }
  {
    const v = (value as any)['state']
    if (v === undefined) {
      // omit undefined
    } else {
      out['state'] = v
    }
  }
  return out as GenesisAllocationDto
}

export function fromDto(dto: GenesisAllocationDto): GenesisAllocation {
  const out: any = {}
  {
    const v = (dto as any)['addr']
    if (v === undefined) {
      // omit undefined
    } else {
      out['addr'] = v as any
    }
  }
  {
    const v = (dto as any)['comment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['comment'] = v as any
    }
  }
  {
    const v = (dto as any)['state']
    if (v === undefined) {
      // omit undefined
    } else {
      out['state'] = v as any
    }
  }
  return out as GenesisAllocation
}

// Msgpack codecs
export function encodeMsgpack(value: GenesisAllocation): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GenesisAllocation {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GenesisAllocation): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GenesisAllocation {
  return fromDto(raw as GenesisAllocationDto)
}

// Array helpers
export function encodeMsgpackArray(values: GenesisAllocation[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GenesisAllocation[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GenesisAllocation[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GenesisAllocation[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GenesisAllocationMsgpackDto = {
  addr: string
  comment: string
  state: {
    algo: bigint
    onl: bigint
    sel?: string
    stprf?: string
    vote?: string
    voteKd?: bigint
    voteFst?: bigint
    voteLst?: bigint
  }
}

function toMsgpackDto(value: GenesisAllocation): GenesisAllocationMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['addr']
    if (v === undefined) {
    } else {
      out['addr'] = v
    }
  }
  {
    const v = (value as any)['comment']
    if (v === undefined) {
    } else {
      out['comment'] = v
    }
  }
  {
    const v = (value as any)['state']
    if (v === undefined) {
    } else {
      out['state'] = v
    }
  }
  return out as GenesisAllocationMsgpackDto
}

function fromMsgpackDto(dto: GenesisAllocationMsgpackDto): GenesisAllocation {
  const out: any = {}
  {
    const v = (dto as any)['addr']
    if (v === undefined) {
    } else {
      out['addr'] = v
    }
  }
  {
    const v = (dto as any)['comment']
    if (v === undefined) {
    } else {
      out['comment'] = v
    }
  }
  {
    const v = (dto as any)['state']
    if (v === undefined) {
    } else {
      out['state'] = v
    }
  }
  return out as GenesisAllocation
}

export const GenesisAllocation = {
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
