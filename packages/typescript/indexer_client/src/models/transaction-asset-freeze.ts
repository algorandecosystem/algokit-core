import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Fields for an asset freeze transaction.
 *
 * Definition:
 * data/transactions/asset.go : AssetFreezeTxnFields
 */
export type TransactionAssetFreeze = {
  /**
   * \[fadd\] Address of the account whose asset is being frozen or thawed.
   */
  address: string

  /**
   * \[faid\] ID of the asset being frozen or thawed.
   */
  assetId: bigint

  /**
   * \[afrz\] The new freeze status.
   */
  newFreezeStatus: boolean
}

// JSON DTO shape for TransactionAssetFreeze with wire keys and JSON-safe primitives
export type TransactionAssetFreezeDto = {
  address: string
  'asset-id': string
  'new-freeze-status': boolean
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionAssetFreeze): TransactionAssetFreezeDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['assetId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-id'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['newFreezeStatus']
    if (v === undefined) {
      // omit undefined
    } else {
      out['new-freeze-status'] = v
    }
  }
  return out as TransactionAssetFreezeDto
}

export function fromDto(dto: TransactionAssetFreezeDto): TransactionAssetFreeze {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
      // omit undefined
    } else {
      out['address'] = v as any
    }
  }
  {
    const v = (dto as any)['asset-id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetId'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['new-freeze-status']
    if (v === undefined) {
      // omit undefined
    } else {
      out['newFreezeStatus'] = v as any
    }
  }
  return out as TransactionAssetFreeze
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionAssetFreeze): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionAssetFreeze {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionAssetFreeze): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionAssetFreeze {
  return fromDto(raw as TransactionAssetFreezeDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionAssetFreeze[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionAssetFreeze[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionAssetFreeze[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionAssetFreeze[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionAssetFreezeMsgpackDto = {
  address: string
  'asset-id': bigint
  'new-freeze-status': boolean
}

function toMsgpackDto(value: TransactionAssetFreeze): TransactionAssetFreezeMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['assetId']
    if (v === undefined) {
    } else {
      out['asset-id'] = v
    }
  }
  {
    const v = (value as any)['newFreezeStatus']
    if (v === undefined) {
    } else {
      out['new-freeze-status'] = v
    }
  }
  return out as TransactionAssetFreezeMsgpackDto
}

function fromMsgpackDto(dto: TransactionAssetFreezeMsgpackDto): TransactionAssetFreeze {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (dto as any)['asset-id']
    if (v === undefined) {
    } else {
      out['assetId'] = v
    }
  }
  {
    const v = (dto as any)['new-freeze-status']
    if (v === undefined) {
    } else {
      out['newFreezeStatus'] = v
    }
  }
  return out as TransactionAssetFreeze
}

export const TransactionAssetFreeze = {
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
