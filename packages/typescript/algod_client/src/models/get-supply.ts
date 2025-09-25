import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Supply represents the current supply of MicroAlgos in the system
 */
export type GetSupply = {
  /**
   * Round
   */
  currentRound: bigint

  /**
   * OnlineMoney
   */
  onlineMoney: bigint

  /**
   * TotalMoney
   */
  totalMoney: bigint
}

// JSON DTO shape for GetSupply with wire keys and JSON-safe primitives
export type GetSupplyDto = {
  current_round: string
  'online-money': string
  'total-money': string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: GetSupply): GetSupplyDto {
  const out: any = {}
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['current_round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['onlineMoney']
    if (v === undefined) {
      // omit undefined
    } else {
      out['online-money'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['totalMoney']
    if (v === undefined) {
      // omit undefined
    } else {
      out['total-money'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as GetSupplyDto
}

export function fromDto(dto: GetSupplyDto): GetSupply {
  const out: any = {}
  {
    const v = (dto as any)['current_round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['currentRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['online-money']
    if (v === undefined) {
      // omit undefined
    } else {
      out['onlineMoney'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['total-money']
    if (v === undefined) {
      // omit undefined
    } else {
      out['totalMoney'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as GetSupply
}

// Msgpack codecs
export function encodeMsgpack(value: GetSupply): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): GetSupply {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: GetSupply): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): GetSupply {
  return fromDto(raw as GetSupplyDto)
}

// Array helpers
export function encodeMsgpackArray(values: GetSupply[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): GetSupply[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: GetSupply[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): GetSupply[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type GetSupplyMsgpackDto = {
  current_round: bigint
  'online-money': bigint
  'total-money': bigint
}

function toMsgpackDto(value: GetSupply): GetSupplyMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['currentRound']
    if (v === undefined) {
    } else {
      out['current_round'] = v
    }
  }
  {
    const v = (value as any)['onlineMoney']
    if (v === undefined) {
    } else {
      out['online-money'] = v
    }
  }
  {
    const v = (value as any)['totalMoney']
    if (v === undefined) {
    } else {
      out['total-money'] = v
    }
  }
  return out as GetSupplyMsgpackDto
}

function fromMsgpackDto(dto: GetSupplyMsgpackDto): GetSupply {
  const out: any = {}
  {
    const v = (dto as any)['current_round']
    if (v === undefined) {
    } else {
      out['currentRound'] = v
    }
  }
  {
    const v = (dto as any)['online-money']
    if (v === undefined) {
    } else {
      out['onlineMoney'] = v
    }
  }
  {
    const v = (dto as any)['total-money']
    if (v === undefined) {
    } else {
      out['totalMoney'] = v
    }
  }
  return out as GetSupply
}

export const GetSupply = {
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
