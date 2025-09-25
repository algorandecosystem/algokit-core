import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { MerkleArrayProof, MerkleArrayProofDto } from './merkle-array-proof'
import { MerkleArrayProof as MerkleArrayProofModel } from './merkle-array-proof'

export type StateProofSignature = {
  falconSignature?: Uint8Array
  merkleArrayIndex?: bigint
  proof?: MerkleArrayProof

  /**
   * \[vkey\]
   */
  verifyingKey?: Uint8Array
}

// JSON DTO shape for StateProofSignature with wire keys and JSON-safe primitives
export type StateProofSignatureDto = {
  'falcon-signature'?: string
  'merkle-array-index'?: bigint
  proof?: MerkleArrayProofDto
  'verifying-key'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateProofSignature): StateProofSignatureDto {
  const out: any = {}
  {
    const v = (value as any)['falconSignature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['falcon-signature'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['merkleArrayIndex']
    if (v === undefined) {
      // omit undefined
    } else {
      out['merkle-array-index'] = v
    }
  }
  {
    const v = (value as any)['proof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proof'] = v === undefined ? v : MerkleArrayProofModel.toDto(v)
    }
  }
  {
    const v = (value as any)['verifyingKey']
    if (v === undefined) {
      // omit undefined
    } else {
      out['verifying-key'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as StateProofSignatureDto
}

export function fromDto(dto: StateProofSignatureDto): StateProofSignature {
  const out: any = {}
  {
    const v = (dto as any)['falcon-signature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['falconSignature'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['merkle-array-index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['merkleArrayIndex'] = v as any
    }
  }
  {
    const v = (dto as any)['proof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['proof'] = v === undefined ? v : MerkleArrayProofModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['verifying-key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['verifyingKey'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as StateProofSignature
}

// Msgpack codecs
export function encodeMsgpack(value: StateProofSignature): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StateProofSignature {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StateProofSignature): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StateProofSignature {
  return fromDto(raw as StateProofSignatureDto)
}

// Array helpers
export function encodeMsgpackArray(values: StateProofSignature[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StateProofSignature[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StateProofSignature[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StateProofSignature[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StateProofSignatureMsgpackDto = {
  'falcon-signature'?: Uint8Array
  'merkle-array-index'?: bigint
  proof?: ReturnType<(typeof MerkleArrayProofModel)['toMsgpackDto']>
  'verifying-key'?: Uint8Array
}

function toMsgpackDto(value: StateProofSignature): StateProofSignatureMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['falconSignature']
    if (v === undefined) {
    } else {
      out['falcon-signature'] = v
    }
  }
  {
    const v = (value as any)['merkleArrayIndex']
    if (v === undefined) {
    } else {
      out['merkle-array-index'] = v
    }
  }
  {
    const v = (value as any)['proof']
    if (v === undefined) {
    } else {
      out['proof'] = MerkleArrayProofModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['verifyingKey']
    if (v === undefined) {
    } else {
      out['verifying-key'] = v
    }
  }
  return out as StateProofSignatureMsgpackDto
}

function fromMsgpackDto(dto: StateProofSignatureMsgpackDto): StateProofSignature {
  const out: any = {}
  {
    const v = (dto as any)['falcon-signature']
    if (v === undefined) {
    } else {
      out['falconSignature'] = v
    }
  }
  {
    const v = (dto as any)['merkle-array-index']
    if (v === undefined) {
    } else {
      out['merkleArrayIndex'] = v
    }
  }
  {
    const v = (dto as any)['proof']
    if (v === undefined) {
    } else {
      out['proof'] = MerkleArrayProofModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['verifying-key']
    if (v === undefined) {
    } else {
      out['verifyingKey'] = v
    }
  }
  return out as StateProofSignature
}

export const StateProofSignature = {
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
