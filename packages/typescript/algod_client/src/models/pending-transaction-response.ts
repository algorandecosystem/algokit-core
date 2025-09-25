import { encodeSignedTransaction, encodeSignedTransactions, decodeSignedTransaction } from '@algorandfoundation/algokit-transact'
import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AccountStateDelta, AccountStateDeltaDto } from './account-state-delta'
import { AccountStateDelta as AccountStateDeltaModel } from './account-state-delta'
import type { StateDelta, StateDeltaDto } from './state-delta'
import { StateDelta as StateDeltaModel } from './state-delta'

/**
 * Details about a pending transaction. If the transaction was recently confirmed, includes confirmation details like the round and reward details.
 */
export type PendingTransactionResponse = {
  /**
   * The asset index if the transaction was found and it created an asset.
   */
  assetId?: bigint

  /**
   * The application index if the transaction was found and it created an application.
   */
  appId?: bigint

  /**
   * Rewards in microalgos applied to the close remainder to account.
   */
  closeRewards?: bigint

  /**
   * Closing amount for the transaction.
   */
  closingAmount?: bigint

  /**
   * The number of the asset's unit that were transferred to the close-to address.
   */
  assetClosingAmount?: bigint

  /**
   * The round where this transaction was confirmed, if present.
   */
  confirmedRound?: bigint

  /**
   * Indicates that the transaction was kicked out of this node's transaction pool (and specifies why that happened).  An empty string indicates the transaction wasn't kicked out of this node's txpool due to an error.
   */
  poolError: string

  /**
   * Rewards in microalgos applied to the receiver account.
   */
  receiverRewards?: bigint

  /**
   * Rewards in microalgos applied to the sender account.
   */
  senderRewards?: bigint

  /**
   * Local state key/value changes for the application being executed by this transaction.
   */
  localStateDelta?: AccountStateDelta[]
  globalStateDelta?: StateDelta

  /**
   * Logs for the application being executed by this transaction.
   */
  logs?: Uint8Array[]

  /**
   * Inner transactions produced by application execution.
   */
  innerTxns?: PendingTransactionResponse[]

  /**
   * The raw signed transaction.
   */
  txn: SignedTransaction
}

// JSON DTO shape for PendingTransactionResponse with wire keys and JSON-safe primitives
export type PendingTransactionResponseDto = {
  'asset-index'?: string
  'application-index'?: string
  'close-rewards'?: bigint
  'closing-amount'?: string
  'asset-closing-amount'?: string
  'confirmed-round'?: string
  'pool-error': string
  'receiver-rewards'?: string
  'sender-rewards'?: string
  'local-state-delta'?: AccountStateDeltaDto[]
  'global-state-delta'?: StateDeltaDto
  logs?: string[]
  'inner-txns'?: PendingTransactionResponseDto[]
  txn: unknown
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: PendingTransactionResponse): PendingTransactionResponseDto {
  const out: any = {}
  {
    const v = (value as any)['assetId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-index'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['appId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['application-index'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['closeRewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['close-rewards'] = v
    }
  }
  {
    const v = (value as any)['closingAmount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closing-amount'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['assetClosingAmount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-closing-amount'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['confirmedRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['confirmed-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['poolError']
    if (v === undefined) {
      // omit undefined
    } else {
      out['pool-error'] = v
    }
  }
  {
    const v = (value as any)['receiverRewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['receiver-rewards'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['senderRewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sender-rewards'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['localStateDelta']
    if (v === undefined) {
      // omit undefined
    } else {
      out['local-state-delta'] = (v as any[]).map((item) => AccountStateDeltaModel.toDto(item))
    }
  }
  {
    const v = (value as any)['globalStateDelta']
    if (v === undefined) {
      // omit undefined
    } else {
      out['global-state-delta'] = v === undefined ? v : StateDeltaModel.toDto(v)
    }
  }
  {
    const v = (value as any)['logs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logs'] = (v as Uint8Array[]).map(toBase64)
    }
  }
  {
    const v = (value as any)['innerTxns']
    if (v === undefined) {
      // omit undefined
    } else {
      out['inner-txns'] = (v as any[]).map((item) => PendingTransactionResponseModel.toDto(item))
    }
  }
  {
    const v = (value as any)['txn']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txn'] = ((): any => {
        const item = v as any
        if (item && typeof item === 'object' && 'txn' in item) return item
        return toBase64(encodeSignedTransaction(item as any))
      })()
    }
  }
  return out as PendingTransactionResponseDto
}

export function fromDto(dto: PendingTransactionResponseDto): PendingTransactionResponse {
  const out: any = {}
  {
    const v = (dto as any)['asset-index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetId'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['application-index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appId'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['close-rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closeRewards'] = v as any
    }
  }
  {
    const v = (dto as any)['closing-amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closingAmount'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['asset-closing-amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetClosingAmount'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['confirmed-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['confirmedRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['pool-error']
    if (v === undefined) {
      // omit undefined
    } else {
      out['poolError'] = v as any
    }
  }
  {
    const v = (dto as any)['receiver-rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['receiverRewards'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['sender-rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['senderRewards'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['local-state-delta']
    if (v === undefined) {
      // omit undefined
    } else {
      out['localStateDelta'] = (v as any[]).map((item) => AccountStateDeltaModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['global-state-delta']
    if (v === undefined) {
      // omit undefined
    } else {
      out['globalStateDelta'] = v === undefined ? v : StateDeltaModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['logs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logs'] = (v as string[]).map(fromBase64)
    }
  }
  {
    const v = (dto as any)['inner-txns']
    if (v === undefined) {
      // omit undefined
    } else {
      out['innerTxns'] = (v as any[]).map((item) => PendingTransactionResponseModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['txn']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txn'] = ((): any => {
        const item = v as any
        if (item instanceof Uint8Array) return decodeSignedTransaction(item)
        if (typeof item === 'string') {
          try {
            return decodeSignedTransaction(fromBase64(item))
          } catch {
            return item
          }
        }
        if (typeof item === 'object' && item != null) {
          try {
            return decodeSignedTransaction(encodeMsgPack(item))
          } catch {
            return item
          }
        }
        return item
      })()
    }
  }
  return out as PendingTransactionResponse
}

// Msgpack codecs
export function encodeMsgpack(value: PendingTransactionResponse): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): PendingTransactionResponse {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: PendingTransactionResponse): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): PendingTransactionResponse {
  return fromDto(raw as PendingTransactionResponseDto)
}

// Array helpers
export function encodeMsgpackArray(values: PendingTransactionResponse[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): PendingTransactionResponse[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: PendingTransactionResponse[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): PendingTransactionResponse[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type PendingTransactionResponseMsgpackDto = {
  'asset-index'?: bigint
  'application-index'?: bigint
  'close-rewards'?: bigint
  'closing-amount'?: bigint
  'asset-closing-amount'?: bigint
  'confirmed-round'?: bigint
  'pool-error': string
  'receiver-rewards'?: bigint
  'sender-rewards'?: bigint
  'local-state-delta'?: ReturnType<(typeof AccountStateDeltaModel)['toMsgpackDto']>[]
  'global-state-delta'?: ReturnType<(typeof StateDeltaModel)['toMsgpackDto']>
  logs?: Uint8Array[]
  'inner-txns'?: ReturnType<(typeof PendingTransactionResponseModel)['toMsgpackDto']>[]
  txn: Uint8Array
}

function toMsgpackDto(value: PendingTransactionResponse): PendingTransactionResponseMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['assetId']
    if (v === undefined) {
    } else {
      out['asset-index'] = v
    }
  }
  {
    const v = (value as any)['appId']
    if (v === undefined) {
    } else {
      out['application-index'] = v
    }
  }
  {
    const v = (value as any)['closeRewards']
    if (v === undefined) {
    } else {
      out['close-rewards'] = v
    }
  }
  {
    const v = (value as any)['closingAmount']
    if (v === undefined) {
    } else {
      out['closing-amount'] = v
    }
  }
  {
    const v = (value as any)['assetClosingAmount']
    if (v === undefined) {
    } else {
      out['asset-closing-amount'] = v
    }
  }
  {
    const v = (value as any)['confirmedRound']
    if (v === undefined) {
    } else {
      out['confirmed-round'] = v
    }
  }
  {
    const v = (value as any)['poolError']
    if (v === undefined) {
    } else {
      out['pool-error'] = v
    }
  }
  {
    const v = (value as any)['receiverRewards']
    if (v === undefined) {
    } else {
      out['receiver-rewards'] = v
    }
  }
  {
    const v = (value as any)['senderRewards']
    if (v === undefined) {
    } else {
      out['sender-rewards'] = v
    }
  }
  {
    const v = (value as any)['localStateDelta']
    if (v === undefined) {
    } else {
      out['local-state-delta'] = (v as any[]).map((item) => AccountStateDeltaModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['globalStateDelta']
    if (v === undefined) {
    } else {
      out['global-state-delta'] = StateDeltaModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['logs']
    if (v === undefined) {
    } else {
      out['logs'] = v as any[]
    }
  }
  {
    const v = (value as any)['innerTxns']
    if (v === undefined) {
    } else {
      out['inner-txns'] = (v as any[]).map((item) => PendingTransactionResponseModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['txn']
    if (v === undefined) {
    } else {
      out['txn'] = encodeSignedTransaction(v as any)
    }
  }
  return out as PendingTransactionResponseMsgpackDto
}

function fromMsgpackDto(dto: PendingTransactionResponseMsgpackDto): PendingTransactionResponse {
  const out: any = {}
  {
    const v = (dto as any)['asset-index']
    if (v === undefined) {
    } else {
      out['assetId'] = v
    }
  }
  {
    const v = (dto as any)['application-index']
    if (v === undefined) {
    } else {
      out['appId'] = v
    }
  }
  {
    const v = (dto as any)['close-rewards']
    if (v === undefined) {
    } else {
      out['closeRewards'] = v
    }
  }
  {
    const v = (dto as any)['closing-amount']
    if (v === undefined) {
    } else {
      out['closingAmount'] = v
    }
  }
  {
    const v = (dto as any)['asset-closing-amount']
    if (v === undefined) {
    } else {
      out['assetClosingAmount'] = v
    }
  }
  {
    const v = (dto as any)['confirmed-round']
    if (v === undefined) {
    } else {
      out['confirmedRound'] = v
    }
  }
  {
    const v = (dto as any)['pool-error']
    if (v === undefined) {
    } else {
      out['poolError'] = v
    }
  }
  {
    const v = (dto as any)['receiver-rewards']
    if (v === undefined) {
    } else {
      out['receiverRewards'] = v
    }
  }
  {
    const v = (dto as any)['sender-rewards']
    if (v === undefined) {
    } else {
      out['senderRewards'] = v
    }
  }
  {
    const v = (dto as any)['local-state-delta']
    if (v === undefined) {
    } else {
      out['localStateDelta'] = (v as any[]).map((item) => AccountStateDeltaModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['global-state-delta']
    if (v === undefined) {
    } else {
      out['globalStateDelta'] = StateDeltaModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['logs']
    if (v === undefined) {
    } else {
      out['logs'] = v as any[]
    }
  }
  {
    const v = (dto as any)['inner-txns']
    if (v === undefined) {
    } else {
      out['innerTxns'] = (v as any[]).map((item) => PendingTransactionResponseModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['txn']
    if (v === undefined) {
    } else {
      out['txn'] = ((): any => {
        // eslint-disable-line @typescript-eslint/no-explicit-any
        const item = v as any
        if (item instanceof Uint8Array) return decodeSignedTransaction(item)
        if (typeof item === 'object' && item != null) {
          try {
            return decodeSignedTransaction(encodeMsgPack(item))
          } catch {
            return item
          }
        }
        return item
      })()
    }
  }
  return out as PendingTransactionResponse
}

export const PendingTransactionResponse = {
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
