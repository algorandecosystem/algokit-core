import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * \[hbprf\] HbProof is a signature using HeartbeatAddress's partkey, thereby showing it is online.
 */
export type HbProofFields = {
  /**
   * \[s\] Signature of the heartbeat message.
   */
  hbSig?: Uint8Array

  /**
   * \[p\] Public key of the heartbeat message.
   */
  hbPk?: Uint8Array

  /**
   * \[p2\] Key for new-style two-level ephemeral signature.
   */
  hbPk2?: Uint8Array

  /**
   * \[p1s\] Signature of OneTimeSignatureSubkeyOffsetID(PK, Batch, Offset) under the key PK2.
   */
  hbPk1sig?: Uint8Array

  /**
   * \[p2s\] Signature of OneTimeSignatureSubkeyBatchID(PK2, Batch) under the master key (OneTimeSignatureVerifier).
   */
  hbPk2sig?: Uint8Array
}

// JSON DTO shape for HbProofFields with wire keys and JSON-safe primitives
export type HbProofFieldsDto = {
  'hb-sig'?: string
  'hb-pk'?: string
  'hb-pk2'?: string
  'hb-pk1sig'?: string
  'hb-pk2sig'?: string
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: HbProofFields): HbProofFieldsDto {
  const out: any = {}
  {
    const v = (value as any)['hbSig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hb-sig'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['hbPk']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hb-pk'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['hbPk2']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hb-pk2'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['hbPk1sig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hb-pk1sig'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['hbPk2sig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hb-pk2sig'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  return out as HbProofFieldsDto
}

export function fromDto(dto: HbProofFieldsDto): HbProofFields {
  const out: any = {}
  {
    const v = (dto as any)['hb-sig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hbSig'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['hb-pk']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hbPk'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['hb-pk2']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hbPk2'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['hb-pk1sig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hbPk1sig'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['hb-pk2sig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['hbPk2sig'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  return out as HbProofFields
}

// Msgpack codecs
export function encodeMsgpack(value: HbProofFields): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): HbProofFields {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: HbProofFields): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): HbProofFields {
  return fromDto(raw as HbProofFieldsDto)
}

// Array helpers
export function encodeMsgpackArray(values: HbProofFields[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): HbProofFields[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: HbProofFields[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): HbProofFields[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type HbProofFieldsMsgpackDto = {
  'hb-sig'?: Uint8Array
  'hb-pk'?: Uint8Array
  'hb-pk2'?: Uint8Array
  'hb-pk1sig'?: Uint8Array
  'hb-pk2sig'?: Uint8Array
}

function toMsgpackDto(value: HbProofFields): HbProofFieldsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['hbSig']
    if (v === undefined) {
    } else {
      out['hb-sig'] = v
    }
  }
  {
    const v = (value as any)['hbPk']
    if (v === undefined) {
    } else {
      out['hb-pk'] = v
    }
  }
  {
    const v = (value as any)['hbPk2']
    if (v === undefined) {
    } else {
      out['hb-pk2'] = v
    }
  }
  {
    const v = (value as any)['hbPk1sig']
    if (v === undefined) {
    } else {
      out['hb-pk1sig'] = v
    }
  }
  {
    const v = (value as any)['hbPk2sig']
    if (v === undefined) {
    } else {
      out['hb-pk2sig'] = v
    }
  }
  return out as HbProofFieldsMsgpackDto
}

function fromMsgpackDto(dto: HbProofFieldsMsgpackDto): HbProofFields {
  const out: any = {}
  {
    const v = (dto as any)['hb-sig']
    if (v === undefined) {
    } else {
      out['hbSig'] = v
    }
  }
  {
    const v = (dto as any)['hb-pk']
    if (v === undefined) {
    } else {
      out['hbPk'] = v
    }
  }
  {
    const v = (dto as any)['hb-pk2']
    if (v === undefined) {
    } else {
      out['hbPk2'] = v
    }
  }
  {
    const v = (dto as any)['hb-pk1sig']
    if (v === undefined) {
    } else {
      out['hbPk1sig'] = v
    }
  }
  {
    const v = (dto as any)['hb-pk2sig']
    if (v === undefined) {
    } else {
      out['hbPk2sig'] = v
    }
  }
  return out as HbProofFields
}

export const HbProofFields = {
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
