import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { MerkleArrayProof, MerkleArrayProofDto } from './merkle-array-proof'
import { MerkleArrayProof as MerkleArrayProofModel } from './merkle-array-proof'
import type { StateProofReveal, StateProofRevealDto } from './state-proof-reveal'
import { StateProofReveal as StateProofRevealModel } from './state-proof-reveal'

/**
 * \[sp\] represents a state proof.
 *
 * Definition:
 * crypto/stateproof/structs.go : StateProof
 */
export type StateProofFields = {
  /**
   * \[c\]
   */
  sigCommit?: Uint8Array

  /**
   * \[w\]
   */
  signedWeight?: bigint
  sigProofs?: MerkleArrayProof
  partProofs?: MerkleArrayProof

  /**
   * \[v\] Salt version of the merkle signature.
   */
  saltVersion?: bigint

  /**
   * \[r\] Note that this is actually stored as a map[uint64] - Reveal in the actual msgp
   */
  reveals?: StateProofReveal[]

  /**
   * \[pr\] Sequence of reveal positions.
   */
  positionsToReveal?: bigint[]
}

// JSON DTO shape for StateProofFields with wire keys and JSON-safe primitives
export type StateProofFieldsDto = {
  'sig-commit'?: string
  'signed-weight'?: string
  'sig-proofs'?: MerkleArrayProofDto
  'part-proofs'?: MerkleArrayProofDto
  'salt-version'?: bigint
  reveals?: StateProofRevealDto[]
  'positions-to-reveal'?: string[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateProofFields): StateProofFieldsDto {
  const out: any = {}
  {
    const v = (value as any)['sigCommit']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sig-commit'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['signedWeight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['signed-weight'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['sigProofs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sig-proofs'] = v === undefined ? v : MerkleArrayProofModel.toDto(v)
    }
  }
  {
    const v = (value as any)['partProofs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['part-proofs'] = v === undefined ? v : MerkleArrayProofModel.toDto(v)
    }
  }
  {
    const v = (value as any)['saltVersion']
    if (v === undefined) {
      // omit undefined
    } else {
      out['salt-version'] = v
    }
  }
  {
    const v = (value as any)['reveals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['reveals'] = (v as any[]).map((item) => StateProofRevealModel.toDto(item))
    }
  }
  {
    const v = (value as any)['positionsToReveal']
    if (v === undefined) {
      // omit undefined
    } else {
      out['positions-to-reveal'] = (v as Array<number | bigint | string>).map((x) => (typeof x === 'bigint' ? x.toString() : String(x)))
    }
  }
  return out as StateProofFieldsDto
}

export function fromDto(dto: StateProofFieldsDto): StateProofFields {
  const out: any = {}
  {
    const v = (dto as any)['sig-commit']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sigCommit'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['signed-weight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['signedWeight'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['sig-proofs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sigProofs'] = v === undefined ? v : MerkleArrayProofModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['part-proofs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['partProofs'] = v === undefined ? v : MerkleArrayProofModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['salt-version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['saltVersion'] = v as any
    }
  }
  {
    const v = (dto as any)['reveals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['reveals'] = (v as any[]).map((item) => StateProofRevealModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['positions-to-reveal']
    if (v === undefined) {
      // omit undefined
    } else {
      out['positionsToReveal'] = (v as Array<string | number | bigint>).map((x) => (typeof x === 'bigint' ? x : BigInt(x as any)))
    }
  }
  return out as StateProofFields
}

// Msgpack codecs
export function encodeMsgpack(value: StateProofFields): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StateProofFields {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StateProofFields): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StateProofFields {
  return fromDto(raw as StateProofFieldsDto)
}

// Array helpers
export function encodeMsgpackArray(values: StateProofFields[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StateProofFields[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StateProofFields[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StateProofFields[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StateProofFieldsMsgpackDto = {
  'sig-commit'?: Uint8Array
  'signed-weight'?: bigint
  'sig-proofs'?: ReturnType<(typeof MerkleArrayProofModel)['toMsgpackDto']>
  'part-proofs'?: ReturnType<(typeof MerkleArrayProofModel)['toMsgpackDto']>
  'salt-version'?: bigint
  reveals?: ReturnType<(typeof StateProofRevealModel)['toMsgpackDto']>[]
  'positions-to-reveal'?: bigint[]
}

function toMsgpackDto(value: StateProofFields): StateProofFieldsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['sigCommit']
    if (v === undefined) {
    } else {
      out['sig-commit'] = v
    }
  }
  {
    const v = (value as any)['signedWeight']
    if (v === undefined) {
    } else {
      out['signed-weight'] = v
    }
  }
  {
    const v = (value as any)['sigProofs']
    if (v === undefined) {
    } else {
      out['sig-proofs'] = MerkleArrayProofModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['partProofs']
    if (v === undefined) {
    } else {
      out['part-proofs'] = MerkleArrayProofModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['saltVersion']
    if (v === undefined) {
    } else {
      out['salt-version'] = v
    }
  }
  {
    const v = (value as any)['reveals']
    if (v === undefined) {
    } else {
      out['reveals'] = (v as any[]).map((item) => StateProofRevealModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['positionsToReveal']
    if (v === undefined) {
    } else {
      out['positions-to-reveal'] = v as any[]
    }
  }
  return out as StateProofFieldsMsgpackDto
}

function fromMsgpackDto(dto: StateProofFieldsMsgpackDto): StateProofFields {
  const out: any = {}
  {
    const v = (dto as any)['sig-commit']
    if (v === undefined) {
    } else {
      out['sigCommit'] = v
    }
  }
  {
    const v = (dto as any)['signed-weight']
    if (v === undefined) {
    } else {
      out['signedWeight'] = v
    }
  }
  {
    const v = (dto as any)['sig-proofs']
    if (v === undefined) {
    } else {
      out['sigProofs'] = MerkleArrayProofModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['part-proofs']
    if (v === undefined) {
    } else {
      out['partProofs'] = MerkleArrayProofModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['salt-version']
    if (v === undefined) {
    } else {
      out['saltVersion'] = v
    }
  }
  {
    const v = (dto as any)['reveals']
    if (v === undefined) {
    } else {
      out['reveals'] = (v as any[]).map((item) => StateProofRevealModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['positions-to-reveal']
    if (v === undefined) {
    } else {
      out['positionsToReveal'] = v as any[]
    }
  }
  return out as StateProofFields
}

export const StateProofFields = {
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
