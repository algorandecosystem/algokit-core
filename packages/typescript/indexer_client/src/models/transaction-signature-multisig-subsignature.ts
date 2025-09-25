import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

export type TransactionSignatureMultisigSubsignature = {
  /**
   * \[pk\]
   */
  publicKey?: Uint8Array

  /**
   * \[s\]
   */
  signature?: Uint8Array
}

// JSON DTO shape for TransactionSignatureMultisigSubsignature with wire keys and JSON-safe primitives
export type TransactionSignatureMultisigSubsignatureDto = {
  'public-key'?: string
  signature?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionSignatureMultisigSubsignature): TransactionSignatureMultisigSubsignatureDto {
  const out: any = {}
  {
    const v = (value as any)['publicKey']
    if (v === undefined) {
      // omit undefined
    } else {
      out['public-key'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['signature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['signature'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as TransactionSignatureMultisigSubsignatureDto
}

export function fromDto(dto: TransactionSignatureMultisigSubsignatureDto): TransactionSignatureMultisigSubsignature {
  const out: any = {}
  {
    const v = (dto as any)['public-key']
    if (v === undefined) {
      // omit undefined
    } else {
      out['publicKey'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['signature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['signature'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as TransactionSignatureMultisigSubsignature
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionSignatureMultisigSubsignature): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionSignatureMultisigSubsignature {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionSignatureMultisigSubsignature): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionSignatureMultisigSubsignature {
  return fromDto(raw as TransactionSignatureMultisigSubsignatureDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionSignatureMultisigSubsignature[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionSignatureMultisigSubsignature[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionSignatureMultisigSubsignature[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionSignatureMultisigSubsignature[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionSignatureMultisigSubsignatureMsgpackDto = {
  'public-key'?: Uint8Array
  signature?: Uint8Array
}

function toMsgpackDto(value: TransactionSignatureMultisigSubsignature): TransactionSignatureMultisigSubsignatureMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['publicKey']
    if (v === undefined) {
    } else {
      out['public-key'] = v
    }
  }
  {
    const v = (value as any)['signature']
    if (v === undefined) {
    } else {
      out['signature'] = v
    }
  }
  return out as TransactionSignatureMultisigSubsignatureMsgpackDto
}

function fromMsgpackDto(dto: TransactionSignatureMultisigSubsignatureMsgpackDto): TransactionSignatureMultisigSubsignature {
  const out: any = {}
  {
    const v = (dto as any)['public-key']
    if (v === undefined) {
    } else {
      out['publicKey'] = v
    }
  }
  {
    const v = (dto as any)['signature']
    if (v === undefined) {
    } else {
      out['signature'] = v
    }
  }
  return out as TransactionSignatureMultisigSubsignature
}

export const TransactionSignatureMultisigSubsignature = {
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
