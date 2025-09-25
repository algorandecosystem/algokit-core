import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * TransactionParams contains the parameters that help a client construct
 * a new transaction.
 */
export type TransactionParams = {
  /**
   * ConsensusVersion indicates the consensus protocol version
   * as of LastRound.
   */
  consensusVersion: string

  /**
   * Fee is the suggested transaction fee
   * Fee is in units of micro-Algos per byte.
   * Fee may fall to zero but transactions must still have a fee of
   * at least MinTxnFee for the current network protocol.
   */
  fee: bigint

  /**
   * GenesisHash is the hash of the genesis block.
   */
  genesisHash: Uint8Array

  /**
   * GenesisID is an ID listed in the genesis block.
   */
  genesisId: string

  /**
   * LastRound indicates the last round seen
   */
  lastRound: bigint

  /**
   * The minimum transaction fee (not per byte) required for the
   * txn to validate for the current network protocol.
   */
  minFee: bigint
}

// JSON DTO shape for TransactionParams with wire keys and JSON-safe primitives
export type TransactionParamsDto = {
  'consensus-version': string
  fee: string
  'genesis-hash': string
  'genesis-id': string
  'last-round': string
  'min-fee': string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionParams): TransactionParamsDto {
  const out: any = {}
  {
    const v = (value as any)['consensusVersion']
    if (v === undefined) {
      // omit undefined
    } else {
      out['consensus-version'] = v
    }
  }
  {
    const v = (value as any)['fee']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fee'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['genesisHash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesis-hash'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['genesisId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesis-id'] = v
    }
  }
  {
    const v = (value as any)['lastRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['minFee']
    if (v === undefined) {
      // omit undefined
    } else {
      out['min-fee'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as TransactionParamsDto
}

export function fromDto(dto: TransactionParamsDto): TransactionParams {
  const out: any = {}
  {
    const v = (dto as any)['consensus-version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['consensusVersion'] = v as any
    }
  }
  {
    const v = (dto as any)['fee']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fee'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['genesis-hash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesisHash'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['genesis-id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['genesisId'] = v as any
    }
  }
  {
    const v = (dto as any)['last-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['min-fee']
    if (v === undefined) {
      // omit undefined
    } else {
      out['minFee'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as TransactionParams
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionParams): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionParams {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionParams): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionParams {
  return fromDto(raw as TransactionParamsDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionParams[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionParams[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionParams[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionParams[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionParamsMsgpackDto = {
  'consensus-version': string
  fee: bigint
  'genesis-hash': Uint8Array
  'genesis-id': string
  'last-round': bigint
  'min-fee': bigint
}

function toMsgpackDto(value: TransactionParams): TransactionParamsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['consensusVersion']
    if (v === undefined) {
    } else {
      out['consensus-version'] = v
    }
  }
  {
    const v = (value as any)['fee']
    if (v === undefined) {
    } else {
      out['fee'] = v
    }
  }
  {
    const v = (value as any)['genesisHash']
    if (v === undefined) {
    } else {
      out['genesis-hash'] = v
    }
  }
  {
    const v = (value as any)['genesisId']
    if (v === undefined) {
    } else {
      out['genesis-id'] = v
    }
  }
  {
    const v = (value as any)['lastRound']
    if (v === undefined) {
    } else {
      out['last-round'] = v
    }
  }
  {
    const v = (value as any)['minFee']
    if (v === undefined) {
    } else {
      out['min-fee'] = v
    }
  }
  return out as TransactionParamsMsgpackDto
}

function fromMsgpackDto(dto: TransactionParamsMsgpackDto): TransactionParams {
  const out: any = {}
  {
    const v = (dto as any)['consensus-version']
    if (v === undefined) {
    } else {
      out['consensusVersion'] = v
    }
  }
  {
    const v = (dto as any)['fee']
    if (v === undefined) {
    } else {
      out['fee'] = v
    }
  }
  {
    const v = (dto as any)['genesis-hash']
    if (v === undefined) {
    } else {
      out['genesisHash'] = v
    }
  }
  {
    const v = (dto as any)['genesis-id']
    if (v === undefined) {
    } else {
      out['genesisId'] = v
    }
  }
  {
    const v = (dto as any)['last-round']
    if (v === undefined) {
    } else {
      out['lastRound'] = v
    }
  }
  {
    const v = (dto as any)['min-fee']
    if (v === undefined) {
    } else {
      out['minFee'] = v
    }
  }
  return out as TransactionParams
}

export const TransactionParams = {
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
