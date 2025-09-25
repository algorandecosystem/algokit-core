import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { HbProofFields, HbProofFieldsDto } from './hb-proof-fields'
import { HbProofFields as HbProofFieldsModel } from './hb-proof-fields'

/**
 * Fields for a heartbeat transaction.
 *
 * Definition:
 * data/transactions/heartbeat.go : HeartbeatTxnFields
 */
export type TransactionHeartbeat = {
  /**
   * \[hbad\] HbAddress is the account this txn is proving onlineness for.
   */
  hbAddress: string
  hbProof: HbProofFields

  /**
   * \[hbsd\] HbSeed must be the block seed for the this transaction's firstValid block.
   */
  hbSeed: Uint8Array

  /**
   * \[hbvid\] HbVoteID must match the HbAddress account's current VoteID.
   */
  hbVoteId: Uint8Array

  /**
   * \[hbkd\] HbKeyDilution must match HbAddress account's current KeyDilution.
   */
  hbKeyDilution: bigint
}

// JSON DTO shape for TransactionHeartbeat with wire keys and JSON-safe primitives
export type TransactionHeartbeatDto = {
  'hb-address': string
  'hb-proof': HbProofFieldsDto
  'hb-seed': string
  'hb-vote-id': string
  'hb-key-dilution': string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionHeartbeat): TransactionHeartbeatDto {
  const out: any = {}
  {
    const v = (value as any)['hbAddress']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hb-address'] = v
    }
  }
  {
    const v = (value as any)['hbProof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hb-proof'] = v === undefined ? v : HbProofFieldsModel.toDto(v)
    }
  }
  {
    const v = (value as any)['hbSeed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hb-seed'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['hbVoteId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hb-vote-id'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['hbKeyDilution']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hb-key-dilution'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as TransactionHeartbeatDto
}

export function fromDto(dto: TransactionHeartbeatDto): TransactionHeartbeat {
  const out: any = {}
  {
    const v = (dto as any)['hb-address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hbAddress'] = v as any
    }
  }
  {
    const v = (dto as any)['hb-proof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hbProof'] = v === undefined ? v : HbProofFieldsModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['hb-seed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hbSeed'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['hb-vote-id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hbVoteId'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['hb-key-dilution']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hbKeyDilution'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as TransactionHeartbeat
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionHeartbeat): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionHeartbeat {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionHeartbeat): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionHeartbeat {
  return fromDto(raw as TransactionHeartbeatDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionHeartbeat[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionHeartbeat[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionHeartbeat[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionHeartbeat[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionHeartbeatMsgpackDto = {
  'hb-address': string
  'hb-proof': ReturnType<(typeof HbProofFieldsModel)['toMsgpackDto']>
  'hb-seed': Uint8Array
  'hb-vote-id': Uint8Array
  'hb-key-dilution': bigint
}

function toMsgpackDto(value: TransactionHeartbeat): TransactionHeartbeatMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['hbAddress']
    if (v === undefined) {
    } else {
      out['hb-address'] = v
    }
  }
  {
    const v = (value as any)['hbProof']
    if (v === undefined) {
    } else {
      out['hb-proof'] = HbProofFieldsModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['hbSeed']
    if (v === undefined) {
    } else {
      out['hb-seed'] = v
    }
  }
  {
    const v = (value as any)['hbVoteId']
    if (v === undefined) {
    } else {
      out['hb-vote-id'] = v
    }
  }
  {
    const v = (value as any)['hbKeyDilution']
    if (v === undefined) {
    } else {
      out['hb-key-dilution'] = v
    }
  }
  return out as TransactionHeartbeatMsgpackDto
}

function fromMsgpackDto(dto: TransactionHeartbeatMsgpackDto): TransactionHeartbeat {
  const out: any = {}
  {
    const v = (dto as any)['hb-address']
    if (v === undefined) {
    } else {
      out['hbAddress'] = v
    }
  }
  {
    const v = (dto as any)['hb-proof']
    if (v === undefined) {
    } else {
      out['hbProof'] = HbProofFieldsModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['hb-seed']
    if (v === undefined) {
    } else {
      out['hbSeed'] = v
    }
  }
  {
    const v = (dto as any)['hb-vote-id']
    if (v === undefined) {
    } else {
      out['hbVoteId'] = v
    }
  }
  {
    const v = (dto as any)['hb-key-dilution']
    if (v === undefined) {
    } else {
      out['hbKeyDilution'] = v
    }
  }
  return out as TransactionHeartbeat
}

export const TransactionHeartbeat = {
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
