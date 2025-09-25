import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { BoxReference, BoxReferenceDto } from './box-reference'
import { BoxReference as BoxReferenceModel } from './box-reference'
import type { HoldingRef, HoldingRefDto } from './holding-ref'
import { HoldingRef as HoldingRefModel } from './holding-ref'
import type { LocalsRef, LocalsRefDto } from './locals-ref'
import { LocalsRef as LocalsRefModel } from './locals-ref'

/**
 * ResourceRef names a single resource. Only one of the fields should be set.
 */
export type ResourceRef = {
  /**
   * \[d\] Account whose balance record is accessible by the executing ApprovalProgram or ClearStateProgram.
   */
  address?: string

  /**
   * \[p\] Application id whose GlobalState may be read by the executing
   * ApprovalProgram or ClearStateProgram.
   */
  applicationId?: bigint

  /**
   * \[s\] Asset whose AssetParams may be read by the executing
   * ApprovalProgram or ClearStateProgram.
   */
  assetId?: bigint
  box?: BoxReference
  holding?: HoldingRef
  local?: LocalsRef
}

// JSON DTO shape for ResourceRef with wire keys and JSON-safe primitives
export type ResourceRefDto = {
  address?: string
  'application-id'?: bigint
  'asset-id'?: string
  box?: BoxReferenceDto
  holding?: HoldingRefDto
  local?: LocalsRefDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ResourceRef): ResourceRefDto {
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
    const v = (value as any)['applicationId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['application-id'] = v
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
    const v = (value as any)['box']
    if (v === undefined) {
      // omit undefined
    } else {
      out['box'] = v === undefined ? v : BoxReferenceModel.toDto(v)
    }
  }
  {
    const v = (value as any)['holding']
    if (v === undefined) {
      // omit undefined
    } else {
      out['holding'] = v === undefined ? v : HoldingRefModel.toDto(v)
    }
  }
  {
    const v = (value as any)['local']
    if (v === undefined) {
      // omit undefined
    } else {
      out['local'] = v === undefined ? v : LocalsRefModel.toDto(v)
    }
  }
  return out as ResourceRefDto
}

export function fromDto(dto: ResourceRefDto): ResourceRef {
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
    const v = (dto as any)['application-id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['applicationId'] = v as any
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
    const v = (dto as any)['box']
    if (v === undefined) {
      // omit undefined
    } else {
      out['box'] = v === undefined ? v : BoxReferenceModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['holding']
    if (v === undefined) {
      // omit undefined
    } else {
      out['holding'] = v === undefined ? v : HoldingRefModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['local']
    if (v === undefined) {
      // omit undefined
    } else {
      out['local'] = v === undefined ? v : LocalsRefModel.fromDto(v)
    }
  }
  return out as ResourceRef
}

// Msgpack codecs
export function encodeMsgpack(value: ResourceRef): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ResourceRef {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ResourceRef): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ResourceRef {
  return fromDto(raw as ResourceRefDto)
}

// Array helpers
export function encodeMsgpackArray(values: ResourceRef[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ResourceRef[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ResourceRef[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ResourceRef[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ResourceRefMsgpackDto = {
  address?: string
  'application-id'?: bigint
  'asset-id'?: bigint
  box?: ReturnType<(typeof BoxReferenceModel)['toMsgpackDto']>
  holding?: ReturnType<(typeof HoldingRefModel)['toMsgpackDto']>
  local?: ReturnType<(typeof LocalsRefModel)['toMsgpackDto']>
}

function toMsgpackDto(value: ResourceRef): ResourceRefMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (value as any)['applicationId']
    if (v === undefined) {
    } else {
      out['application-id'] = v
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
    const v = (value as any)['box']
    if (v === undefined) {
    } else {
      out['box'] = BoxReferenceModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['holding']
    if (v === undefined) {
    } else {
      out['holding'] = HoldingRefModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['local']
    if (v === undefined) {
    } else {
      out['local'] = LocalsRefModel.toMsgpackDto(v)
    }
  }
  return out as ResourceRefMsgpackDto
}

function fromMsgpackDto(dto: ResourceRefMsgpackDto): ResourceRef {
  const out: any = {}
  {
    const v = (dto as any)['address']
    if (v === undefined) {
    } else {
      out['address'] = v
    }
  }
  {
    const v = (dto as any)['application-id']
    if (v === undefined) {
    } else {
      out['applicationId'] = v
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
    const v = (dto as any)['box']
    if (v === undefined) {
    } else {
      out['box'] = BoxReferenceModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['holding']
    if (v === undefined) {
    } else {
      out['holding'] = HoldingRefModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['local']
    if (v === undefined) {
    } else {
      out['local'] = LocalsRefModel.fromMsgpackDto(v)
    }
  }
  return out as ResourceRef
}

export const ResourceRef = {
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
