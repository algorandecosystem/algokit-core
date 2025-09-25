import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { HashFactory, HashFactoryDto } from './hash-factory'
import { HashFactory as HashFactoryModel } from './hash-factory'

export type MerkleArrayProof = {
  /**
   * \[pth\]
   */
  path?: Uint8Array[]
  hashFactory?: HashFactory

  /**
   * \[td\]
   */
  treeDepth?: bigint
}

// JSON DTO shape for MerkleArrayProof with wire keys and JSON-safe primitives
export type MerkleArrayProofDto = {
  path?: string[]
  'hash-factory'?: HashFactoryDto
  'tree-depth'?: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: MerkleArrayProof): MerkleArrayProofDto {
  const out: any = {}
  {
    const v = (value as any)['path']
    if (v === undefined) {
      // omit undefined
    } else {
      out['path'] = (v as Uint8Array[]).map(toBase64)
    }
  }
  {
    const v = (value as any)['hashFactory']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hash-factory'] = v === undefined ? v : HashFactoryModel.toDto(v)
    }
  }
  {
    const v = (value as any)['treeDepth']
    if (v === undefined) {
      // omit undefined
    } else {
      out['tree-depth'] = v
    }
  }
  return out as MerkleArrayProofDto
}

export function fromDto(dto: MerkleArrayProofDto): MerkleArrayProof {
  const out: any = {}
  {
    const v = (dto as any)['path']
    if (v === undefined) {
      // omit undefined
    } else {
      out['path'] = (v as string[]).map(fromBase64)
    }
  }
  {
    const v = (dto as any)['hash-factory']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hashFactory'] = v === undefined ? v : HashFactoryModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['tree-depth']
    if (v === undefined) {
      // omit undefined
    } else {
      out['treeDepth'] = v as any
    }
  }
  return out as MerkleArrayProof
}

// Msgpack codecs
export function encodeMsgpack(value: MerkleArrayProof): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): MerkleArrayProof {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: MerkleArrayProof): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): MerkleArrayProof {
  return fromDto(raw as MerkleArrayProofDto)
}

// Array helpers
export function encodeMsgpackArray(values: MerkleArrayProof[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): MerkleArrayProof[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: MerkleArrayProof[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): MerkleArrayProof[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type MerkleArrayProofMsgpackDto = {
  path?: Uint8Array[]
  'hash-factory'?: ReturnType<(typeof HashFactoryModel)['toMsgpackDto']>
  'tree-depth'?: bigint
}

function toMsgpackDto(value: MerkleArrayProof): MerkleArrayProofMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['path']
    if (v === undefined) {
    } else {
      out['path'] = v as any[]
    }
  }
  {
    const v = (value as any)['hashFactory']
    if (v === undefined) {
    } else {
      out['hash-factory'] = HashFactoryModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['treeDepth']
    if (v === undefined) {
    } else {
      out['tree-depth'] = v
    }
  }
  return out as MerkleArrayProofMsgpackDto
}

function fromMsgpackDto(dto: MerkleArrayProofMsgpackDto): MerkleArrayProof {
  const out: any = {}
  {
    const v = (dto as any)['path']
    if (v === undefined) {
    } else {
      out['path'] = v as any[]
    }
  }
  {
    const v = (dto as any)['hash-factory']
    if (v === undefined) {
    } else {
      out['hashFactory'] = HashFactoryModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['tree-depth']
    if (v === undefined) {
    } else {
      out['treeDepth'] = v
    }
  }
  return out as MerkleArrayProof
}

export const MerkleArrayProof = {
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
