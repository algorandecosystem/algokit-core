import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { TransactionSignatureLogicsig, TransactionSignatureLogicsigDto } from './transaction-signature-logicsig'
import { TransactionSignatureLogicsig as TransactionSignatureLogicsigModel } from './transaction-signature-logicsig'
import type { TransactionSignatureMultisig, TransactionSignatureMultisigDto } from './transaction-signature-multisig'
import { TransactionSignatureMultisig as TransactionSignatureMultisigModel } from './transaction-signature-multisig'

/**
 * Validation signature associated with some data. Only one of the signatures should be provided.
 */
export type TransactionSignature = {
  logicsig?: TransactionSignatureLogicsig
  multisig?: TransactionSignatureMultisig

  /**
   * \[sig\] Standard ed25519 signature.
   */
  sig?: Uint8Array
}

// JSON DTO shape for TransactionSignature with wire keys and JSON-safe primitives
export type TransactionSignatureDto = {
  logicsig?: TransactionSignatureLogicsigDto
  multisig?: TransactionSignatureMultisigDto
  sig?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionSignature): TransactionSignatureDto {
  const out: any = {}
  {
    const v = (value as any)['logicsig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logicsig'] = v === undefined ? v : TransactionSignatureLogicsigModel.toDto(v)
    }
  }
  {
    const v = (value as any)['multisig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['multisig'] = v === undefined ? v : TransactionSignatureMultisigModel.toDto(v)
    }
  }
  {
    const v = (value as any)['sig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sig'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as TransactionSignatureDto
}

export function fromDto(dto: TransactionSignatureDto): TransactionSignature {
  const out: any = {}
  {
    const v = (dto as any)['logicsig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logicsig'] = v === undefined ? v : TransactionSignatureLogicsigModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['multisig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['multisig'] = v === undefined ? v : TransactionSignatureMultisigModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['sig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['sig'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as TransactionSignature
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionSignature): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionSignature {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionSignature): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionSignature {
  return fromDto(raw as TransactionSignatureDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionSignature[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionSignature[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionSignature[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionSignature[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionSignatureMsgpackDto = {
  logicsig?: ReturnType<(typeof TransactionSignatureLogicsigModel)['toMsgpackDto']>
  multisig?: ReturnType<(typeof TransactionSignatureMultisigModel)['toMsgpackDto']>
  sig?: Uint8Array
}

function toMsgpackDto(value: TransactionSignature): TransactionSignatureMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['logicsig']
    if (v === undefined) {
    } else {
      out['logicsig'] = TransactionSignatureLogicsigModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['multisig']
    if (v === undefined) {
    } else {
      out['multisig'] = TransactionSignatureMultisigModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['sig']
    if (v === undefined) {
    } else {
      out['sig'] = v
    }
  }
  return out as TransactionSignatureMsgpackDto
}

function fromMsgpackDto(dto: TransactionSignatureMsgpackDto): TransactionSignature {
  const out: any = {}
  {
    const v = (dto as any)['logicsig']
    if (v === undefined) {
    } else {
      out['logicsig'] = TransactionSignatureLogicsigModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['multisig']
    if (v === undefined) {
    } else {
      out['multisig'] = TransactionSignatureMultisigModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['sig']
    if (v === undefined) {
    } else {
      out['sig'] = v
    }
  }
  return out as TransactionSignature
}

export const TransactionSignature = {
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
