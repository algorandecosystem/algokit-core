import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Proof of transaction in a block.
 */
export type TransactionProof = {
  /**
   * Proof of transaction membership.
   */
  proof: Uint8Array

  /**
   * Hash of SignedTxnInBlock for verifying proof.
   */
  stibhash: Uint8Array

  /**
   * Represents the depth of the tree that is being proven, i.e. the number of edges from a leaf to the root.
   */
  treedepth: bigint

  /**
   * Index of the transaction in the block's payset.
   */
  idx: bigint

  /**
   * The type of hash function used to create the proof, must be one of:
   * * sha512_256
   * * sha256
   */
  hashtype: 'sha512_256' | 'sha256'
}

// JSON DTO shape for TransactionProof with wire keys and JSON-safe primitives
export type TransactionProofDto = {
  proof: string
  stibhash: string
  treedepth: string
  idx: string
  hashtype: 'sha512_256' | 'sha256'
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionProof): TransactionProofDto {
  const out: any = {}
  {
    const v = (value as any)['proof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proof'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['stibhash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stibhash'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['treedepth']
    if (v === undefined) {
      // omit undefined
    } else {
      out['treedepth'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['idx']
    if (v === undefined) {
      // omit undefined
    } else {
      out['idx'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['hashtype']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hashtype'] = v
    }
  }
  return out as TransactionProofDto
}

export function fromDto(dto: TransactionProofDto): TransactionProof {
  const out: any = {}
  {
    const v = (dto as any)['proof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proof'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['stibhash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stibhash'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['treedepth']
    if (v === undefined) {
      // omit undefined
    } else {
      out['treedepth'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['idx']
    if (v === undefined) {
      // omit undefined
    } else {
      out['idx'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['hashtype']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hashtype'] = v as any
    }
  }
  return out as TransactionProof
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionProof): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionProof {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionProof): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionProof {
  return fromDto(raw as TransactionProofDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionProof[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionProof[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionProof[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionProof[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionProofMsgpackDto = {
  proof: Uint8Array
  stibhash: Uint8Array
  treedepth: bigint
  idx: bigint
  hashtype: 'sha512_256' | 'sha256'
}

function toMsgpackDto(value: TransactionProof): TransactionProofMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['proof']
    if (v === undefined) {
    } else {
      out['proof'] = v
    }
  }
  {
    const v = (value as any)['stibhash']
    if (v === undefined) {
    } else {
      out['stibhash'] = v
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
    const v = (value as any)['idx']
    if (v === undefined) {
    } else {
      out['idx'] = v
    }
  }
  {
    const v = (value as any)['hashtype']
    if (v === undefined) {
    } else {
      out['hashtype'] = v
    }
  }
  return out as TransactionProofMsgpackDto
}

function fromMsgpackDto(dto: TransactionProofMsgpackDto): TransactionProof {
  const out: any = {}
  {
    const v = (dto as any)['proof']
    if (v === undefined) {
    } else {
      out['proof'] = v
    }
  }
  {
    const v = (dto as any)['stibhash']
    if (v === undefined) {
    } else {
      out['stibhash'] = v
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
    const v = (dto as any)['idx']
    if (v === undefined) {
    } else {
      out['idx'] = v
    }
  }
  {
    const v = (dto as any)['hashtype']
    if (v === undefined) {
    } else {
      out['hashtype'] = v
    }
  }
  return out as TransactionProof
}

export const TransactionProof = {
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
