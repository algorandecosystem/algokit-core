import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { AccountParticipation, AccountParticipationDto } from './account-participation'
import { AccountParticipation as AccountParticipationModel } from './account-participation'
import type { Application, ApplicationDto } from './application'
import { Application as ApplicationModel } from './application'
import type { ApplicationLocalState, ApplicationLocalStateDto } from './application-local-state'
import { ApplicationLocalState as ApplicationLocalStateModel } from './application-local-state'
import type { ApplicationStateSchema, ApplicationStateSchemaDto } from './application-state-schema'
import { ApplicationStateSchema as ApplicationStateSchemaModel } from './application-state-schema'
import type { Asset, AssetDto } from './asset'
import { Asset as AssetModel } from './asset'
import type { AssetHolding, AssetHoldingDto } from './asset-holding'
import { AssetHolding as AssetHoldingModel } from './asset-holding'

/**
 * Account information at a given round.
 *
 * Definition:
 * data/basics/userBalance.go : AccountData
 */
export type Account = {
  /**
   * the account public key
   */
  address: string

  /**
   * total number of MicroAlgos in the account
   */
  amount: bigint

  /**
   * MicroAlgo balance required by the account.
   *
   * The requirement grows based on asset and application usage.
   */
  minBalance: bigint

  /**
   * specifies the amount of MicroAlgos in the account, without the pending rewards.
   */
  amountWithoutPendingRewards: bigint

  /**
   * application local data stored in this account.
   *
   * Note the raw object uses `map[int] -> AppLocalState` for this type.
   */
  appsLocalState?: ApplicationLocalState[]
  appsTotalSchema?: ApplicationStateSchema

  /**
   * the sum of all extra application program pages for this account.
   */
  appsTotalExtraPages?: bigint

  /**
   * assets held by this account.
   *
   * Note the raw object uses `map[int] -> AssetHolding` for this type.
   */
  assets?: AssetHolding[]

  /**
   * parameters of applications created by this account including app global data.
   *
   * Note: the raw account uses `map[int] -> AppParams` for this type.
   */
  createdApps?: Application[]

  /**
   * parameters of assets created by this account.
   *
   * Note: the raw account uses `map[int] -> Asset` for this type.
   */
  createdAssets?: Asset[]
  participation?: AccountParticipation

  /**
   * can the account receive block incentives if its balance is in range at proposal time.
   */
  incentiveEligible?: boolean

  /**
   * amount of MicroAlgos of pending rewards in this account.
   */
  pendingRewards: bigint

  /**
   * used as part of the rewards computation. Only applicable to accounts which are participating.
   */
  rewardBase?: bigint

  /**
   * total rewards of MicroAlgos the account has received, including pending rewards.
   */
  rewards: bigint

  /**
   * The round for which this information is relevant.
   */
  round: bigint

  /**
   * voting status of the account's MicroAlgos
   * * Offline - indicates that the associated account is delegated.
   * *  Online  - indicates that the associated account used as part of the delegation pool.
   * *   NotParticipating - indicates that the associated account is neither a delegator nor a delegate.
   */
  status: string

  /**
   * the type of signature used by this account, must be one of:
   * * sig
   * * msig
   * * lsig
   * * or null if unknown
   */
  sigType?: 'sig' | 'msig' | 'lsig'

  /**
   * The count of all applications that have been opted in, equivalent to the count of application local data (AppLocalState objects) stored in this account.
   */
  totalAppsOptedIn: bigint

  /**
   * The count of all assets that have been opted in, equivalent to the count of AssetHolding objects held by this account.
   */
  totalAssetsOptedIn: bigint

  /**
   * For app-accounts only. The total number of bytes allocated for the keys and values of boxes which belong to the associated application.
   */
  totalBoxBytes: bigint

  /**
   * For app-accounts only. The total number of boxes which belong to the associated application.
   */
  totalBoxes: bigint

  /**
   * The count of all apps (AppParams objects) created by this account.
   */
  totalCreatedApps: bigint

  /**
   * The count of all assets (AssetParams objects) created by this account.
   */
  totalCreatedAssets: bigint

  /**
   * The address against which signing should be checked. If empty, the address of the current account is used. This field can be updated in any transaction by setting the RekeyTo field.
   */
  authAddr?: string

  /**
   * The round in which this account last proposed the block.
   */
  lastProposed?: bigint

  /**
   * The round in which this account last went online, or explicitly renewed their online status.
   */
  lastHeartbeat?: bigint

  /**
   * Whether or not this account is currently closed.
   */
  deleted?: boolean

  /**
   * Round during which this account first appeared in a transaction.
   */
  createdAtRound?: bigint

  /**
   * Round during which this account was most recently closed.
   */
  closedAtRound?: bigint
}

// JSON DTO shape for Account with wire keys and JSON-safe primitives
export type AccountDto = {
  address: string
  amount: string
  'min-balance': string
  'amount-without-pending-rewards': string
  'apps-local-state'?: ApplicationLocalStateDto[]
  'apps-total-schema'?: ApplicationStateSchemaDto
  'apps-total-extra-pages'?: bigint
  assets?: AssetHoldingDto[]
  'created-apps'?: ApplicationDto[]
  'created-assets'?: AssetDto[]
  participation?: AccountParticipationDto
  'incentive-eligible'?: boolean
  'pending-rewards': string
  'reward-base'?: string
  rewards: string
  round: string
  status: string
  'sig-type'?: 'sig' | 'msig' | 'lsig'
  'total-apps-opted-in': bigint
  'total-assets-opted-in': bigint
  'total-box-bytes': bigint
  'total-boxes': bigint
  'total-created-apps': bigint
  'total-created-assets': bigint
  'auth-addr'?: string
  'last-proposed'?: string
  'last-heartbeat'?: string
  deleted?: boolean
  'created-at-round'?: string
  'closed-at-round'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: Account): AccountDto {
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
    const v = (value as any)['amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['amount'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['minBalance']
    if (v === undefined) {
      // omit undefined
    } else {
      out['min-balance'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['amountWithoutPendingRewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['amount-without-pending-rewards'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['appsLocalState']
    if (v === undefined) {
      // omit undefined
    } else {
      out['apps-local-state'] = (v as any[]).map((item) => ApplicationLocalStateModel.toDto(item))
    }
  }
  {
    const v = (value as any)['appsTotalSchema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['apps-total-schema'] = v === undefined ? v : ApplicationStateSchemaModel.toDto(v)
    }
  }
  {
    const v = (value as any)['appsTotalExtraPages']
    if (v === undefined) {
      // omit undefined
    } else {
      out['apps-total-extra-pages'] = v
    }
  }
  {
    const v = (value as any)['assets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assets'] = (v as any[]).map((item) => AssetHoldingModel.toDto(item))
    }
  }
  {
    const v = (value as any)['createdApps']
    if (v === undefined) {
      // omit undefined
    } else {
      out['created-apps'] = (v as any[]).map((item) => ApplicationModel.toDto(item))
    }
  }
  {
    const v = (value as any)['createdAssets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['created-assets'] = (v as any[]).map((item) => AssetModel.toDto(item))
    }
  }
  {
    const v = (value as any)['participation']
    if (v === undefined) {
      // omit undefined
    } else {
      out['participation'] = v === undefined ? v : AccountParticipationModel.toDto(v)
    }
  }
  {
    const v = (value as any)['incentiveEligible']
    if (v === undefined) {
      // omit undefined
    } else {
      out['incentive-eligible'] = v
    }
  }
  {
    const v = (value as any)['pendingRewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['pending-rewards'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['rewardBase']
    if (v === undefined) {
      // omit undefined
    } else {
      out['reward-base'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewards'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['status']
    if (v === undefined) {
      // omit undefined
    } else {
      out['status'] = v
    }
  }
  {
    const v = (value as any)['sigType']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sig-type'] = v
    }
  }
  {
    const v = (value as any)['totalAppsOptedIn']
    if (v === undefined) {
      // omit undefined
    } else {
      out['total-apps-opted-in'] = v
    }
  }
  {
    const v = (value as any)['totalAssetsOptedIn']
    if (v === undefined) {
      // omit undefined
    } else {
      out['total-assets-opted-in'] = v
    }
  }
  {
    const v = (value as any)['totalBoxBytes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['total-box-bytes'] = v
    }
  }
  {
    const v = (value as any)['totalBoxes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['total-boxes'] = v
    }
  }
  {
    const v = (value as any)['totalCreatedApps']
    if (v === undefined) {
      // omit undefined
    } else {
      out['total-created-apps'] = v
    }
  }
  {
    const v = (value as any)['totalCreatedAssets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['total-created-assets'] = v
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
    const v = (value as any)['lastProposed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-proposed'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['lastHeartbeat']
    if (v === undefined) {
      // omit undefined
    } else {
      out['last-heartbeat'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['deleted']
    if (v === undefined) {
      // omit undefined
    } else {
      out['deleted'] = v
    }
  }
  {
    const v = (value as any)['createdAtRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['created-at-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['closedAtRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closed-at-round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as AccountDto
}

export function fromDto(dto: AccountDto): Account {
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
    const v = (dto as any)['amount']
    if (v === undefined) {
      // omit undefined
    } else {
      out['amount'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['min-balance']
    if (v === undefined) {
      // omit undefined
    } else {
      out['minBalance'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['amount-without-pending-rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['amountWithoutPendingRewards'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['apps-local-state']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appsLocalState'] = (v as any[]).map((item) => ApplicationLocalStateModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['apps-total-schema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appsTotalSchema'] = v === undefined ? v : ApplicationStateSchemaModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['apps-total-extra-pages']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appsTotalExtraPages'] = v as any
    }
  }
  {
    const v = (dto as any)['assets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assets'] = (v as any[]).map((item) => AssetHoldingModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['created-apps']
    if (v === undefined) {
      // omit undefined
    } else {
      out['createdApps'] = (v as any[]).map((item) => ApplicationModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['created-assets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['createdAssets'] = (v as any[]).map((item) => AssetModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['participation']
    if (v === undefined) {
      // omit undefined
    } else {
      out['participation'] = v === undefined ? v : AccountParticipationModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['incentive-eligible']
    if (v === undefined) {
      // omit undefined
    } else {
      out['incentiveEligible'] = v as any
    }
  }
  {
    const v = (dto as any)['pending-rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['pendingRewards'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['reward-base']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewardBase'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['rewards']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewards'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['status']
    if (v === undefined) {
      // omit undefined
    } else {
      out['status'] = v as any
    }
  }
  {
    const v = (dto as any)['sig-type']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sigType'] = v as any
    }
  }
  {
    const v = (dto as any)['total-apps-opted-in']
    if (v === undefined) {
      // omit undefined
    } else {
      out['totalAppsOptedIn'] = v as any
    }
  }
  {
    const v = (dto as any)['total-assets-opted-in']
    if (v === undefined) {
      // omit undefined
    } else {
      out['totalAssetsOptedIn'] = v as any
    }
  }
  {
    const v = (dto as any)['total-box-bytes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['totalBoxBytes'] = v as any
    }
  }
  {
    const v = (dto as any)['total-boxes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['totalBoxes'] = v as any
    }
  }
  {
    const v = (dto as any)['total-created-apps']
    if (v === undefined) {
      // omit undefined
    } else {
      out['totalCreatedApps'] = v as any
    }
  }
  {
    const v = (dto as any)['total-created-assets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['totalCreatedAssets'] = v as any
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
    const v = (dto as any)['last-proposed']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastProposed'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['last-heartbeat']
    if (v === undefined) {
      // omit undefined
    } else {
      out['lastHeartbeat'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['deleted']
    if (v === undefined) {
      // omit undefined
    } else {
      out['deleted'] = v as any
    }
  }
  {
    const v = (dto as any)['created-at-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['createdAtRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['closed-at-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['closedAtRound'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as Account
}

// Msgpack codecs
export function encodeMsgpack(value: Account): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): Account {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: Account): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): Account {
  return fromDto(raw as AccountDto)
}

// Array helpers
export function encodeMsgpackArray(values: Account[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): Account[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: Account[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): Account[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AccountMsgpackDto = {
  address: string
  amount: bigint
  'min-balance': bigint
  'amount-without-pending-rewards': bigint
  'apps-local-state'?: ReturnType<(typeof ApplicationLocalStateModel)['toMsgpackDto']>[]
  'apps-total-schema'?: ReturnType<(typeof ApplicationStateSchemaModel)['toMsgpackDto']>
  'apps-total-extra-pages'?: bigint
  assets?: ReturnType<(typeof AssetHoldingModel)['toMsgpackDto']>[]
  'created-apps'?: ReturnType<(typeof ApplicationModel)['toMsgpackDto']>[]
  'created-assets'?: ReturnType<(typeof AssetModel)['toMsgpackDto']>[]
  participation?: ReturnType<(typeof AccountParticipationModel)['toMsgpackDto']>
  'incentive-eligible'?: boolean
  'pending-rewards': bigint
  'reward-base'?: bigint
  rewards: bigint
  round: bigint
  status: string
  'sig-type'?: 'sig' | 'msig' | 'lsig'
  'total-apps-opted-in': bigint
  'total-assets-opted-in': bigint
  'total-box-bytes': bigint
  'total-boxes': bigint
  'total-created-apps': bigint
  'total-created-assets': bigint
  'auth-addr'?: string
  'last-proposed'?: bigint
  'last-heartbeat'?: bigint
  deleted?: boolean
  'created-at-round'?: bigint
  'closed-at-round'?: bigint
}

function toMsgpackDto(value: Account): AccountMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['amount']
    if (v === undefined) {
    } else {
      out['amount'] = v
    }
  }
  {
    const v = (value as any)['minBalance']
    if (v === undefined) {
    } else {
      out['min-balance'] = v
    }
  }
  {
    const v = (value as any)['amountWithoutPendingRewards']
    if (v === undefined) {
    } else {
      out['amount-without-pending-rewards'] = v
    }
  }
  {
    const v = (value as any)['appsLocalState']
    if (v === undefined) {
    } else {
      out['apps-local-state'] = (v as any[]).map((item) => ApplicationLocalStateModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['appsTotalSchema']
    if (v === undefined) {
    } else {
      out['apps-total-schema'] = ApplicationStateSchemaModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['appsTotalExtraPages']
    if (v === undefined) {
    } else {
      out['apps-total-extra-pages'] = v
    }
  }
  {
    const v = (value as any)['assets']
    if (v === undefined) {
    } else {
      out['assets'] = (v as any[]).map((item) => AssetHoldingModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['createdApps']
    if (v === undefined) {
    } else {
      out['created-apps'] = (v as any[]).map((item) => ApplicationModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['createdAssets']
    if (v === undefined) {
    } else {
      out['created-assets'] = (v as any[]).map((item) => AssetModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['participation']
    if (v === undefined) {
    } else {
      out['participation'] = AccountParticipationModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['incentiveEligible']
    if (v === undefined) {
    } else {
      out['incentive-eligible'] = v
    }
  }
  {
    const v = (value as any)['pendingRewards']
    if (v === undefined) {
    } else {
      out['pending-rewards'] = v
    }
  }
  {
    const v = (value as any)['rewardBase']
    if (v === undefined) {
    } else {
      out['reward-base'] = v
    }
  }
  {
    const v = (value as any)['rewards']
    if (v === undefined) {
    } else {
      out['rewards'] = v
    }
  }
  {
    const v = (value as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (value as any)['status']
    if (v === undefined) {
    } else {
      out['status'] = v
    }
  }
  {
    const v = (value as any)['sigType']
    if (v === undefined) {
    } else {
      out['sig-type'] = v
    }
  }
  {
    const v = (value as any)['totalAppsOptedIn']
    if (v === undefined) {
    } else {
      out['total-apps-opted-in'] = v
    }
  }
  {
    const v = (value as any)['totalAssetsOptedIn']
    if (v === undefined) {
    } else {
      out['total-assets-opted-in'] = v
    }
  }
  {
    const v = (value as any)['totalBoxBytes']
    if (v === undefined) {
    } else {
      out['total-box-bytes'] = v
    }
  }
  {
    const v = (value as any)['totalBoxes']
    if (v === undefined) {
    } else {
      out['total-boxes'] = v
    }
  }
  {
    const v = (value as any)['totalCreatedApps']
    if (v === undefined) {
    } else {
      out['total-created-apps'] = v
    }
  }
  {
    const v = (value as any)['totalCreatedAssets']
    if (v === undefined) {
    } else {
      out['total-created-assets'] = v
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
    const v = (value as any)['lastProposed']
    if (v === undefined) {
    } else {
      out['last-proposed'] = v
    }
  }
  {
    const v = (value as any)['lastHeartbeat']
    if (v === undefined) {
    } else {
      out['last-heartbeat'] = v
    }
  }
  {
    const v = (value as any)['deleted']
    if (v === undefined) {
    } else {
      out['deleted'] = v
    }
  }
  {
    const v = (value as any)['createdAtRound']
    if (v === undefined) {
    } else {
      out['created-at-round'] = v
    }
  }
  {
    const v = (value as any)['closedAtRound']
    if (v === undefined) {
    } else {
      out['closed-at-round'] = v
    }
  }
  return out as AccountMsgpackDto
}

function fromMsgpackDto(dto: AccountMsgpackDto): Account {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (dto as any)['amount']
    if (v === undefined) {
    } else {
      out['amount'] = v
    }
  }
  {
    const v = (dto as any)['min-balance']
    if (v === undefined) {
    } else {
      out['minBalance'] = v
    }
  }
  {
    const v = (dto as any)['amount-without-pending-rewards']
    if (v === undefined) {
    } else {
      out['amountWithoutPendingRewards'] = v
    }
  }
  {
    const v = (dto as any)['apps-local-state']
    if (v === undefined) {
    } else {
      out['appsLocalState'] = (v as any[]).map((item) => ApplicationLocalStateModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['apps-total-schema']
    if (v === undefined) {
    } else {
      out['appsTotalSchema'] = ApplicationStateSchemaModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['apps-total-extra-pages']
    if (v === undefined) {
    } else {
      out['appsTotalExtraPages'] = v
    }
  }
  {
    const v = (dto as any)['assets']
    if (v === undefined) {
    } else {
      out['assets'] = (v as any[]).map((item) => AssetHoldingModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['created-apps']
    if (v === undefined) {
    } else {
      out['createdApps'] = (v as any[]).map((item) => ApplicationModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['created-assets']
    if (v === undefined) {
    } else {
      out['createdAssets'] = (v as any[]).map((item) => AssetModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['participation']
    if (v === undefined) {
    } else {
      out['participation'] = AccountParticipationModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['incentive-eligible']
    if (v === undefined) {
    } else {
      out['incentiveEligible'] = v
    }
  }
  {
    const v = (dto as any)['pending-rewards']
    if (v === undefined) {
    } else {
      out['pendingRewards'] = v
    }
  }
  {
    const v = (dto as any)['reward-base']
    if (v === undefined) {
    } else {
      out['rewardBase'] = v
    }
  }
  {
    const v = (dto as any)['rewards']
    if (v === undefined) {
    } else {
      out['rewards'] = v
    }
  }
  {
    const v = (dto as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (dto as any)['status']
    if (v === undefined) {
    } else {
      out['status'] = v
    }
  }
  {
    const v = (dto as any)['sig-type']
    if (v === undefined) {
    } else {
      out['sigType'] = v
    }
  }
  {
    const v = (dto as any)['total-apps-opted-in']
    if (v === undefined) {
    } else {
      out['totalAppsOptedIn'] = v
    }
  }
  {
    const v = (dto as any)['total-assets-opted-in']
    if (v === undefined) {
    } else {
      out['totalAssetsOptedIn'] = v
    }
  }
  {
    const v = (dto as any)['total-box-bytes']
    if (v === undefined) {
    } else {
      out['totalBoxBytes'] = v
    }
  }
  {
    const v = (dto as any)['total-boxes']
    if (v === undefined) {
    } else {
      out['totalBoxes'] = v
    }
  }
  {
    const v = (dto as any)['total-created-apps']
    if (v === undefined) {
    } else {
      out['totalCreatedApps'] = v
    }
  }
  {
    const v = (dto as any)['total-created-assets']
    if (v === undefined) {
    } else {
      out['totalCreatedAssets'] = v
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
    const v = (dto as any)['last-proposed']
    if (v === undefined) {
    } else {
      out['lastProposed'] = v
    }
  }
  {
    const v = (dto as any)['last-heartbeat']
    if (v === undefined) {
    } else {
      out['lastHeartbeat'] = v
    }
  }
  {
    const v = (dto as any)['deleted']
    if (v === undefined) {
    } else {
      out['deleted'] = v
    }
  }
  {
    const v = (dto as any)['created-at-round']
    if (v === undefined) {
    } else {
      out['createdAtRound'] = v
    }
  }
  {
    const v = (dto as any)['closed-at-round']
    if (v === undefined) {
    } else {
      out['closedAtRound'] = v
    }
  }
  return out as Account
}

export const Account = {
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
