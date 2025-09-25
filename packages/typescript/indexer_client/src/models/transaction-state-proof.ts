import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { IndexerStateProofMessage, IndexerStateProofMessageDto } from './indexer-state-proof-message'
import { IndexerStateProofMessage as IndexerStateProofMessageModel } from './indexer-state-proof-message'
import type { StateProofFields, StateProofFieldsDto } from './state-proof-fields'
import { StateProofFields as StateProofFieldsModel } from './state-proof-fields'

/**
 * Fields for a state proof transaction.
 *
 * Definition:
 * data/transactions/stateproof.go : StateProofTxnFields
 */
export type TransactionStateProof = {
  /**
   * \[sptype\] Type of the state proof. Integer representing an entry defined in protocol/stateproof.go
   */
  stateProofType?: bigint
  stateProof?: StateProofFields
  message?: IndexerStateProofMessage
}

// JSON DTO shape for TransactionStateProof with wire keys and JSON-safe primitives
export type TransactionStateProofDto = {
  'state-proof-type'?: string
  'state-proof'?: StateProofFieldsDto
  message?: IndexerStateProofMessageDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionStateProof): TransactionStateProofDto {
  const out: any = {}
  {
    const v = (value as any)['stateProofType']
    if (v === undefined) {
      // omit undefined
    } else {
      out['state-proof-type'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['stateProof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['state-proof'] = v === undefined ? v : StateProofFieldsModel.toDto(v)
    }
  }
  {
    const v = (value as any)['message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['message'] = v === undefined ? v : IndexerStateProofMessageModel.toDto(v)
    }
  }
  return out as TransactionStateProofDto
}

export function fromDto(dto: TransactionStateProofDto): TransactionStateProof {
  const out: any = {}
  {
    const v = (dto as any)['state-proof-type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stateProofType'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['state-proof']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stateProof'] = v === undefined ? v : StateProofFieldsModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['message']
    if (v === undefined) {
      // omit undefined
    } else {
      out['message'] = v === undefined ? v : IndexerStateProofMessageModel.fromDto(v)
    }
  }
  return out as TransactionStateProof
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionStateProof): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionStateProof {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionStateProof): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionStateProof {
  return fromDto(raw as TransactionStateProofDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionStateProof[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionStateProof[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionStateProof[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionStateProof[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionStateProofMsgpackDto = {
  'state-proof-type'?: bigint
  'state-proof'?: ReturnType<(typeof StateProofFieldsModel)['toMsgpackDto']>
  message?: ReturnType<(typeof IndexerStateProofMessageModel)['toMsgpackDto']>
}

function toMsgpackDto(value: TransactionStateProof): TransactionStateProofMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['stateProofType']
    if (v === undefined) {
    } else {
      out['state-proof-type'] = v
    }
  }
  {
    const v = (value as any)['stateProof']
    if (v === undefined) {
    } else {
      out['state-proof'] = StateProofFieldsModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['message']
    if (v === undefined) {
    } else {
      out['message'] = IndexerStateProofMessageModel.toMsgpackDto(v)
    }
  }
  return out as TransactionStateProofMsgpackDto
}

function fromMsgpackDto(dto: TransactionStateProofMsgpackDto): TransactionStateProof {
  const out: any = {}
  {
    const v = (dto as any)['state-proof-type']
    if (v === undefined) {
    } else {
      out['stateProofType'] = v
    }
  }
  {
    const v = (dto as any)['state-proof']
    if (v === undefined) {
    } else {
      out['stateProof'] = StateProofFieldsModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['message']
    if (v === undefined) {
    } else {
      out['message'] = IndexerStateProofMessageModel.fromMsgpackDto(v)
    }
  }
  return out as TransactionStateProof
}

export const TransactionStateProof = {
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
