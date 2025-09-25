import { encodeSignedTransaction, encodeSignedTransactions, decodeSignedTransaction } from '@algorandfoundation/algokit-transact'
import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { Account, AccountDto } from './account'
import { Account as AccountModel } from './account'
import type { Application, ApplicationDto } from './application'
import { Application as ApplicationModel } from './application'
import type { DryrunSource, DryrunSourceDto } from './dryrun-source'
import { DryrunSource as DryrunSourceModel } from './dryrun-source'

/**
 * Request data type for dryrun endpoint. Given the Transactions and simulated ledger state upload, run TEAL scripts and return debugging information.
 */
export type DryrunRequest = {
  txns: SignedTransaction[]
  accounts: Account[]
  apps: Application[]

  /**
   * ProtocolVersion specifies a specific version string to operate under, otherwise whatever the current protocol of the network this algod is running in.
   */
  protocolVersion: string

  /**
   * Round is available to some TEAL scripts. Defaults to the current round on the network this algod is attached to.
   */
  round: bigint

  /**
   * LatestTimestamp is available to some TEAL scripts. Defaults to the latest confirmed timestamp this algod is attached to.
   */
  latestTimestamp: bigint
  sources: DryrunSource[]
}

// JSON DTO shape for DryrunRequest with wire keys and JSON-safe primitives
export type DryrunRequestDto = {
  txns: unknown[]
  accounts: AccountDto[]
  apps: ApplicationDto[]
  'protocol-version': string
  round: string
  'latest-timestamp': bigint
  sources: DryrunSourceDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: DryrunRequest): DryrunRequestDto {
  const out: any = {}
  {
    const v = (value as any)['txns']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txns'] = (v as any[]).map((item) => {
        if (item && typeof item === 'object' && 'txn' in (item as any)) return item as any
        return toBase64(encodeSignedTransaction(item as any))
      })
    }
  }
  {
    const v = (value as any)['accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['accounts'] = (v as any[]).map((item) => AccountModel.toDto(item))
    }
  }
  {
    const v = (value as any)['apps']
    if (v === undefined) {
      // omit undefined
    } else {
      out['apps'] = (v as any[]).map((item) => ApplicationModel.toDto(item))
    }
  }
  {
    const v = (value as any)['protocolVersion']
    if (v === undefined) {
      // omit undefined
    } else {
      out['protocol-version'] = v
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
    const v = (value as any)['latestTimestamp']
    if (v === undefined) {
      // omit undefined
    } else {
      out['latest-timestamp'] = v
    }
  }
  {
    const v = (value as any)['sources']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sources'] = (v as any[]).map((item) => DryrunSourceModel.toDto(item))
    }
  }
  return out as DryrunRequestDto
}

export function fromDto(dto: DryrunRequestDto): DryrunRequest {
  const out: any = {}
  {
    const v = (dto as any)['txns']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txns'] = (v as any[]).map((item) => {
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
      })
    }
  }
  {
    const v = (dto as any)['accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['accounts'] = (v as any[]).map((item) => AccountModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['apps']
    if (v === undefined) {
      // omit undefined
    } else {
      out['apps'] = (v as any[]).map((item) => ApplicationModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['protocol-version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['protocolVersion'] = v as any
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
    const v = (dto as any)['latest-timestamp']
    if (v === undefined) {
      // omit undefined
    } else {
      out['latestTimestamp'] = v as any
    }
  }
  {
    const v = (dto as any)['sources']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sources'] = (v as any[]).map((item) => DryrunSourceModel.fromDto(item))
    }
  }
  return out as DryrunRequest
}

// Msgpack codecs
export function encodeMsgpack(value: DryrunRequest): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): DryrunRequest {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: DryrunRequest): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): DryrunRequest {
  return fromDto(raw as DryrunRequestDto)
}

// Array helpers
export function encodeMsgpackArray(values: DryrunRequest[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): DryrunRequest[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: DryrunRequest[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): DryrunRequest[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type DryrunRequestMsgpackDto = {
  txns: Uint8Array[]
  accounts: ReturnType<(typeof AccountModel)['toMsgpackDto']>[]
  apps: ReturnType<(typeof ApplicationModel)['toMsgpackDto']>[]
  'protocol-version': string
  round: bigint
  'latest-timestamp': bigint
  sources: ReturnType<(typeof DryrunSourceModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: DryrunRequest): DryrunRequestMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['txns']
    if (v === undefined) {
    } else {
      out['txns'] = (v as any[]).map((item) => encodeSignedTransaction(item as any))
    }
  }
  {
    const v = (value as any)['accounts']
    if (v === undefined) {
    } else {
      out['accounts'] = (v as any[]).map((item) => AccountModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['apps']
    if (v === undefined) {
    } else {
      out['apps'] = (v as any[]).map((item) => ApplicationModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['protocolVersion']
    if (v === undefined) {
    } else {
      out['protocol-version'] = v
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
    const v = (value as any)['latestTimestamp']
    if (v === undefined) {
    } else {
      out['latest-timestamp'] = v
    }
  }
  {
    const v = (value as any)['sources']
    if (v === undefined) {
    } else {
      out['sources'] = (v as any[]).map((item) => DryrunSourceModel.toMsgpackDto(item))
    }
  }
  return out as DryrunRequestMsgpackDto
}

function fromMsgpackDto(dto: DryrunRequestMsgpackDto): DryrunRequest {
  const out: any = {}
  {
    const v = (dto as any)['txns']
    if (v === undefined) {
    } else {
      out['txns'] = (v as any[]).map((item) => {
        // eslint-disable-line @typescript-eslint/no-explicit-any
        if (item instanceof Uint8Array) return decodeSignedTransaction(item)
        if (typeof item === 'object' && item != null) {
          try {
            return decodeSignedTransaction(encodeMsgPack(item))
          } catch {
            return item as any
          }
        }
        return item as any
      })
    }
  }
  {
    const v = (dto as any)['accounts']
    if (v === undefined) {
    } else {
      out['accounts'] = (v as any[]).map((item) => AccountModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['apps']
    if (v === undefined) {
    } else {
      out['apps'] = (v as any[]).map((item) => ApplicationModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['protocol-version']
    if (v === undefined) {
    } else {
      out['protocolVersion'] = v
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
    const v = (dto as any)['latest-timestamp']
    if (v === undefined) {
    } else {
      out['latestTimestamp'] = v
    }
  }
  {
    const v = (dto as any)['sources']
    if (v === undefined) {
    } else {
      out['sources'] = (v as any[]).map((item) => DryrunSourceModel.fromMsgpackDto(item))
    }
  }
  return out as DryrunRequest
}

export const DryrunRequest = {
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
