import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * AssetParams specifies the parameters for an asset.
 *
 * \[apar\] when part of an AssetConfig transaction.
 *
 * Definition:
 * data/transactions/asset.go : AssetParams
 */
export type AssetParams = {
  /**
   * Address of account used to clawback holdings of this asset.  If empty, clawback is not permitted.
   */
  clawback?: string

  /**
   * The address that created this asset. This is the address where the parameters for this asset can be found, and also the address where unwanted asset units can be sent in the worst case.
   */
  creator: string

  /**
   * The number of digits to use after the decimal point when displaying this asset. If 0, the asset is not divisible. If 1, the base unit of the asset is in tenths. If 2, the base unit of the asset is in hundredths, and so on. This value must be between 0 and 19 (inclusive).
   */
  decimals: bigint

  /**
   * Whether holdings of this asset are frozen by default.
   */
  defaultFrozen?: boolean

  /**
   * Address of account used to freeze holdings of this asset.  If empty, freezing is not permitted.
   */
  freeze?: string

  /**
   * Address of account used to manage the keys of this asset and to destroy it.
   */
  manager?: string

  /**
   * A commitment to some unspecified asset metadata. The format of this metadata is up to the application.
   */
  metadataHash?: Uint8Array

  /**
   * Name of this asset, as supplied by the creator. Included only when the asset name is composed of printable utf-8 characters.
   */
  name?: string

  /**
   * Base64 encoded name of this asset, as supplied by the creator.
   */
  nameB64?: Uint8Array

  /**
   * Address of account holding reserve (non-minted) units of this asset.
   */
  reserve?: string

  /**
   * The total number of units of this asset.
   */
  total: bigint

  /**
   * Name of a unit of this asset, as supplied by the creator. Included only when the name of a unit of this asset is composed of printable utf-8 characters.
   */
  unitName?: string

  /**
   * Base64 encoded name of a unit of this asset, as supplied by the creator.
   */
  unitNameB64?: Uint8Array

  /**
   * URL where more information about the asset can be retrieved. Included only when the URL is composed of printable utf-8 characters.
   */
  url?: string

  /**
   * Base64 encoded URL where more information about the asset can be retrieved.
   */
  urlB64?: Uint8Array
}

// JSON DTO shape for AssetParams with wire keys and JSON-safe primitives
export type AssetParamsDto = {
  clawback?: string
  creator: string
  decimals: bigint
  'default-frozen'?: boolean
  freeze?: string
  manager?: string
  'metadata-hash'?: string
  name?: string
  'name-b64'?: string
  reserve?: string
  total: string
  'unit-name'?: string
  'unit-name-b64'?: string
  url?: string
  'url-b64'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AssetParams): AssetParamsDto {
  const out: any = {}
  {
    const v = (value as any)['clawback']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clawback'] = v
    }
  }
  {
    const v = (value as any)['creator']
    if (v === undefined) {
      // omit undefined
    } else {
      out['creator'] = v
    }
  }
  {
    const v = (value as any)['decimals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['decimals'] = v
    }
  }
  {
    const v = (value as any)['defaultFrozen']
    if (v === undefined) {
      // omit undefined
    } else {
      out['default-frozen'] = v
    }
  }
  {
    const v = (value as any)['freeze']
    if (v === undefined) {
      // omit undefined
    } else {
      out['freeze'] = v
    }
  }
  {
    const v = (value as any)['manager']
    if (v === undefined) {
      // omit undefined
    } else {
      out['manager'] = v
    }
  }
  {
    const v = (value as any)['metadataHash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['metadata-hash'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['name']
    if (v === undefined) {
      // omit undefined
    } else {
      out['name'] = v
    }
  }
  {
    const v = (value as any)['nameB64']
    if (v === undefined) {
      // omit undefined
    } else {
      out['name-b64'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['reserve']
    if (v === undefined) {
      // omit undefined
    } else {
      out['reserve'] = v
    }
  }
  {
    const v = (value as any)['total']
    if (v === undefined) {
      // omit undefined
    } else {
      out['total'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['unitName']
    if (v === undefined) {
      // omit undefined
    } else {
      out['unit-name'] = v
    }
  }
  {
    const v = (value as any)['unitNameB64']
    if (v === undefined) {
      // omit undefined
    } else {
      out['unit-name-b64'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['url']
    if (v === undefined) {
      // omit undefined
    } else {
      out['url'] = v
    }
  }
  {
    const v = (value as any)['urlB64']
    if (v === undefined) {
      // omit undefined
    } else {
      out['url-b64'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as AssetParamsDto
}

export function fromDto(dto: AssetParamsDto): AssetParams {
  const out: any = {}
  {
    const v = (dto as any)['clawback']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clawback'] = v as any
    }
  }
  {
    const v = (dto as any)['creator']
    if (v === undefined) {
      // omit undefined
    } else {
      out['creator'] = v as any
    }
  }
  {
    const v = (dto as any)['decimals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['decimals'] = v as any
    }
  }
  {
    const v = (dto as any)['default-frozen']
    if (v === undefined) {
      // omit undefined
    } else {
      out['defaultFrozen'] = v as any
    }
  }
  {
    const v = (dto as any)['freeze']
    if (v === undefined) {
      // omit undefined
    } else {
      out['freeze'] = v as any
    }
  }
  {
    const v = (dto as any)['manager']
    if (v === undefined) {
      // omit undefined
    } else {
      out['manager'] = v as any
    }
  }
  {
    const v = (dto as any)['metadata-hash']
    if (v === undefined) {
      // omit undefined
    } else {
      out['metadataHash'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['name']
    if (v === undefined) {
      // omit undefined
    } else {
      out['name'] = v as any
    }
  }
  {
    const v = (dto as any)['name-b64']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nameB64'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['reserve']
    if (v === undefined) {
      // omit undefined
    } else {
      out['reserve'] = v as any
    }
  }
  {
    const v = (dto as any)['total']
    if (v === undefined) {
      // omit undefined
    } else {
      out['total'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['unit-name']
    if (v === undefined) {
      // omit undefined
    } else {
      out['unitName'] = v as any
    }
  }
  {
    const v = (dto as any)['unit-name-b64']
    if (v === undefined) {
      // omit undefined
    } else {
      out['unitNameB64'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['url']
    if (v === undefined) {
      // omit undefined
    } else {
      out['url'] = v as any
    }
  }
  {
    const v = (dto as any)['url-b64']
    if (v === undefined) {
      // omit undefined
    } else {
      out['urlB64'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as AssetParams
}

// Msgpack codecs
export function encodeMsgpack(value: AssetParams): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AssetParams {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AssetParams): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AssetParams {
  return fromDto(raw as AssetParamsDto)
}

// Array helpers
export function encodeMsgpackArray(values: AssetParams[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AssetParams[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AssetParams[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AssetParams[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AssetParamsMsgpackDto = {
  clawback?: string
  creator: string
  decimals: bigint
  'default-frozen'?: boolean
  freeze?: string
  manager?: string
  'metadata-hash'?: Uint8Array
  name?: string
  'name-b64'?: Uint8Array
  reserve?: string
  total: bigint
  'unit-name'?: string
  'unit-name-b64'?: Uint8Array
  url?: string
  'url-b64'?: Uint8Array
}

function toMsgpackDto(value: AssetParams): AssetParamsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['clawback']
    if (v === undefined) {
    } else {
      out['clawback'] = v
    }
  }
  {
    const v = (value as any)['creator']
    if (v === undefined) {
    } else {
      out['creator'] = v
    }
  }
  {
    const v = (value as any)['decimals']
    if (v === undefined) {
    } else {
      out['decimals'] = v
    }
  }
  {
    const v = (value as any)['defaultFrozen']
    if (v === undefined) {
    } else {
      out['default-frozen'] = v
    }
  }
  {
    const v = (value as any)['freeze']
    if (v === undefined) {
    } else {
      out['freeze'] = v
    }
  }
  {
    const v = (value as any)['manager']
    if (v === undefined) {
    } else {
      out['manager'] = v
    }
  }
  {
    const v = (value as any)['metadataHash']
    if (v === undefined) {
    } else {
      out['metadata-hash'] = v
    }
  }
  {
    const v = (value as any)['name']
    if (v === undefined) {
    } else {
      out['name'] = v
    }
  }
  {
    const v = (value as any)['nameB64']
    if (v === undefined) {
    } else {
      out['name-b64'] = v
    }
  }
  {
    const v = (value as any)['reserve']
    if (v === undefined) {
    } else {
      out['reserve'] = v
    }
  }
  {
    const v = (value as any)['total']
    if (v === undefined) {
    } else {
      out['total'] = v
    }
  }
  {
    const v = (value as any)['unitName']
    if (v === undefined) {
    } else {
      out['unit-name'] = v
    }
  }
  {
    const v = (value as any)['unitNameB64']
    if (v === undefined) {
    } else {
      out['unit-name-b64'] = v
    }
  }
  {
    const v = (value as any)['url']
    if (v === undefined) {
    } else {
      out['url'] = v
    }
  }
  {
    const v = (value as any)['urlB64']
    if (v === undefined) {
    } else {
      out['url-b64'] = v
    }
  }
  return out as AssetParamsMsgpackDto
}

function fromMsgpackDto(dto: AssetParamsMsgpackDto): AssetParams {
  const out: any = {}
  {
    const v = (dto as any)['clawback']
    if (v === undefined) {
    } else {
      out['clawback'] = v
    }
  }
  {
    const v = (dto as any)['creator']
    if (v === undefined) {
    } else {
      out['creator'] = v
    }
  }
  {
    const v = (dto as any)['decimals']
    if (v === undefined) {
    } else {
      out['decimals'] = v
    }
  }
  {
    const v = (dto as any)['default-frozen']
    if (v === undefined) {
    } else {
      out['defaultFrozen'] = v
    }
  }
  {
    const v = (dto as any)['freeze']
    if (v === undefined) {
    } else {
      out['freeze'] = v
    }
  }
  {
    const v = (dto as any)['manager']
    if (v === undefined) {
    } else {
      out['manager'] = v
    }
  }
  {
    const v = (dto as any)['metadata-hash']
    if (v === undefined) {
    } else {
      out['metadataHash'] = v
    }
  }
  {
    const v = (dto as any)['name']
    if (v === undefined) {
    } else {
      out['name'] = v
    }
  }
  {
    const v = (dto as any)['name-b64']
    if (v === undefined) {
    } else {
      out['nameB64'] = v
    }
  }
  {
    const v = (dto as any)['reserve']
    if (v === undefined) {
    } else {
      out['reserve'] = v
    }
  }
  {
    const v = (dto as any)['total']
    if (v === undefined) {
    } else {
      out['total'] = v
    }
  }
  {
    const v = (dto as any)['unit-name']
    if (v === undefined) {
    } else {
      out['unitName'] = v
    }
  }
  {
    const v = (dto as any)['unit-name-b64']
    if (v === undefined) {
    } else {
      out['unitNameB64'] = v
    }
  }
  {
    const v = (dto as any)['url']
    if (v === undefined) {
    } else {
      out['url'] = v
    }
  }
  {
    const v = (dto as any)['url-b64']
    if (v === undefined) {
    } else {
      out['urlB64'] = v
    }
  }
  return out as AssetParams
}

export const AssetParams = {
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
