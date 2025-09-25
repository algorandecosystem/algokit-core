import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { TransactionSignatureMultisig, TransactionSignatureMultisigDto } from './transaction-signature-multisig'
import { TransactionSignatureMultisig as TransactionSignatureMultisigModel } from './transaction-signature-multisig'

/**
 * \[lsig\] Programatic transaction signature.
 *
 * Definition:
 * data/transactions/logicsig.go
 */
export type TransactionSignatureLogicsig = {
  /**
   * \[arg\] Logic arguments, base64 encoded.
   */
  args?: string[]

  /**
   * \[l\] Program signed by a signature or multi signature, or hashed to be the address of an account. Base64 encoded TEAL program.
   */
  logic: Uint8Array
  multisigSignature?: TransactionSignatureMultisig
  logicMultisigSignature?: TransactionSignatureMultisig

  /**
   * \[sig\] ed25519 signature.
   */
  signature?: Uint8Array
}

// JSON DTO shape for TransactionSignatureLogicsig with wire keys and JSON-safe primitives
export type TransactionSignatureLogicsigDto = {
  args?: string[][]
  logic: string
  'multisig-signature'?: TransactionSignatureMultisigDto
  'logic-multisig-signature'?: TransactionSignatureMultisigDto
  signature?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionSignatureLogicsig): TransactionSignatureLogicsigDto {
  const out: any = {}
  {
    const v = (value as any)['args']
    if (v === undefined) {
      // omit undefined
    } else {
      out['args'] = v as any[]
    }
  }
  {
    const v = (value as any)['logic']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logic'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['multisigSignature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['multisig-signature'] = v === undefined ? v : TransactionSignatureMultisigModel.toDto(v)
    }
  }
  {
    const v = (value as any)['logicMultisigSignature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logic-multisig-signature'] = v === undefined ? v : TransactionSignatureMultisigModel.toDto(v)
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
  return out as TransactionSignatureLogicsigDto
}

export function fromDto(dto: TransactionSignatureLogicsigDto): TransactionSignatureLogicsig {
  const out: any = {}
  {
    const v = (dto as any)['args']
    if (v === undefined) {
      // omit undefined
    } else {
      out['args'] = v as any[]
    }
  }
  {
    const v = (dto as any)['logic']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logic'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['multisig-signature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['multisigSignature'] = v === undefined ? v : TransactionSignatureMultisigModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['logic-multisig-signature']
    if (v === undefined) {
      // omit undefined
    } else {
      out['logicMultisigSignature'] = v === undefined ? v : TransactionSignatureMultisigModel.fromDto(v)
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
  return out as TransactionSignatureLogicsig
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionSignatureLogicsig): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionSignatureLogicsig {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionSignatureLogicsig): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionSignatureLogicsig {
  return fromDto(raw as TransactionSignatureLogicsigDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionSignatureLogicsig[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionSignatureLogicsig[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionSignatureLogicsig[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionSignatureLogicsig[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionSignatureLogicsigMsgpackDto = {
  args?: string[][]
  logic: Uint8Array
  'multisig-signature'?: ReturnType<(typeof TransactionSignatureMultisigModel)['toMsgpackDto']>
  'logic-multisig-signature'?: ReturnType<(typeof TransactionSignatureMultisigModel)['toMsgpackDto']>
  signature?: Uint8Array
}

function toMsgpackDto(value: TransactionSignatureLogicsig): TransactionSignatureLogicsigMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['args']
    if (v === undefined) {
    } else {
      out['args'] = v as any[]
    }
  }
  {
    const v = (value as any)['logic']
    if (v === undefined) {
    } else {
      out['logic'] = v
    }
  }
  {
    const v = (value as any)['multisigSignature']
    if (v === undefined) {
    } else {
      out['multisig-signature'] = TransactionSignatureMultisigModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['logicMultisigSignature']
    if (v === undefined) {
    } else {
      out['logic-multisig-signature'] = TransactionSignatureMultisigModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['signature']
    if (v === undefined) {
    } else {
      out['signature'] = v
    }
  }
  return out as TransactionSignatureLogicsigMsgpackDto
}

function fromMsgpackDto(dto: TransactionSignatureLogicsigMsgpackDto): TransactionSignatureLogicsig {
  const out: any = {}
  {
    const v = (dto as any)['args']
    if (v === undefined) {
    } else {
      out['args'] = v as any[]
    }
  }
  {
    const v = (dto as any)['logic']
    if (v === undefined) {
    } else {
      out['logic'] = v
    }
  }
  {
    const v = (dto as any)['multisig-signature']
    if (v === undefined) {
    } else {
      out['multisigSignature'] = TransactionSignatureMultisigModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['logic-multisig-signature']
    if (v === undefined) {
    } else {
      out['logicMultisigSignature'] = TransactionSignatureMultisigModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['signature']
    if (v === undefined) {
    } else {
      out['signature'] = v
    }
  }
  return out as TransactionSignatureLogicsig
}

export const TransactionSignatureLogicsig = {
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
