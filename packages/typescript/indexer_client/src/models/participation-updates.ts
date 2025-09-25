import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Participation account data that needs to be checked/acted on by the network.
 */
export type ParticipationUpdates = {
  /**
   * \[partupdrmv\] a list of online accounts that needs to be converted to offline since their participation key expired.
   */
  expiredParticipationAccounts?: string[]

  /**
   * \[partupabs\] a list of online accounts that need to be suspended.
   */
  absentParticipationAccounts?: string[]
}

// JSON DTO shape for ParticipationUpdates with wire keys and JSON-safe primitives
export type ParticipationUpdatesDto = {
  'expired-participation-accounts'?: string[]
  'absent-participation-accounts'?: string[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ParticipationUpdates): ParticipationUpdatesDto {
  const out: any = {}
  {
    const v = (value as any)['expiredParticipationAccounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['expired-participation-accounts'] = v as any[]
    }
  }
  {
    const v = (value as any)['absentParticipationAccounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['absent-participation-accounts'] = v as any[]
    }
  }
  return out as ParticipationUpdatesDto
}

export function fromDto(dto: ParticipationUpdatesDto): ParticipationUpdates {
  const out: any = {}
  {
    const v = (dto as any)['expired-participation-accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['expiredParticipationAccounts'] = v as any[]
    }
  }
  {
    const v = (dto as any)['absent-participation-accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['absentParticipationAccounts'] = v as any[]
    }
  }
  return out as ParticipationUpdates
}

// Msgpack codecs
export function encodeMsgpack(value: ParticipationUpdates): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ParticipationUpdates {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ParticipationUpdates): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ParticipationUpdates {
  return fromDto(raw as ParticipationUpdatesDto)
}

// Array helpers
export function encodeMsgpackArray(values: ParticipationUpdates[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ParticipationUpdates[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ParticipationUpdates[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ParticipationUpdates[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ParticipationUpdatesMsgpackDto = {
  'expired-participation-accounts'?: string[]
  'absent-participation-accounts'?: string[]
}

function toMsgpackDto(value: ParticipationUpdates): ParticipationUpdatesMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['expiredParticipationAccounts']
    if (v === undefined) {
    } else {
      out['expired-participation-accounts'] = v as any[]
    }
  }
  {
    const v = (value as any)['absentParticipationAccounts']
    if (v === undefined) {
    } else {
      out['absent-participation-accounts'] = v as any[]
    }
  }
  return out as ParticipationUpdatesMsgpackDto
}

function fromMsgpackDto(dto: ParticipationUpdatesMsgpackDto): ParticipationUpdates {
  const out: any = {}
  {
    const v = (dto as any)['expired-participation-accounts']
    if (v === undefined) {
    } else {
      out['expiredParticipationAccounts'] = v as any[]
    }
  }
  {
    const v = (dto as any)['absent-participation-accounts']
    if (v === undefined) {
    } else {
      out['absentParticipationAccounts'] = v as any[]
    }
  }
  return out as ParticipationUpdates
}

export const ParticipationUpdates = {
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
