import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * DryrunSource is TEAL source text that gets uploaded, compiled, and inserted into transactions or application state.
 */
export type DryrunSource = {
  /**
   * FieldName is what kind of sources this is. If lsig then it goes into the transactions[this.TxnIndex].LogicSig. If approv or clearp it goes into the Approval Program or Clear State Program of application[this.AppIndex].
   */
  fieldName: string
  source: string
  txnIndex: bigint
  appIndex: bigint
}

// JSON DTO shape for DryrunSource with wire keys and JSON-safe primitives
export type DryrunSourceDto = {
  'field-name': string
  source: string
  'txn-index': bigint
  'app-index': string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: DryrunSource): DryrunSourceDto {
  const out: any = {}
  {
    const v = (value as any)['fieldName']
    if (v === undefined) {
      // omit undefined
    } else {
      out['field-name'] = v
    }
  }
  {
    const v = (value as any)['source']
    if (v === undefined) {
      // omit undefined
    } else {
      out['source'] = v
    }
  }
  {
    const v = (value as any)['txnIndex']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txn-index'] = v
    }
  }
  {
    const v = (value as any)['appIndex']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-index'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  return out as DryrunSourceDto
}

export function fromDto(dto: DryrunSourceDto): DryrunSource {
  const out: any = {}
  {
    const v = (dto as any)['field-name']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fieldName'] = v as any
    }
  }
  {
    const v = (dto as any)['source']
    if (v === undefined) {
      // omit undefined
    } else {
      out['source'] = v as any
    }
  }
  {
    const v = (dto as any)['txn-index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txnIndex'] = v as any
    }
  }
  {
    const v = (dto as any)['app-index']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appIndex'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  return out as DryrunSource
}

// Msgpack codecs
export function encodeMsgpack(value: DryrunSource): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): DryrunSource {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: DryrunSource): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): DryrunSource {
  return fromDto(raw as DryrunSourceDto)
}

// Array helpers
export function encodeMsgpackArray(values: DryrunSource[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): DryrunSource[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: DryrunSource[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): DryrunSource[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type DryrunSourceMsgpackDto = {
  'field-name': string
  source: string
  'txn-index': bigint
  'app-index': bigint
}

function toMsgpackDto(value: DryrunSource): DryrunSourceMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['fieldName']
    if (v === undefined) {
    } else {
      out['field-name'] = v
    }
  }
  {
    const v = (value as any)['source']
    if (v === undefined) {
    } else {
      out['source'] = v
    }
  }
  {
    const v = (value as any)['txnIndex']
    if (v === undefined) {
    } else {
      out['txn-index'] = v
    }
  }
  {
    const v = (value as any)['appIndex']
    if (v === undefined) {
    } else {
      out['app-index'] = v
    }
  }
  return out as DryrunSourceMsgpackDto
}

function fromMsgpackDto(dto: DryrunSourceMsgpackDto): DryrunSource {
  const out: any = {}
  {
    const v = (dto as any)['field-name']
    if (v === undefined) {
    } else {
      out['fieldName'] = v
    }
  }
  {
    const v = (dto as any)['source']
    if (v === undefined) {
    } else {
      out['source'] = v
    }
  }
  {
    const v = (dto as any)['txn-index']
    if (v === undefined) {
    } else {
      out['txnIndex'] = v
    }
  }
  {
    const v = (dto as any)['app-index']
    if (v === undefined) {
    } else {
      out['appIndex'] = v
    }
  }
  return out as DryrunSource
}

export const DryrunSource = {
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
