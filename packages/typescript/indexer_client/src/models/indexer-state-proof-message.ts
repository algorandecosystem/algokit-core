import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type IndexerStateProofMessage = {
  /**
   * \[b\]
   */
  blockHeadersCommitment?: Uint8Array

  /**
   * \[v\]
   */
  votersCommitment?: Uint8Array

  /**
   * \[P\]
   */
  lnProvenWeight?: bigint

  /**
   * \[f\]
   */
  firstAttestedRound?: bigint

  /**
   * \[l\]
   */
  latestAttestedRound?: bigint
}

// JSON DTO shape for IndexerStateProofMessage with wire keys and JSON-safe primitives
export type IndexerStateProofMessageDto = {
  'block-headers-commitment'?: string
  'voters-commitment'?: string
  'ln-proven-weight'?: string
  'first-attested-round'?: string
  'latest-attested-round'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: IndexerStateProofMessage): IndexerStateProofMessageDto {
  const out: any = {}
  {
    const v = (value as any)['blockHeadersCommitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['block-headers-commitment'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['votersCommitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['voters-commitment'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['lnProvenWeight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['ln-proven-weight'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['firstAttestedRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['first-attested-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['latestAttestedRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['latest-attested-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as IndexerStateProofMessageDto
}

export function fromDto(dto: IndexerStateProofMessageDto): IndexerStateProofMessage {
  const out: any = {}
  {
    const v = (dto as any)['block-headers-commitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['blockHeadersCommitment'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['voters-commitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['votersCommitment'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['ln-proven-weight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lnProvenWeight'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['first-attested-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['firstAttestedRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['latest-attested-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['latestAttestedRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as IndexerStateProofMessage
}

// Msgpack codecs
export function encodeMsgpack(value: IndexerStateProofMessage): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): IndexerStateProofMessage {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: IndexerStateProofMessage): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): IndexerStateProofMessage {
  return fromDto(raw as IndexerStateProofMessageDto)
}

// Array helpers
export function encodeMsgpackArray(values: IndexerStateProofMessage[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): IndexerStateProofMessage[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: IndexerStateProofMessage[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): IndexerStateProofMessage[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type IndexerStateProofMessageMsgpackDto = {
  'block-headers-commitment'?: Uint8Array
  'voters-commitment'?: Uint8Array
  'ln-proven-weight'?: bigint
  'first-attested-round'?: bigint
  'latest-attested-round'?: bigint
}

function toMsgpackDto(value: IndexerStateProofMessage): IndexerStateProofMessageMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['blockHeadersCommitment']
    if (v === undefined) {
    } else {
      out['block-headers-commitment'] = v
    }
  }
  {
    const v = (value as any)['votersCommitment']
    if (v === undefined) {
    } else {
      out['voters-commitment'] = v
    }
  }
  {
    const v = (value as any)['lnProvenWeight']
    if (v === undefined) {
    } else {
      out['ln-proven-weight'] = v
    }
  }
  {
    const v = (value as any)['firstAttestedRound']
    if (v === undefined) {
    } else {
      out['first-attested-round'] = v
    }
  }
  {
    const v = (value as any)['latestAttestedRound']
    if (v === undefined) {
    } else {
      out['latest-attested-round'] = v
    }
  }
  return out as IndexerStateProofMessageMsgpackDto
}

function fromMsgpackDto(dto: IndexerStateProofMessageMsgpackDto): IndexerStateProofMessage {
  const out: any = {}
  {
    const v = (dto as any)['block-headers-commitment']
    if (v === undefined) {
    } else {
      out['blockHeadersCommitment'] = v
    }
  }
  {
    const v = (dto as any)['voters-commitment']
    if (v === undefined) {
    } else {
      out['votersCommitment'] = v
    }
  }
  {
    const v = (dto as any)['ln-proven-weight']
    if (v === undefined) {
    } else {
      out['lnProvenWeight'] = v
    }
  }
  {
    const v = (dto as any)['first-attested-round']
    if (v === undefined) {
    } else {
      out['firstAttestedRound'] = v
    }
  }
  {
    const v = (dto as any)['latest-attested-round']
    if (v === undefined) {
    } else {
      out['latestAttestedRound'] = v
    }
  }
  return out as IndexerStateProofMessage
}

export const IndexerStateProofMessage = {
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
