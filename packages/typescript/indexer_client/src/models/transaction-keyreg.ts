import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Fields for a keyreg transaction.
 *
 * Definition:
 * data/transactions/keyreg.go : KeyregTxnFields
 */
export type TransactionKeyreg = {
  /**
   * \[nonpart\] Mark the account as participating or non-participating.
   */
  nonParticipation?: boolean

  /**
   * \[selkey\] Public key used with the Verified Random Function (VRF) result during committee selection.
   */
  selectionParticipationKey?: Uint8Array

  /**
   * \[votefst\] First round this participation key is valid.
   */
  voteFirstValid?: bigint

  /**
   * \[votekd\] Number of subkeys in each batch of participation keys.
   */
  voteKeyDilution?: bigint

  /**
   * \[votelst\] Last round this participation key is valid.
   */
  voteLastValid?: bigint

  /**
   * \[votekey\] Participation public key used in key registration transactions.
   */
  voteParticipationKey?: Uint8Array

  /**
   * \[sprfkey\] State proof key used in key registration transactions.
   */
  stateProofKey?: Uint8Array
}

// JSON DTO shape for TransactionKeyreg with wire keys and JSON-safe primitives
export type TransactionKeyregDto = {
  'non-participation'?: boolean
  'selection-participation-key'?: string
  'vote-first-valid'?: string
  'vote-key-dilution'?: string
  'vote-last-valid'?: string
  'vote-participation-key'?: string
  'state-proof-key'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionKeyreg): TransactionKeyregDto {
  const out: any = {}
  {
    const v = (value as any)['nonParticipation']
    if (v === undefined) {
      // omit undefined
    } else {
      out['non-participation'] = v
    }
  }
  {
    const v = (value as any)['selectionParticipationKey']
    if (v === undefined) {
      // omit undefined
    } else {
      out['selection-participation-key'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['voteFirstValid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['vote-first-valid'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['voteKeyDilution']
    if (v === undefined) {
      // omit undefined
    } else {
      out['vote-key-dilution'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['voteLastValid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['vote-last-valid'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['voteParticipationKey']
    if (v === undefined) {
      // omit undefined
    } else {
      out['vote-participation-key'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['stateProofKey']
    if (v === undefined) {
      // omit undefined
    } else {
      out['state-proof-key'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as TransactionKeyregDto
}

export function fromDto(dto: TransactionKeyregDto): TransactionKeyreg {
  const out: any = {}
  {
    const v = (dto as any)['non-participation']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nonParticipation'] = v as any
    }
  }
  {
    const v = (dto as any)['selection-participation-key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['selectionParticipationKey'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['vote-first-valid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['voteFirstValid'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['vote-key-dilution']
    if (v === undefined) {
      // omit undefined
    } else {
      out['voteKeyDilution'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['vote-last-valid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['voteLastValid'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['vote-participation-key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['voteParticipationKey'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['state-proof-key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stateProofKey'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as TransactionKeyreg
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionKeyreg): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionKeyreg {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionKeyreg): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionKeyreg {
  return fromDto(raw as TransactionKeyregDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionKeyreg[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionKeyreg[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionKeyreg[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionKeyreg[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionKeyregMsgpackDto = {
  'non-participation'?: boolean
  'selection-participation-key'?: Uint8Array
  'vote-first-valid'?: bigint
  'vote-key-dilution'?: bigint
  'vote-last-valid'?: bigint
  'vote-participation-key'?: Uint8Array
  'state-proof-key'?: Uint8Array
}

function toMsgpackDto(value: TransactionKeyreg): TransactionKeyregMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['nonParticipation']
    if (v === undefined) {
    } else {
      out['non-participation'] = v
    }
  }
  {
    const v = (value as any)['selectionParticipationKey']
    if (v === undefined) {
    } else {
      out['selection-participation-key'] = v
    }
  }
  {
    const v = (value as any)['voteFirstValid']
    if (v === undefined) {
    } else {
      out['vote-first-valid'] = v
    }
  }
  {
    const v = (value as any)['voteKeyDilution']
    if (v === undefined) {
    } else {
      out['vote-key-dilution'] = v
    }
  }
  {
    const v = (value as any)['voteLastValid']
    if (v === undefined) {
    } else {
      out['vote-last-valid'] = v
    }
  }
  {
    const v = (value as any)['voteParticipationKey']
    if (v === undefined) {
    } else {
      out['vote-participation-key'] = v
    }
  }
  {
    const v = (value as any)['stateProofKey']
    if (v === undefined) {
    } else {
      out['state-proof-key'] = v
    }
  }
  return out as TransactionKeyregMsgpackDto
}

function fromMsgpackDto(dto: TransactionKeyregMsgpackDto): TransactionKeyreg {
  const out: any = {}
  {
    const v = (dto as any)['non-participation']
    if (v === undefined) {
    } else {
      out['nonParticipation'] = v
    }
  }
  {
    const v = (dto as any)['selection-participation-key']
    if (v === undefined) {
    } else {
      out['selectionParticipationKey'] = v
    }
  }
  {
    const v = (dto as any)['vote-first-valid']
    if (v === undefined) {
    } else {
      out['voteFirstValid'] = v
    }
  }
  {
    const v = (dto as any)['vote-key-dilution']
    if (v === undefined) {
    } else {
      out['voteKeyDilution'] = v
    }
  }
  {
    const v = (dto as any)['vote-last-valid']
    if (v === undefined) {
    } else {
      out['voteLastValid'] = v
    }
  }
  {
    const v = (dto as any)['vote-participation-key']
    if (v === undefined) {
    } else {
      out['voteParticipationKey'] = v
    }
  }
  {
    const v = (dto as any)['state-proof-key']
    if (v === undefined) {
    } else {
      out['stateProofKey'] = v
    }
  }
  return out as TransactionKeyreg
}

export const TransactionKeyreg = {
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
