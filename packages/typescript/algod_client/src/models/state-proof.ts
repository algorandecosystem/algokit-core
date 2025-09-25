import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { StateProofMessage, StateProofMessageDto } from './state-proof-message'
import { StateProofMessage as StateProofMessageModel } from './state-proof-message'

/**
 * Represents a state proof and its corresponding message
 */
export type StateProof = {
  message: StateProofMessage

  /**
   * The encoded StateProof for the message.
   */
  stateProof: Uint8Array
}

// JSON DTO shape for StateProof with wire keys and JSON-safe primitives
export type StateProofDto = {
  Message: StateProofMessageDto
  StateProof: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateProof): StateProofDto {
  const out: any = {}
  {
    const v = (value as any)['message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['Message'] = v === undefined ? v : StateProofMessageModel.toDto(v)
    }
  }
  {
    const v = (value as any)['stateProof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['StateProof'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as StateProofDto
}

export function fromDto(dto: StateProofDto): StateProof {
  const out: any = {}
  {
    const v = (dto as any)['Message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['message'] = v === undefined ? v : StateProofMessageModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['StateProof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stateProof'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as StateProof
}

// Msgpack codecs
export function encodeMsgpack(value: StateProof): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StateProof {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StateProof): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StateProof {
  return fromDto(raw as StateProofDto)
}

// Array helpers
export function encodeMsgpackArray(values: StateProof[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StateProof[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StateProof[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StateProof[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StateProofMsgpackDto = {
  Message: ReturnType<(typeof StateProofMessageModel)['toMsgpackDto']>
  StateProof: Uint8Array
}

function toMsgpackDto(value: StateProof): StateProofMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['message']
    if (v === undefined) {
    } else {
      out['Message'] = StateProofMessageModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['stateProof']
    if (v === undefined) {
    } else {
      out['StateProof'] = v
    }
  }
  return out as StateProofMsgpackDto
}

function fromMsgpackDto(dto: StateProofMsgpackDto): StateProof {
  const out: any = {}
  {
    const v = (dto as any)['Message']
    if (v === undefined) {
    } else {
      out['message'] = StateProofMessageModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['StateProof']
    if (v === undefined) {
    } else {
      out['stateProof'] = v
    }
  }
  return out as StateProof
}

export const StateProof = {
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
