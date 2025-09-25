import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { StateProofVerifier, StateProofVerifierDto } from './state-proof-verifier'
import { StateProofVerifier as StateProofVerifierModel } from './state-proof-verifier'

export type StateProofParticipant = {
  verifier?: StateProofVerifier

  /**
   * \[w\]
   */
  weight?: bigint
}

// JSON DTO shape for StateProofParticipant with wire keys and JSON-safe primitives
export type StateProofParticipantDto = {
  verifier?: StateProofVerifierDto
  weight?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateProofParticipant): StateProofParticipantDto {
  const out: any = {}
  {
    const v = (value as any)['verifier']
    if (v === undefined) {
      // omit undefined
    } else {
      out['verifier'] = v === undefined ? v : StateProofVerifierModel.toDto(v)
    }
  }
  {
    const v = (value as any)['weight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['weight'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as StateProofParticipantDto
}

export function fromDto(dto: StateProofParticipantDto): StateProofParticipant {
  const out: any = {}
  {
    const v = (dto as any)['verifier']
    if (v === undefined) {
      // omit undefined
    } else {
      out['verifier'] = v === undefined ? v : StateProofVerifierModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['weight']
    if (v === undefined) {
      // omit undefined
    } else {
      out['weight'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as StateProofParticipant
}

// Msgpack codecs
export function encodeMsgpack(value: StateProofParticipant): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StateProofParticipant {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StateProofParticipant): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StateProofParticipant {
  return fromDto(raw as StateProofParticipantDto)
}

// Array helpers
export function encodeMsgpackArray(values: StateProofParticipant[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StateProofParticipant[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StateProofParticipant[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StateProofParticipant[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StateProofParticipantMsgpackDto = {
  verifier?: ReturnType<(typeof StateProofVerifierModel)['toMsgpackDto']>
  weight?: bigint
}

function toMsgpackDto(value: StateProofParticipant): StateProofParticipantMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['verifier']
    if (v === undefined) {
    } else {
      out['verifier'] = StateProofVerifierModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['weight']
    if (v === undefined) {
    } else {
      out['weight'] = v
    }
  }
  return out as StateProofParticipantMsgpackDto
}

function fromMsgpackDto(dto: StateProofParticipantMsgpackDto): StateProofParticipant {
  const out: any = {}
  {
    const v = (dto as any)['verifier']
    if (v === undefined) {
    } else {
      out['verifier'] = StateProofVerifierModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['weight']
    if (v === undefined) {
    } else {
      out['weight'] = v
    }
  }
  return out as StateProofParticipant
}

export const StateProofParticipant = {
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
