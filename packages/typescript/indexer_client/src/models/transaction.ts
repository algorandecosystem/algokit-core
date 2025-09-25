import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AccountStateDelta, AccountStateDeltaDto } from './account-state-delta'
import { AccountStateDelta as AccountStateDeltaModel } from './account-state-delta'
import type { StateDelta, StateDeltaDto } from './state-delta'
import { StateDelta as StateDeltaModel } from './state-delta'
import type { TransactionApplication, TransactionApplicationDto } from './transaction-application'
import { TransactionApplication as TransactionApplicationModel } from './transaction-application'
import type { TransactionAssetConfig, TransactionAssetConfigDto } from './transaction-asset-config'
import { TransactionAssetConfig as TransactionAssetConfigModel } from './transaction-asset-config'
import type { TransactionAssetFreeze, TransactionAssetFreezeDto } from './transaction-asset-freeze'
import { TransactionAssetFreeze as TransactionAssetFreezeModel } from './transaction-asset-freeze'
import type { TransactionAssetTransfer, TransactionAssetTransferDto } from './transaction-asset-transfer'
import { TransactionAssetTransfer as TransactionAssetTransferModel } from './transaction-asset-transfer'
import type { TransactionHeartbeat, TransactionHeartbeatDto } from './transaction-heartbeat'
import { TransactionHeartbeat as TransactionHeartbeatModel } from './transaction-heartbeat'
import type { TransactionKeyreg, TransactionKeyregDto } from './transaction-keyreg'
import { TransactionKeyreg as TransactionKeyregModel } from './transaction-keyreg'
import type { TransactionPayment, TransactionPaymentDto } from './transaction-payment'
import { TransactionPayment as TransactionPaymentModel } from './transaction-payment'
import type { TransactionSignature, TransactionSignatureDto } from './transaction-signature'
import { TransactionSignature as TransactionSignatureModel } from './transaction-signature'
import type { TransactionStateProof, TransactionStateProofDto } from './transaction-state-proof'
import { TransactionStateProof as TransactionStateProofModel } from './transaction-state-proof'

/**
 * Contains all fields common to all transactions and serves as an envelope to all transactions type. Represents both regular and inner transactions.
 *
 * Definition:
 * data/transactions/signedtxn.go : SignedTxn
 * data/transactions/transaction.go : Transaction
 */
export type Transaction = {
  applicationTransaction?: TransactionApplication
  assetConfigTransaction?: TransactionAssetConfig
  assetFreezeTransaction?: TransactionAssetFreeze
  assetTransferTransaction?: TransactionAssetTransfer
  stateProofTransaction?: TransactionStateProof
  heartbeatTransaction?: TransactionHeartbeat

  /**
   * \[sgnr\] this is included with signed transactions when the signing address does not equal the sender. The backend can use this to ensure that auth addr is equal to the accounts auth addr.
   */
  authAddr?: string

  /**
   * \[rc\] rewards applied to close-remainder-to account.
   */
  closeRewards?: bigint

  /**
   * \[ca\] closing amount for transaction.
   */
  closingAmount?: bigint

  /**
   * Round when the transaction was confirmed.
   */
  confirmedRound?: bigint

  /**
   * Specifies an application index (ID) if an application was created with this transaction.
   */
  createdApplicationIndex?: bigint

  /**
   * Specifies an asset index (ID) if an asset was created with this transaction.
   */
  createdAssetIndex?: bigint

  /**
   * \[fee\] Transaction fee.
   */
  fee: bigint

  /**
   * \[fv\] First valid round for this transaction.
   */
  firstValid: bigint

  /**
   * \[gh\] Hash of genesis block.
   */
  genesisHash?: Uint8Array

  /**
   * \[gen\] genesis block ID.
   */
  genesisId?: string

  /**
   * \[grp\] Base64 encoded byte array of a sha512/256 digest. When present indicates that this transaction is part of a transaction group and the value is the sha512/256 hash of the transactions in that group.
   */
  group?: Uint8Array

  /**
   * Transaction ID
   */
  id?: string

  /**
   * Offset into the round where this transaction was confirmed.
   */
  intraRoundOffset?: bigint
  keyregTransaction?: TransactionKeyreg

  /**
   * \[lv\] Last valid round for this transaction.
   */
  lastValid: bigint

  /**
   * \[lx\] Base64 encoded 32-byte array. Lease enforces mutual exclusion of transactions.  If this field is nonzero, then once the transaction is confirmed, it acquires the lease identified by the (Sender, Lease) pair of the transaction until the LastValid round passes.  While this transaction possesses the lease, no other transaction specifying this lease can be confirmed.
   */
  lease?: Uint8Array

  /**
   * \[note\] Free form data.
   */
  note?: Uint8Array
  paymentTransaction?: TransactionPayment

  /**
   * \[rr\] rewards applied to receiver account.
   */
  receiverRewards?: bigint

  /**
   * \[rekey\] when included in a valid transaction, the accounts auth addr will be updated with this value and future signatures must be signed with the key represented by this address.
   */
  rekeyTo?: string

  /**
   * Time when the block this transaction is in was confirmed.
   */
  roundTime?: bigint

  /**
   * \[snd\] Sender's address.
   */
  sender: string

  /**
   * \[rs\] rewards applied to sender account.
   */
  senderRewards?: bigint
  signature?: TransactionSignature

  /**
   * \[type\] Indicates what type of transaction this is. Different types have different fields.
   *
   * Valid types, and where their fields are stored:
   * * \[pay\] payment-transaction
   * * \[keyreg\] keyreg-transaction
   * * \[acfg\] asset-config-transaction
   * * \[axfer\] asset-transfer-transaction
   * * \[afrz\] asset-freeze-transaction
   * * \[appl\] application-transaction
   * * \[stpf\] state-proof-transaction
   * * \[hb\] heartbeat-transaction
   */
  txType: 'pay' | 'keyreg' | 'acfg' | 'axfer' | 'afrz' | 'appl' | 'stpf' | 'hb'

  /**
   * \[ld\] Local state key/value changes for the application being executed by this transaction.
   */
  localStateDelta?: AccountStateDelta[]
  globalStateDelta?: StateDelta

  /**
   * \[lg\] Logs for the application being executed by this transaction.
   */
  logs?: Uint8Array[]

  /**
   * Inner transactions produced by application execution.
   */
  innerTxns?: Transaction[]
}

// JSON DTO shape for Transaction with wire keys and JSON-safe primitives
export type TransactionDto = {
  'application-transaction'?: TransactionApplicationDto
  'asset-config-transaction'?: TransactionAssetConfigDto
  'asset-freeze-transaction'?: TransactionAssetFreezeDto
  'asset-transfer-transaction'?: TransactionAssetTransferDto
  'state-proof-transaction'?: TransactionStateProofDto
  'heartbeat-transaction'?: TransactionHeartbeatDto
  'auth-addr'?: string
  'close-rewards'?: bigint
  'closing-amount'?: string
  'confirmed-round'?: string
  'created-application-index'?: bigint
  'created-asset-index'?: bigint
  fee: string
  'first-valid': bigint
  'genesis-hash'?: string
  'genesis-id'?: string
  group?: string
  id?: string
  'intra-round-offset'?: bigint
  'keyreg-transaction'?: TransactionKeyregDto
  'last-valid': bigint
  lease?: string
  note?: string
  'payment-transaction'?: TransactionPaymentDto
  'receiver-rewards'?: bigint
  'rekey-to'?: string
  'round-time'?: bigint
  sender: string
  'sender-rewards'?: bigint
  signature?: TransactionSignatureDto
  'tx-type': 'pay' | 'keyreg' | 'acfg' | 'axfer' | 'afrz' | 'appl' | 'stpf' | 'hb'
  'local-state-delta'?: AccountStateDeltaDto[]
  'global-state-delta'?: StateDeltaDto
  logs?: string[]
  'inner-txns'?: TransactionDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: Transaction): TransactionDto {
  const out: any = {}
  {
    const v = (value as any)['applicationTransaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['application-transaction'] = v === undefined ? v : TransactionApplicationModel.toDto(v)
    }
  }
  {
    const v = (value as any)['assetConfigTransaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-config-transaction'] = v === undefined ? v : TransactionAssetConfigModel.toDto(v)
    }
  }
  {
    const v = (value as any)['assetFreezeTransaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-freeze-transaction'] = v === undefined ? v : TransactionAssetFreezeModel.toDto(v)
    }
  }
  {
    const v = (value as any)['assetTransferTransaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-transfer-transaction'] = v === undefined ? v : TransactionAssetTransferModel.toDto(v)
    }
  }
  {
    const v = (value as any)['stateProofTransaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['state-proof-transaction'] = v === undefined ? v : TransactionStateProofModel.toDto(v)
    }
  }
  {
    const v = (value as any)['heartbeatTransaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['heartbeat-transaction'] = v === undefined ? v : TransactionHeartbeatModel.toDto(v)
    }
  }
  {
    const v = (value as any)['authAddr']
    if (v === undefined) {
      // omit undefined
    } else {
      out['auth-addr'] = v
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
    const v = (value as any)['confirmedRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['confirmed-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['createdApplicationIndex']
    if (v === undefined) {
      // omit undefined
    } else {
      out['created-application-index'] = v
    }
  }
  {
    const v = (value as any)['createdAssetIndex']
    if (v === undefined) {
      // omit undefined
    } else {
      out['created-asset-index'] = v
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
    const v = (value as any)['firstValid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['first-valid'] = v
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
    const v = (value as any)['group']
    if (v === undefined) {
      // omit undefined
    } else {
      out['group'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['id'] = v
    }
  }
  {
    const v = (value as any)['intraRoundOffset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['intra-round-offset'] = v
    }
  }
  {
    const v = (value as any)['keyregTransaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['keyreg-transaction'] = v === undefined ? v : TransactionKeyregModel.toDto(v)
    }
  }
  {
    const v = (value as any)['lastValid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-valid'] = v
    }
  }
  {
    const v = (value as any)['lease']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lease'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['note']
    if (v === undefined) {
      // omit undefined
    } else {
      out['note'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['paymentTransaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['payment-transaction'] = v === undefined ? v : TransactionPaymentModel.toDto(v)
    }
  }
  {
    const v = (value as any)['receiverRewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['receiver-rewards'] = v
    }
  }
  {
    const v = (value as any)['rekeyTo']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rekey-to'] = v
    }
  }
  {
    const v = (value as any)['roundTime']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round-time'] = v
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
  {
    const v = (value as any)['senderRewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sender-rewards'] = v
    }
  }
  {
    const v = (value as any)['signature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['signature'] = v === undefined ? v : TransactionSignatureModel.toDto(v)
    }
  }
  {
    const v = (value as any)['txType']
    if (v === undefined) {
      // omit undefined
    } else {
      out['tx-type'] = v
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
      out['inner-txns'] = (v as any[]).map((item) => TransactionModel.toDto(item))
    }
  }
  return out as TransactionDto
}

export function fromDto(dto: TransactionDto): Transaction {
  const out: any = {}
  {
    const v = (dto as any)['application-transaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['applicationTransaction'] = v === undefined ? v : TransactionApplicationModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['asset-config-transaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetConfigTransaction'] = v === undefined ? v : TransactionAssetConfigModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['asset-freeze-transaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetFreezeTransaction'] = v === undefined ? v : TransactionAssetFreezeModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['asset-transfer-transaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetTransferTransaction'] = v === undefined ? v : TransactionAssetTransferModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['state-proof-transaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['stateProofTransaction'] = v === undefined ? v : TransactionStateProofModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['heartbeat-transaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['heartbeatTransaction'] = v === undefined ? v : TransactionHeartbeatModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['auth-addr']
    if (v === undefined) {
      // omit undefined
    } else {
      out['authAddr'] = v as any
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
    const v = (dto as any)['confirmed-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['confirmedRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['created-application-index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['createdApplicationIndex'] = v as any
    }
  }
  {
    const v = (dto as any)['created-asset-index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['createdAssetIndex'] = v as any
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
    const v = (dto as any)['first-valid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['firstValid'] = v as any
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
    const v = (dto as any)['group']
    if (v === undefined) {
      // omit undefined
    } else {
      out['group'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['id'] = v as any
    }
  }
  {
    const v = (dto as any)['intra-round-offset']
    if (v === undefined) {
      // omit undefined
    } else {
      out['intraRoundOffset'] = v as any
    }
  }
  {
    const v = (dto as any)['keyreg-transaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['keyregTransaction'] = v === undefined ? v : TransactionKeyregModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['last-valid']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastValid'] = v as any
    }
  }
  {
    const v = (dto as any)['lease']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lease'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['note']
    if (v === undefined) {
      // omit undefined
    } else {
      out['note'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['payment-transaction']
    if (v === undefined) {
      // omit undefined
    } else {
      out['paymentTransaction'] = v === undefined ? v : TransactionPaymentModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['receiver-rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['receiverRewards'] = v as any
    }
  }
  {
    const v = (dto as any)['rekey-to']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rekeyTo'] = v as any
    }
  }
  {
    const v = (dto as any)['round-time']
    if (v === undefined) {
      // omit undefined
    } else {
      out['roundTime'] = v as any
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
  {
    const v = (dto as any)['sender-rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['senderRewards'] = v as any
    }
  }
  {
    const v = (dto as any)['signature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['signature'] = v === undefined ? v : TransactionSignatureModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['tx-type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txType'] = v as any
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
      out['innerTxns'] = (v as any[]).map((item) => TransactionModel.fromDto(item))
    }
  }
  return out as Transaction
}

// Msgpack codecs
export function encodeMsgpack(value: Transaction): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): Transaction {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: Transaction): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): Transaction {
  return fromDto(raw as TransactionDto)
}

// Array helpers
export function encodeMsgpackArray(values: Transaction[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): Transaction[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: Transaction[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): Transaction[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionMsgpackDto = {
  'application-transaction'?: ReturnType<(typeof TransactionApplicationModel)['toMsgpackDto']>
  'asset-config-transaction'?: ReturnType<(typeof TransactionAssetConfigModel)['toMsgpackDto']>
  'asset-freeze-transaction'?: ReturnType<(typeof TransactionAssetFreezeModel)['toMsgpackDto']>
  'asset-transfer-transaction'?: ReturnType<(typeof TransactionAssetTransferModel)['toMsgpackDto']>
  'state-proof-transaction'?: ReturnType<(typeof TransactionStateProofModel)['toMsgpackDto']>
  'heartbeat-transaction'?: ReturnType<(typeof TransactionHeartbeatModel)['toMsgpackDto']>
  'auth-addr'?: string
  'close-rewards'?: bigint
  'closing-amount'?: bigint
  'confirmed-round'?: bigint
  'created-application-index'?: bigint
  'created-asset-index'?: bigint
  fee: bigint
  'first-valid': bigint
  'genesis-hash'?: Uint8Array
  'genesis-id'?: string
  group?: Uint8Array
  id?: string
  'intra-round-offset'?: bigint
  'keyreg-transaction'?: ReturnType<(typeof TransactionKeyregModel)['toMsgpackDto']>
  'last-valid': bigint
  lease?: Uint8Array
  note?: Uint8Array
  'payment-transaction'?: ReturnType<(typeof TransactionPaymentModel)['toMsgpackDto']>
  'receiver-rewards'?: bigint
  'rekey-to'?: string
  'round-time'?: bigint
  sender: string
  'sender-rewards'?: bigint
  signature?: ReturnType<(typeof TransactionSignatureModel)['toMsgpackDto']>
  'tx-type': 'pay' | 'keyreg' | 'acfg' | 'axfer' | 'afrz' | 'appl' | 'stpf' | 'hb'
  'local-state-delta'?: ReturnType<(typeof AccountStateDeltaModel)['toMsgpackDto']>[]
  'global-state-delta'?: ReturnType<(typeof StateDeltaModel)['toMsgpackDto']>
  logs?: Uint8Array[][]
  'inner-txns'?: ReturnType<(typeof TransactionModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: Transaction): TransactionMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['applicationTransaction']
    if (v === undefined) {
    } else {
      out['application-transaction'] = TransactionApplicationModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['assetConfigTransaction']
    if (v === undefined) {
    } else {
      out['asset-config-transaction'] = TransactionAssetConfigModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['assetFreezeTransaction']
    if (v === undefined) {
    } else {
      out['asset-freeze-transaction'] = TransactionAssetFreezeModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['assetTransferTransaction']
    if (v === undefined) {
    } else {
      out['asset-transfer-transaction'] = TransactionAssetTransferModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['stateProofTransaction']
    if (v === undefined) {
    } else {
      out['state-proof-transaction'] = TransactionStateProofModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['heartbeatTransaction']
    if (v === undefined) {
    } else {
      out['heartbeat-transaction'] = TransactionHeartbeatModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['authAddr']
    if (v === undefined) {
    } else {
      out['auth-addr'] = v
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
    const v = (value as any)['confirmedRound']
    if (v === undefined) {
    } else {
      out['confirmed-round'] = v
    }
  }
  {
    const v = (value as any)['createdApplicationIndex']
    if (v === undefined) {
    } else {
      out['created-application-index'] = v
    }
  }
  {
    const v = (value as any)['createdAssetIndex']
    if (v === undefined) {
    } else {
      out['created-asset-index'] = v
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
    const v = (value as any)['firstValid']
    if (v === undefined) {
    } else {
      out['first-valid'] = v
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
    const v = (value as any)['group']
    if (v === undefined) {
    } else {
      out['group'] = v
    }
  }
  {
    const v = (value as any)['id']
    if (v === undefined) {
    } else {
      out['id'] = v
    }
  }
  {
    const v = (value as any)['intraRoundOffset']
    if (v === undefined) {
    } else {
      out['intra-round-offset'] = v
    }
  }
  {
    const v = (value as any)['keyregTransaction']
    if (v === undefined) {
    } else {
      out['keyreg-transaction'] = TransactionKeyregModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['lastValid']
    if (v === undefined) {
    } else {
      out['last-valid'] = v
    }
  }
  {
    const v = (value as any)['lease']
    if (v === undefined) {
    } else {
      out['lease'] = v
    }
  }
  {
    const v = (value as any)['note']
    if (v === undefined) {
    } else {
      out['note'] = v
    }
  }
  {
    const v = (value as any)['paymentTransaction']
    if (v === undefined) {
    } else {
      out['payment-transaction'] = TransactionPaymentModel.toMsgpackDto(v)
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
    const v = (value as any)['rekeyTo']
    if (v === undefined) {
    } else {
      out['rekey-to'] = v
    }
  }
  {
    const v = (value as any)['roundTime']
    if (v === undefined) {
    } else {
      out['round-time'] = v
    }
  }
  {
    const v = (value as any)['sender']
    if (v === undefined) {
    } else {
      out['sender'] = v
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
    const v = (value as any)['signature']
    if (v === undefined) {
    } else {
      out['signature'] = TransactionSignatureModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['txType']
    if (v === undefined) {
    } else {
      out['tx-type'] = v
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
      out['inner-txns'] = (v as any[]).map((item) => TransactionModel.toMsgpackDto(item))
    }
  }
  return out as TransactionMsgpackDto
}

function fromMsgpackDto(dto: TransactionMsgpackDto): Transaction {
  const out: any = {}
  {
    const v = (dto as any)['application-transaction']
    if (v === undefined) {
    } else {
      out['applicationTransaction'] = TransactionApplicationModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['asset-config-transaction']
    if (v === undefined) {
    } else {
      out['assetConfigTransaction'] = TransactionAssetConfigModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['asset-freeze-transaction']
    if (v === undefined) {
    } else {
      out['assetFreezeTransaction'] = TransactionAssetFreezeModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['asset-transfer-transaction']
    if (v === undefined) {
    } else {
      out['assetTransferTransaction'] = TransactionAssetTransferModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['state-proof-transaction']
    if (v === undefined) {
    } else {
      out['stateProofTransaction'] = TransactionStateProofModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['heartbeat-transaction']
    if (v === undefined) {
    } else {
      out['heartbeatTransaction'] = TransactionHeartbeatModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['auth-addr']
    if (v === undefined) {
    } else {
      out['authAddr'] = v
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
    const v = (dto as any)['confirmed-round']
    if (v === undefined) {
    } else {
      out['confirmedRound'] = v
    }
  }
  {
    const v = (dto as any)['created-application-index']
    if (v === undefined) {
    } else {
      out['createdApplicationIndex'] = v
    }
  }
  {
    const v = (dto as any)['created-asset-index']
    if (v === undefined) {
    } else {
      out['createdAssetIndex'] = v
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
    const v = (dto as any)['first-valid']
    if (v === undefined) {
    } else {
      out['firstValid'] = v
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
    const v = (dto as any)['group']
    if (v === undefined) {
    } else {
      out['group'] = v
    }
  }
  {
    const v = (dto as any)['id']
    if (v === undefined) {
    } else {
      out['id'] = v
    }
  }
  {
    const v = (dto as any)['intra-round-offset']
    if (v === undefined) {
    } else {
      out['intraRoundOffset'] = v
    }
  }
  {
    const v = (dto as any)['keyreg-transaction']
    if (v === undefined) {
    } else {
      out['keyregTransaction'] = TransactionKeyregModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['last-valid']
    if (v === undefined) {
    } else {
      out['lastValid'] = v
    }
  }
  {
    const v = (dto as any)['lease']
    if (v === undefined) {
    } else {
      out['lease'] = v
    }
  }
  {
    const v = (dto as any)['note']
    if (v === undefined) {
    } else {
      out['note'] = v
    }
  }
  {
    const v = (dto as any)['payment-transaction']
    if (v === undefined) {
    } else {
      out['paymentTransaction'] = TransactionPaymentModel.fromMsgpackDto(v)
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
    const v = (dto as any)['rekey-to']
    if (v === undefined) {
    } else {
      out['rekeyTo'] = v
    }
  }
  {
    const v = (dto as any)['round-time']
    if (v === undefined) {
    } else {
      out['roundTime'] = v
    }
  }
  {
    const v = (dto as any)['sender']
    if (v === undefined) {
    } else {
      out['sender'] = v
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
    const v = (dto as any)['signature']
    if (v === undefined) {
    } else {
      out['signature'] = TransactionSignatureModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['tx-type']
    if (v === undefined) {
    } else {
      out['txType'] = v
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
      out['innerTxns'] = (v as any[]).map((item) => TransactionModel.fromMsgpackDto(item))
    }
  }
  return out as Transaction
}

export const Transaction = {
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
