import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type {
  TransactionSignatureMultisigSubsignature,
  TransactionSignatureMultisigSubsignatureDto,
} from './transaction-signature-multisig-subsignature'
import { TransactionSignatureMultisigSubsignature as TransactionSignatureMultisigSubsignatureModel } from './transaction-signature-multisig-subsignature'

/**
 * structure holding multiple subsignatures.
 *
 * Definition:
 * crypto/multisig.go : MultisigSig
 */
export type TransactionSignatureMultisig = {
  /**
   * \[subsig\] holds pairs of public key and signatures.
   */
  subsignature?: TransactionSignatureMultisigSubsignature[]

  /**
   * \[thr\]
   */
  threshold?: bigint

  /**
   * \[v\]
   */
  version?: bigint
}

// JSON DTO shape for TransactionSignatureMultisig with wire keys and JSON-safe primitives
export type TransactionSignatureMultisigDto = {
  subsignature?: TransactionSignatureMultisigSubsignatureDto[]
  threshold?: bigint
  version?: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionSignatureMultisig): TransactionSignatureMultisigDto {
  const out: any = {}
  {
    const v = (value as any)['subsignature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['subsignature'] = (v as any[]).map((item) => TransactionSignatureMultisigSubsignatureModel.toDto(item))
    }
  }
  {
    const v = (value as any)['threshold']
    if (v === undefined) {
      // omit undefined
    } else {
      out['threshold'] = v
    }
  }
  {
    const v = (value as any)['version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['version'] = v
    }
  }
  return out as TransactionSignatureMultisigDto
}

export function fromDto(dto: TransactionSignatureMultisigDto): TransactionSignatureMultisig {
  const out: any = {}
  {
    const v = (dto as any)['subsignature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['subsignature'] = (v as any[]).map((item) => TransactionSignatureMultisigSubsignatureModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['threshold']
    if (v === undefined) {
      // omit undefined
    } else {
      out['threshold'] = v as any
    }
  }
  {
    const v = (dto as any)['version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['version'] = v as any
    }
  }
  return out as TransactionSignatureMultisig
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionSignatureMultisig): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionSignatureMultisig {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionSignatureMultisig): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionSignatureMultisig {
  return fromDto(raw as TransactionSignatureMultisigDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionSignatureMultisig[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionSignatureMultisig[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionSignatureMultisig[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionSignatureMultisig[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionSignatureMultisigMsgpackDto = {
  subsignature?: ReturnType<(typeof TransactionSignatureMultisigSubsignatureModel)['toMsgpackDto']>[]
  threshold?: bigint
  version?: bigint
}

function toMsgpackDto(value: TransactionSignatureMultisig): TransactionSignatureMultisigMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['subsignature']
    if (v === undefined) {
    } else {
      out['subsignature'] = (v as any[]).map((item) => TransactionSignatureMultisigSubsignatureModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['threshold']
    if (v === undefined) {
    } else {
      out['threshold'] = v
    }
  }
  {
    const v = (value as any)['version']
    if (v === undefined) {
    } else {
      out['version'] = v
    }
  }
  return out as TransactionSignatureMultisigMsgpackDto
}

function fromMsgpackDto(dto: TransactionSignatureMultisigMsgpackDto): TransactionSignatureMultisig {
  const out: any = {}
  {
    const v = (dto as any)['subsignature']
    if (v === undefined) {
    } else {
      out['subsignature'] = (v as any[]).map((item) => TransactionSignatureMultisigSubsignatureModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['threshold']
    if (v === undefined) {
    } else {
      out['threshold'] = v
    }
  }
  {
    const v = (dto as any)['version']
    if (v === undefined) {
    } else {
      out['version'] = v
    }
  }
  return out as TransactionSignatureMultisig
}

export const TransactionSignatureMultisig = {
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
