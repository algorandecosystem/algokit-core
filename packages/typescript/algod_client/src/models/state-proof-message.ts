import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Represents the message that the state proofs are attesting to.
 */
export type StateProofMessage = {
  /**
   * The vector commitment root on all light block headers within a state proof interval.
   */
  blockHeadersCommitment: Uint8Array

  /**
   * The vector commitment root of the top N accounts to sign the next StateProof.
   */
  votersCommitment: Uint8Array

  /**
   * An integer value representing the natural log of the proven weight with 16 bits of precision. This value would be used to verify the next state proof.
   */
  lnProvenWeight: bigint

  /**
   * The first round the message attests to.
   */
  firstAttestedRound: bigint

  /**
   * The last round the message attests to.
   */
  lastAttestedRound: bigint
}

// JSON DTO shape for StateProofMessage with wire keys and JSON-safe primitives
export type StateProofMessageDto = {
  BlockHeadersCommitment: string
  VotersCommitment: string
  LnProvenWeight: string
  FirstAttestedRound: string
  LastAttestedRound: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateProofMessage): StateProofMessageDto {
  const out: any = {}
  {
    const v = (value as any)['blockHeadersCommitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['BlockHeadersCommitment'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['votersCommitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['VotersCommitment'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['lnProvenWeight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['LnProvenWeight'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['firstAttestedRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['FirstAttestedRound'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['lastAttestedRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['LastAttestedRound'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as StateProofMessageDto
}

export function fromDto(dto: StateProofMessageDto): StateProofMessage {
  const out: any = {}
  {
    const v = (dto as any)['BlockHeadersCommitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['blockHeadersCommitment'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['VotersCommitment']
    if (v === undefined) {
      // omit undefined
    } else {
      out['votersCommitment'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['LnProvenWeight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lnProvenWeight'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['FirstAttestedRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['firstAttestedRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['LastAttestedRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastAttestedRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as StateProofMessage
}

// Msgpack codecs
export function encodeMsgpack(value: StateProofMessage): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StateProofMessage {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StateProofMessage): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StateProofMessage {
  return fromDto(raw as StateProofMessageDto)
}

// Array helpers
export function encodeMsgpackArray(values: StateProofMessage[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StateProofMessage[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StateProofMessage[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StateProofMessage[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StateProofMessageMsgpackDto = {
  BlockHeadersCommitment: Uint8Array
  VotersCommitment: Uint8Array
  LnProvenWeight: bigint
  FirstAttestedRound: bigint
  LastAttestedRound: bigint
}

function toMsgpackDto(value: StateProofMessage): StateProofMessageMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['blockHeadersCommitment']
    if (v === undefined) {
    } else {
      out['BlockHeadersCommitment'] = v
    }
  }
  {
    const v = (value as any)['votersCommitment']
    if (v === undefined) {
    } else {
      out['VotersCommitment'] = v
    }
  }
  {
    const v = (value as any)['lnProvenWeight']
    if (v === undefined) {
    } else {
      out['LnProvenWeight'] = v
    }
  }
  {
    const v = (value as any)['firstAttestedRound']
    if (v === undefined) {
    } else {
      out['FirstAttestedRound'] = v
    }
  }
  {
    const v = (value as any)['lastAttestedRound']
    if (v === undefined) {
    } else {
      out['LastAttestedRound'] = v
    }
  }
  return out as StateProofMessageMsgpackDto
}

function fromMsgpackDto(dto: StateProofMessageMsgpackDto): StateProofMessage {
  const out: any = {}
  {
    const v = (dto as any)['BlockHeadersCommitment']
    if (v === undefined) {
    } else {
      out['blockHeadersCommitment'] = v
    }
  }
  {
    const v = (dto as any)['VotersCommitment']
    if (v === undefined) {
    } else {
      out['votersCommitment'] = v
    }
  }
  {
    const v = (dto as any)['LnProvenWeight']
    if (v === undefined) {
    } else {
      out['lnProvenWeight'] = v
    }
  }
  {
    const v = (dto as any)['FirstAttestedRound']
    if (v === undefined) {
    } else {
      out['firstAttestedRound'] = v
    }
  }
  {
    const v = (dto as any)['LastAttestedRound']
    if (v === undefined) {
    } else {
      out['lastAttestedRound'] = v
    }
  }
  return out as StateProofMessage
}

export const StateProofMessage = {
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
