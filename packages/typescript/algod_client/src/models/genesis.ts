import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { GenesisAllocation, GenesisAllocationDto } from './genesis-allocation'
import { GenesisAllocation as GenesisAllocationModel } from './genesis-allocation'

export type Genesis = {
  alloc: GenesisAllocation[]
  comment?: string
  devmode?: boolean
  fees: string
  id: string
  network: string
  proto: string
  rwd: string
  timestamp: bigint
}

// JSON DTO shape for Genesis with wire keys and JSON-safe primitives
export type GenesisDto = {
  alloc: GenesisAllocationDto[]
  comment?: string
  devmode?: boolean
  fees: string
  id: string
  network: string
  proto: string
  rwd: string
  timestamp: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: Genesis): GenesisDto {
  const out: any = {}
  {
    const v = (value as any)['alloc']
    if (v === undefined) {
      // omit undefined
    } else {
      out['alloc'] = (v as any[]).map((item) => GenesisAllocationModel.toDto(item))
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
    const v = (value as any)['devmode']
    if (v === undefined) {
      // omit undefined
    } else {
      out['devmode'] = v
    }
  }
  {
    const v = (value as any)['fees']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fees'] = v
    }
  }
  {
    const v = (value as any)['id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['id'] = v
    }
  }
  {
    const v = (value as any)['network']
    if (v === undefined) {
      // omit undefined
    } else {
      out['network'] = v
    }
  }
  {
    const v = (value as any)['proto']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proto'] = v
    }
  }
  {
    const v = (value as any)['rwd']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rwd'] = v
    }
  }
  {
    const v = (value as any)['timestamp']
    if (v === undefined) {
      // omit undefined
    } else {
      out['timestamp'] = v
    }
  }
  return out as GenesisDto
}

export function fromDto(dto: GenesisDto): Genesis {
  const out: any = {}
  {
    const v = (dto as any)['alloc']
    if (v === undefined) {
      // omit undefined
    } else {
      out['alloc'] = (v as any[]).map((item) => GenesisAllocationModel.fromDto(item))
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
    const v = (dto as any)['devmode']
    if (v === undefined) {
      // omit undefined
    } else {
      out['devmode'] = v as any
    }
  }
  {
    const v = (dto as any)['fees']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fees'] = v as any
    }
  }
  {
    const v = (dto as any)['id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['id'] = v as any
    }
  }
  {
    const v = (dto as any)['network']
    if (v === undefined) {
      // omit undefined
    } else {
      out['network'] = v as any
    }
  }
  {
    const v = (dto as any)['proto']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proto'] = v as any
    }
  }
  {
    const v = (dto as any)['rwd']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rwd'] = v as any
    }
  }
  {
    const v = (dto as any)['timestamp']
    if (v === undefined) {
      // omit undefined
    } else {
      out['timestamp'] = v as any
    }
  }
  return out as Genesis
}

// Msgpack codecs
export function encodeMsgpack(value: Genesis): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): Genesis {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: Genesis): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): Genesis {
  return fromDto(raw as GenesisDto)
}

// Array helpers
export function encodeMsgpackArray(values: Genesis[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): Genesis[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: Genesis[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): Genesis[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GenesisMsgpackDto = {
  alloc: ReturnType<(typeof GenesisAllocationModel)['toMsgpackDto']>[]
  comment?: string
  devmode?: boolean
  fees: string
  id: string
  network: string
  proto: string
  rwd: string
  timestamp: bigint
}

function toMsgpackDto(value: Genesis): GenesisMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['alloc']
    if (v === undefined) {
    } else {
      out['alloc'] = (v as any[]).map((item) => GenesisAllocationModel.toMsgpackDto(item))
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
    const v = (value as any)['devmode']
    if (v === undefined) {
    } else {
      out['devmode'] = v
    }
  }
  {
    const v = (value as any)['fees']
    if (v === undefined) {
    } else {
      out['fees'] = v
    }
  }
  {
    const v = (value as any)['id']
    if (v === undefined) {
    } else {
      out['id'] = v
    }
  }
  {
    const v = (value as any)['network']
    if (v === undefined) {
    } else {
      out['network'] = v
    }
  }
  {
    const v = (value as any)['proto']
    if (v === undefined) {
    } else {
      out['proto'] = v
    }
  }
  {
    const v = (value as any)['rwd']
    if (v === undefined) {
    } else {
      out['rwd'] = v
    }
  }
  {
    const v = (value as any)['timestamp']
    if (v === undefined) {
    } else {
      out['timestamp'] = v
    }
  }
  return out as GenesisMsgpackDto
}

function fromMsgpackDto(dto: GenesisMsgpackDto): Genesis {
  const out: any = {}
  {
    const v = (dto as any)['alloc']
    if (v === undefined) {
    } else {
      out['alloc'] = (v as any[]).map((item) => GenesisAllocationModel.fromMsgpackDto(item))
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
    const v = (dto as any)['devmode']
    if (v === undefined) {
    } else {
      out['devmode'] = v
    }
  }
  {
    const v = (dto as any)['fees']
    if (v === undefined) {
    } else {
      out['fees'] = v
    }
  }
  {
    const v = (dto as any)['id']
    if (v === undefined) {
    } else {
      out['id'] = v
    }
  }
  {
    const v = (dto as any)['network']
    if (v === undefined) {
    } else {
      out['network'] = v
    }
  }
  {
    const v = (dto as any)['proto']
    if (v === undefined) {
    } else {
      out['proto'] = v
    }
  }
  {
    const v = (dto as any)['rwd']
    if (v === undefined) {
    } else {
      out['rwd'] = v
    }
  }
  {
    const v = (dto as any)['timestamp']
    if (v === undefined) {
    } else {
      out['timestamp'] = v
    }
  }
  return out as Genesis
}

export const Genesis = {
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
