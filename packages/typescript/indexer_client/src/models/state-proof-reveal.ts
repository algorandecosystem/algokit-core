import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { StateProofParticipant, StateProofParticipantDto } from './state-proof-participant'
import { StateProofParticipant as StateProofParticipantModel } from './state-proof-participant'
import type { StateProofSigSlot, StateProofSigSlotDto } from './state-proof-sig-slot'
import { StateProofSigSlot as StateProofSigSlotModel } from './state-proof-sig-slot'

export type StateProofReveal = {
  /**
   * The position in the signature and participants arrays corresponding to this entry.
   */
  position?: bigint
  sigSlot?: StateProofSigSlot
  participant?: StateProofParticipant
}

// JSON DTO shape for StateProofReveal with wire keys and JSON-safe primitives
export type StateProofRevealDto = {
  position?: string
  'sig-slot'?: StateProofSigSlotDto
  participant?: StateProofParticipantDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: StateProofReveal): StateProofRevealDto {
  const out: any = {}
  {
    const v = (value as any)['position']
    if (v === undefined) {
      // omit undefined
    } else {
      out['position'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['sigSlot']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sig-slot'] = v === undefined ? v : StateProofSigSlotModel.toDto(v)
    }
  }
  {
    const v = (value as any)['participant']
    if (v === undefined) {
      // omit undefined
    } else {
      out['participant'] = v === undefined ? v : StateProofParticipantModel.toDto(v)
    }
  }
  return out as StateProofRevealDto
}

export function fromDto(dto: StateProofRevealDto): StateProofReveal {
  const out: any = {}
  {
    const v = (dto as any)['position']
    if (v === undefined) {
      // omit undefined
    } else {
      out['position'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['sig-slot']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sigSlot'] = v === undefined ? v : StateProofSigSlotModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['participant']
    if (v === undefined) {
      // omit undefined
    } else {
      out['participant'] = v === undefined ? v : StateProofParticipantModel.fromDto(v)
    }
  }
  return out as StateProofReveal
}

// Msgpack codecs
export function encodeMsgpack(value: StateProofReveal): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): StateProofReveal {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: StateProofReveal): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): StateProofReveal {
  return fromDto(raw as StateProofRevealDto)
}

// Array helpers
export function encodeMsgpackArray(values: StateProofReveal[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): StateProofReveal[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: StateProofReveal[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): StateProofReveal[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type StateProofRevealMsgpackDto = {
  position?: bigint
  'sig-slot'?: ReturnType<(typeof StateProofSigSlotModel)['toMsgpackDto']>
  participant?: ReturnType<(typeof StateProofParticipantModel)['toMsgpackDto']>
}

function toMsgpackDto(value: StateProofReveal): StateProofRevealMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['position']
    if (v === undefined) {
    } else {
      out['position'] = v
    }
  }
  {
    const v = (value as any)['sigSlot']
    if (v === undefined) {
    } else {
      out['sig-slot'] = StateProofSigSlotModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['participant']
    if (v === undefined) {
    } else {
      out['participant'] = StateProofParticipantModel.toMsgpackDto(v)
    }
  }
  return out as StateProofRevealMsgpackDto
}

function fromMsgpackDto(dto: StateProofRevealMsgpackDto): StateProofReveal {
  const out: any = {}
  {
    const v = (dto as any)['position']
    if (v === undefined) {
    } else {
      out['position'] = v
    }
  }
  {
    const v = (dto as any)['sig-slot']
    if (v === undefined) {
    } else {
      out['sigSlot'] = StateProofSigSlotModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['participant']
    if (v === undefined) {
    } else {
      out['participant'] = StateProofParticipantModel.fromMsgpackDto(v)
    }
  }
  return out as StateProofReveal
}

export const StateProofReveal = {
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
