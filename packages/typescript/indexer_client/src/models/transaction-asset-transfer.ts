import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Fields for an asset transfer transaction.
 *
 * Definition:
 * data/transactions/asset.go : AssetTransferTxnFields
 */
export type TransactionAssetTransfer = {
  /**
   * \[aamt\] Amount of asset to transfer. A zero amount transferred to self allocates that asset in the account's Assets map.
   */
  amount: bigint

  /**
   * \[xaid\] ID of the asset being transferred.
   */
  assetId: bigint

  /**
   * Number of assets transferred to the close-to account as part of the transaction.
   */
  closeAmount?: bigint

  /**
   * \[aclose\] Indicates that the asset should be removed from the account's Assets map, and specifies where the remaining asset holdings should be transferred.  It's always valid to transfer remaining asset holdings to the creator account.
   */
  closeTo?: string

  /**
   * \[arcv\] Recipient address of the transfer.
   */
  receiver: string

  /**
   * \[asnd\] The effective sender during a clawback transactions. If this is not a zero value, the real transaction sender must be the Clawback address from the AssetParams.
   */
  sender?: string
}

// JSON DTO shape for TransactionAssetTransfer with wire keys and JSON-safe primitives
export type TransactionAssetTransferDto = {
  amount: string
  'asset-id': string
  'close-amount'?: string
  'close-to'?: string
  receiver: string
  sender?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionAssetTransfer): TransactionAssetTransferDto {
  const out: any = {}
  {
    const v = (value as any)['amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['amount'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
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
    const v = (value as any)['closeAmount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['close-amount'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['closeTo']
    if (v === undefined) {
      // omit undefined
    } else {
      out['close-to'] = v
    }
  }
  {
    const v = (value as any)['receiver']
    if (v === undefined) {
      // omit undefined
    } else {
      out['receiver'] = v
    }
  }
  {
    const v = (value as any)['sender']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sender'] = v
    }
  }
  return out as TransactionAssetTransferDto
}

export function fromDto(dto: TransactionAssetTransferDto): TransactionAssetTransfer {
  const out: any = {}
  {
    const v = (dto as any)['amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['amount'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
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
    const v = (dto as any)['close-amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closeAmount'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['close-to']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closeTo'] = v as any
    }
  }
  {
    const v = (dto as any)['receiver']
    if (v === undefined) {
      // omit undefined
    } else {
      out['receiver'] = v as any
    }
  }
  {
    const v = (dto as any)['sender']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sender'] = v as any
    }
  }
  return out as TransactionAssetTransfer
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionAssetTransfer): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionAssetTransfer {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionAssetTransfer): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionAssetTransfer {
  return fromDto(raw as TransactionAssetTransferDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionAssetTransfer[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionAssetTransfer[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionAssetTransfer[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionAssetTransfer[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionAssetTransferMsgpackDto = {
  amount: bigint
  'asset-id': bigint
  'close-amount'?: bigint
  'close-to'?: string
  receiver: string
  sender?: string
}

function toMsgpackDto(value: TransactionAssetTransfer): TransactionAssetTransferMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['amount']
    if (v === undefined) {
    } else {
      out['amount'] = v
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
    const v = (value as any)['closeAmount']
    if (v === undefined) {
    } else {
      out['close-amount'] = v
    }
  }
  {
    const v = (value as any)['closeTo']
    if (v === undefined) {
    } else {
      out['close-to'] = v
    }
  }
  {
    const v = (value as any)['receiver']
    if (v === undefined) {
    } else {
      out['receiver'] = v
    }
  }
  {
    const v = (value as any)['sender']
    if (v === undefined) {
    } else {
      out['sender'] = v
    }
  }
  return out as TransactionAssetTransferMsgpackDto
}

function fromMsgpackDto(dto: TransactionAssetTransferMsgpackDto): TransactionAssetTransfer {
  const out: any = {}
  {
    const v = (dto as any)['amount']
    if (v === undefined) {
    } else {
      out['amount'] = v
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
    const v = (dto as any)['close-amount']
    if (v === undefined) {
    } else {
      out['closeAmount'] = v
    }
  }
  {
    const v = (dto as any)['close-to']
    if (v === undefined) {
    } else {
      out['closeTo'] = v
    }
  }
  {
    const v = (dto as any)['receiver']
    if (v === undefined) {
    } else {
      out['receiver'] = v
    }
  }
  {
    const v = (dto as any)['sender']
    if (v === undefined) {
    } else {
      out['sender'] = v
    }
  }
  return out as TransactionAssetTransfer
}

export const TransactionAssetTransfer = {
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
