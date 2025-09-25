import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { DryrunTxnResult, DryrunTxnResultDto } from './dryrun-txn-result'
import { DryrunTxnResult as DryrunTxnResultModel } from './dryrun-txn-result'

export type TealDryrun = {
  txns: DryrunTxnResult[]
  error: string

  /**
   * Protocol version is the protocol version Dryrun was operated under.
   */
  protocolVersion: string
}

// JSON DTO shape for TealDryrun with wire keys and JSON-safe primitives
export type TealDryrunDto = {
  txns: DryrunTxnResultDto[]
  error: string
  'protocol-version': string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TealDryrun): TealDryrunDto {
  const out: any = {}
  {
    const v = (value as any)['txns']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txns'] = (v as any[]).map((item) => DryrunTxnResultModel.toDto(item))
    }
  }
  {
    const v = (value as any)['error']
    if (v === undefined) {
      // omit undefined
    } else {
      out['error'] = v
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
  return out as TealDryrunDto
}

export function fromDto(dto: TealDryrunDto): TealDryrun {
  const out: any = {}
  {
    const v = (dto as any)['txns']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txns'] = (v as any[]).map((item) => DryrunTxnResultModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['error']
    if (v === undefined) {
      // omit undefined
    } else {
      out['error'] = v as any
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
  return out as TealDryrun
}

// Msgpack codecs
export function encodeMsgpack(value: TealDryrun): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TealDryrun {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TealDryrun): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TealDryrun {
  return fromDto(raw as TealDryrunDto)
}

// Array helpers
export function encodeMsgpackArray(values: TealDryrun[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TealDryrun[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TealDryrun[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TealDryrun[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TealDryrunMsgpackDto = {
  txns: ReturnType<(typeof DryrunTxnResultModel)['toMsgpackDto']>[]
  error: string
  'protocol-version': string
}

function toMsgpackDto(value: TealDryrun): TealDryrunMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['txns']
    if (v === undefined) {
    } else {
      out['txns'] = (v as any[]).map((item) => DryrunTxnResultModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['error']
    if (v === undefined) {
    } else {
      out['error'] = v
    }
  }
  {
    const v = (value as any)['protocolVersion']
    if (v === undefined) {
    } else {
      out['protocol-version'] = v
    }
  }
  return out as TealDryrunMsgpackDto
}

function fromMsgpackDto(dto: TealDryrunMsgpackDto): TealDryrun {
  const out: any = {}
  {
    const v = (dto as any)['txns']
    if (v === undefined) {
    } else {
      out['txns'] = (v as any[]).map((item) => DryrunTxnResultModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['error']
    if (v === undefined) {
    } else {
      out['error'] = v
    }
  }
  {
    const v = (dto as any)['protocol-version']
    if (v === undefined) {
    } else {
      out['protocolVersion'] = v
    }
  }
  return out as TealDryrun
}

export const TealDryrun = {
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
