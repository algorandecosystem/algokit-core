import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Proof of membership and position of a light block header.
 */
export type LightBlockHeaderProof = {
  /**
   * The index of the light block header in the vector commitment tree
   */
  index: bigint

  /**
   * Represents the depth of the tree that is being proven, i.e. the number of edges from a leaf to the root.
   */
  treedepth: bigint

  /**
   * The encoded proof.
   */
  proof: Uint8Array
}

// JSON DTO shape for LightBlockHeaderProof with wire keys and JSON-safe primitives
export type LightBlockHeaderProofDto = {
  index: string
  treedepth: bigint
  proof: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: LightBlockHeaderProof): LightBlockHeaderProofDto {
  const out: any = {}
  {
    const v = (value as any)['index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['index'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['treedepth']
    if (v === undefined) {
      // omit undefined
    } else {
      out['treedepth'] = v
    }
  }
  {
    const v = (value as any)['proof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proof'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as LightBlockHeaderProofDto
}

export function fromDto(dto: LightBlockHeaderProofDto): LightBlockHeaderProof {
  const out: any = {}
  {
    const v = (dto as any)['index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['index'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['treedepth']
    if (v === undefined) {
      // omit undefined
    } else {
      out['treedepth'] = v as any
    }
  }
  {
    const v = (dto as any)['proof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proof'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as LightBlockHeaderProof
}

// Msgpack codecs
export function encodeMsgpack(value: LightBlockHeaderProof): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): LightBlockHeaderProof {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: LightBlockHeaderProof): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): LightBlockHeaderProof {
  return fromDto(raw as LightBlockHeaderProofDto)
}

// Array helpers
export function encodeMsgpackArray(values: LightBlockHeaderProof[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): LightBlockHeaderProof[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: LightBlockHeaderProof[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): LightBlockHeaderProof[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type LightBlockHeaderProofMsgpackDto = {
  index: bigint
  treedepth: bigint
  proof: Uint8Array
}

function toMsgpackDto(value: LightBlockHeaderProof): LightBlockHeaderProofMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['index']
    if (v === undefined) {
    } else {
      out['index'] = v
    }
  }
  {
    const v = (value as any)['treedepth']
    if (v === undefined) {
    } else {
      out['treedepth'] = v
    }
  }
  {
    const v = (value as any)['proof']
    if (v === undefined) {
    } else {
      out['proof'] = v
    }
  }
  return out as LightBlockHeaderProofMsgpackDto
}

function fromMsgpackDto(dto: LightBlockHeaderProofMsgpackDto): LightBlockHeaderProof {
  const out: any = {}
  {
    const v = (dto as any)['index']
    if (v === undefined) {
    } else {
      out['index'] = v
    }
  }
  {
    const v = (dto as any)['treedepth']
    if (v === undefined) {
    } else {
      out['treedepth'] = v
    }
  }
  {
    const v = (dto as any)['proof']
    if (v === undefined) {
    } else {
      out['proof'] = v
    }
  }
  return out as LightBlockHeaderProof
}

export const LightBlockHeaderProof = {
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
