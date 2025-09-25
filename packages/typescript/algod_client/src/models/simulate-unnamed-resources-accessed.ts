import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { ApplicationLocalReference, ApplicationLocalReferenceDto } from './application-local-reference'
import { ApplicationLocalReference as ApplicationLocalReferenceModel } from './application-local-reference'
import type { AssetHoldingReference, AssetHoldingReferenceDto } from './asset-holding-reference'
import { AssetHoldingReference as AssetHoldingReferenceModel } from './asset-holding-reference'
import type { BoxReference, BoxReferenceDto } from './box-reference'
import { BoxReference as BoxReferenceModel } from './box-reference'

/**
 * These are resources that were accessed by this group that would normally have caused failure, but were allowed in simulation. Depending on where this object is in the response, the unnamed resources it contains may or may not qualify for group resource sharing. If this is a field in SimulateTransactionGroupResult, the resources do qualify, but if this is a field in SimulateTransactionResult, they do not qualify. In order to make this group valid for actual submission, resources that qualify for group sharing can be made available by any transaction of the group; otherwise, resources must be placed in the same transaction which accessed them.
 */
export type SimulateUnnamedResourcesAccessed = {
  /**
   * The unnamed accounts that were referenced. The order of this array is arbitrary.
   */
  accounts?: string[]

  /**
   * The unnamed assets that were referenced. The order of this array is arbitrary.
   */
  assets?: bigint[]

  /**
   * The unnamed applications that were referenced. The order of this array is arbitrary.
   */
  apps?: bigint[]

  /**
   * The unnamed boxes that were referenced. The order of this array is arbitrary.
   */
  boxes?: BoxReference[]

  /**
   * The number of extra box references used to increase the IO budget. This is in addition to the references defined in the input transaction group and any referenced to unnamed boxes.
   */
  extraBoxRefs?: bigint

  /**
   * The unnamed asset holdings that were referenced. The order of this array is arbitrary.
   */
  assetHoldings?: AssetHoldingReference[]

  /**
   * The unnamed application local states that were referenced. The order of this array is arbitrary.
   */
  appLocals?: ApplicationLocalReference[]
}

// JSON DTO shape for SimulateUnnamedResourcesAccessed with wire keys and JSON-safe primitives
export type SimulateUnnamedResourcesAccessedDto = {
  accounts?: string[]
  assets?: string[]
  apps?: string[]
  boxes?: BoxReferenceDto[]
  'extra-box-refs'?: bigint
  'asset-holdings'?: AssetHoldingReferenceDto[]
  'app-locals'?: ApplicationLocalReferenceDto[]
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulateUnnamedResourcesAccessed): SimulateUnnamedResourcesAccessedDto {
  const out: any = {}
  {
    const v = (value as any)['accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['accounts'] = v as any[]
    }
  }
  {
    const v = (value as any)['assets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assets'] = (v as Array<number | bigint | string>).map((x) => (typeof x === 'bigint' ? x.toString() : String(x)))
    }
  }
  {
    const v = (value as any)['apps']
    if (v === undefined) {
      // omit undefined
    } else {
      out['apps'] = (v as Array<number | bigint | string>).map((x) => (typeof x === 'bigint' ? x.toString() : String(x)))
    }
  }
  {
    const v = (value as any)['boxes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxReferenceModel.toDto(item))
    }
  }
  {
    const v = (value as any)['extraBoxRefs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['extra-box-refs'] = v
    }
  }
  {
    const v = (value as any)['assetHoldings']
    if (v === undefined) {
      // omit undefined
    } else {
      out['asset-holdings'] = (v as any[]).map((item) => AssetHoldingReferenceModel.toDto(item))
    }
  }
  {
    const v = (value as any)['appLocals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-locals'] = (v as any[]).map((item) => ApplicationLocalReferenceModel.toDto(item))
    }
  }
  return out as SimulateUnnamedResourcesAccessedDto
}

export function fromDto(dto: SimulateUnnamedResourcesAccessedDto): SimulateUnnamedResourcesAccessed {
  const out: any = {}
  {
    const v = (dto as any)['accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['accounts'] = v as any[]
    }
  }
  {
    const v = (dto as any)['assets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assets'] = (v as Array<string | number | bigint>).map((x) => (typeof x === 'bigint' ? x : BigInt(x as any)))
    }
  }
  {
    const v = (dto as any)['apps']
    if (v === undefined) {
      // omit undefined
    } else {
      out['apps'] = (v as Array<string | number | bigint>).map((x) => (typeof x === 'bigint' ? x : BigInt(x as any)))
    }
  }
  {
    const v = (dto as any)['boxes']
    if (v === undefined) {
      // omit undefined
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxReferenceModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['extra-box-refs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['extraBoxRefs'] = v as any
    }
  }
  {
    const v = (dto as any)['asset-holdings']
    if (v === undefined) {
      // omit undefined
    } else {
      out['assetHoldings'] = (v as any[]).map((item) => AssetHoldingReferenceModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['app-locals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appLocals'] = (v as any[]).map((item) => ApplicationLocalReferenceModel.fromDto(item))
    }
  }
  return out as SimulateUnnamedResourcesAccessed
}

// Msgpack codecs
export function encodeMsgpack(value: SimulateUnnamedResourcesAccessed): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulateUnnamedResourcesAccessed {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulateUnnamedResourcesAccessed): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulateUnnamedResourcesAccessed {
  return fromDto(raw as SimulateUnnamedResourcesAccessedDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulateUnnamedResourcesAccessed[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulateUnnamedResourcesAccessed[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulateUnnamedResourcesAccessed[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulateUnnamedResourcesAccessed[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulateUnnamedResourcesAccessedMsgpackDto = {
  accounts?: string[]
  assets?: bigint[]
  apps?: bigint[]
  boxes?: ReturnType<(typeof BoxReferenceModel)['toMsgpackDto']>[]
  'extra-box-refs'?: bigint
  'asset-holdings'?: ReturnType<(typeof AssetHoldingReferenceModel)['toMsgpackDto']>[]
  'app-locals'?: ReturnType<(typeof ApplicationLocalReferenceModel)['toMsgpackDto']>[]
}

function toMsgpackDto(value: SimulateUnnamedResourcesAccessed): SimulateUnnamedResourcesAccessedMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['accounts']
    if (v === undefined) {
    } else {
      out['accounts'] = v as any[]
    }
  }
  {
    const v = (value as any)['assets']
    if (v === undefined) {
    } else {
      out['assets'] = v as any[]
    }
  }
  {
    const v = (value as any)['apps']
    if (v === undefined) {
    } else {
      out['apps'] = v as any[]
    }
  }
  {
    const v = (value as any)['boxes']
    if (v === undefined) {
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxReferenceModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['extraBoxRefs']
    if (v === undefined) {
    } else {
      out['extra-box-refs'] = v
    }
  }
  {
    const v = (value as any)['assetHoldings']
    if (v === undefined) {
    } else {
      out['asset-holdings'] = (v as any[]).map((item) => AssetHoldingReferenceModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['appLocals']
    if (v === undefined) {
    } else {
      out['app-locals'] = (v as any[]).map((item) => ApplicationLocalReferenceModel.toMsgpackDto(item))
    }
  }
  return out as SimulateUnnamedResourcesAccessedMsgpackDto
}

function fromMsgpackDto(dto: SimulateUnnamedResourcesAccessedMsgpackDto): SimulateUnnamedResourcesAccessed {
  const out: any = {}
  {
    const v = (dto as any)['accounts']
    if (v === undefined) {
    } else {
      out['accounts'] = v as any[]
    }
  }
  {
    const v = (dto as any)['assets']
    if (v === undefined) {
    } else {
      out['assets'] = v as any[]
    }
  }
  {
    const v = (dto as any)['apps']
    if (v === undefined) {
    } else {
      out['apps'] = v as any[]
    }
  }
  {
    const v = (dto as any)['boxes']
    if (v === undefined) {
    } else {
      out['boxes'] = (v as any[]).map((item) => BoxReferenceModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['extra-box-refs']
    if (v === undefined) {
    } else {
      out['extraBoxRefs'] = v
    }
  }
  {
    const v = (dto as any)['asset-holdings']
    if (v === undefined) {
    } else {
      out['assetHoldings'] = (v as any[]).map((item) => AssetHoldingReferenceModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['app-locals']
    if (v === undefined) {
    } else {
      out['appLocals'] = (v as any[]).map((item) => ApplicationLocalReferenceModel.fromMsgpackDto(item))
    }
  }
  return out as SimulateUnnamedResourcesAccessed
}

export const SimulateUnnamedResourcesAccessed = {
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
