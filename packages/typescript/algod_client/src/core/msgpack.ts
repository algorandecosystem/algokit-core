import { encode as msgpackEncode, decode as msgpackDecode } from '@msgpack/msgpack'
import { IntDecoding } from './json'

/**
 * Prepare value for Algorand-compliant msgpack encoding.
 * Implements strict Algorand msgpack rules (matching go-algorand behavior):
 * 1. Omit zero values, empty strings, and empty arrays (RecursiveEmptyCheck)
 * 2. Convert bigints to smallest safe integer type (PositiveIntUnsigned)
 * 3. Sorted keys and canonical encoding are handled by msgpackEncode options
 *
 * These rules apply universally for both API communication and transaction encoding,
 * as go-algorand uses the same codec settings for all msgpack operations.
 */
// No global pre-encoding mutation; domain codecs handle omission/short-keys when applicable.

/**
 * Encode a value as msgpack using Algorand's strict encoding rules.
 * This matches go-algorand's protocol.CodecHandle settings:
 * - Canonical = true (sorted keys, deterministic encoding)
 * - RecursiveEmptyCheck = true (omit empty/zero values recursively)
 * - PositiveIntUnsigned = true (use smallest unsigned integer types)
 *
 * @param value - The value to encode
 * @returns Encoded msgpack bytes
 */
export function encodeMsgPack<T>(value: T): Uint8Array {
  return msgpackEncode(value as unknown as any, {
    // eslint-disable-line @typescript-eslint/no-explicit-any
    sortKeys: true, // Canonical = true in go-algorand
    forceIntegerToFloat: false,
    ignoreUndefined: true,
    initialBufferSize: 2048,
    useBigInt64: true, // Support for large integers
  })
}

export function decodeMsgPack<T>(buffer: Uint8Array): T {
  return msgpackDecode(buffer, { useBigInt64: true }) as T
}

export function normalizeMsgPackIntegers(value: any, intDecoding: IntDecoding): any {
  // eslint-disable-line @typescript-eslint/no-explicit-any
  switch (intDecoding) {
    case IntDecoding.BIGINT:
      return value
    case IntDecoding.UNSAFE:
      return mapBigInts(value, (bi) => Number(bi))
    case IntDecoding.SAFE:
      // Throw if any bigint is not safely representable
      traverse(value, (v) => {
        if (typeof v === 'bigint' && !Number.isSafeInteger(Number(v))) {
          throw new Error('Integer exceeds safe range while intDecoding is "safe"')
        }
      })
      return mapBigInts(value, (bi) => Number(bi))
    case IntDecoding.MIXED:
    default:
      return mapBigInts(value, (bi) => {
        const asNum = Number(bi)
        return Number.isSafeInteger(asNum) ? asNum : bi
      })
  }
}

// Helpers to map SignedTransactionDto <-> AlgokitSignedTransaction if present in responses
// Mapping and integer normalization handled centrally; domain codecs are used via vendor extensions.

function traverse(obj: any, fn: (v: any) => void): void {
  // eslint-disable-line @typescript-eslint/no-explicit-any
  if (obj == null) return
  fn(obj)
  if (Array.isArray(obj)) {
    for (const v of obj) traverse(v, fn)
  } else if (typeof obj === 'object') {
    for (const v of Object.values(obj)) traverse(v, fn)
  }
}

function mapBigInts(obj: any, mapFn: (bi: bigint) => any): any {
  // eslint-disable-line @typescript-eslint/no-explicit-any
  if (obj == null) return obj
  if (typeof obj === 'bigint') return mapFn(obj)
  if (Array.isArray(obj)) return obj.map((v) => mapBigInts(v, mapFn))
  if (typeof obj === 'object') {
    const out: any = Array.isArray(obj) ? [] : { ...obj } // eslint-disable-line @typescript-eslint/no-explicit-any
    for (const [k, v] of Object.entries(obj)) {
      out[k] = mapBigInts(v, mapFn)
    }
    return out
  }
  return obj
}
