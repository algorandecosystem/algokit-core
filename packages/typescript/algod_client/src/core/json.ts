// Use require to avoid ESM/CJS interop issues in consumers
// eslint-disable-next-line @typescript-eslint/no-explicit-any
declare const require: any
// eslint-disable-next-line @typescript-eslint/no-var-requires
const JSONBigFactory = require('json-bigint')

/**
 * Configure how integers in JSON response will be decoded.
 */
export enum IntDecoding {
  /**
   * All integers will be decoded as Numbers, meaning any values greater than
   * Number.MAX_SAFE_INTEGER will lose precision.
   */
  UNSAFE = 'unsafe',

  /**
   * All integers will be decoded as Numbers, but if any values are greater than
   * Number.MAX_SAFE_INTEGER an error will be thrown.
   */
  SAFE = 'safe',

  /**
   * Integers will be decoded as Numbers if they are less than or equal to
   * Number.MAX_SAFE_INTEGER, otherwise they will be decoded as BigInts.
   */
  MIXED = 'mixed',

  /**
   * All integers will be decoded as BigInts.
   */
  BIGINT = 'bigint',
}

// Instances
const JSONBigMixed = JSONBigFactory({ useNativeBigInt: true, alwaysParseAsBig: false })
const JSONBigAllBig = JSONBigFactory({ useNativeBigInt: true, alwaysParseAsBig: true })

function traverseAndThrowOnBigInt(obj: any): void {
  // eslint-disable-line @typescript-eslint/no-explicit-any
  if (obj === null || obj === undefined) return
  const t = typeof obj
  if (t === 'bigint') {
    throw new Error('Integer exceeds safe range while intDecoding is "safe"')
  }
  if (t !== 'object') return
  if (Array.isArray(obj)) {
    for (const v of obj) traverseAndThrowOnBigInt(v)
  } else {
    for (const v of Object.values(obj)) traverseAndThrowOnBigInt(v)
  }
}

function convertLargeNumericStrings(obj: any): any {
  // eslint-disable-line @typescript-eslint/no-explicit-any
  if (obj == null) return obj
  if (typeof obj === 'string') {
    if (/^\d+$/.test(obj)) {
      const asNum = Number(obj)
      if (!Number.isSafeInteger(asNum)) return BigInt(obj)
    }
    return obj
  }
  if (Array.isArray(obj)) return obj.map(convertLargeNumericStrings)
  if (typeof obj === 'object') {
    for (const k of Object.keys(obj)) obj[k] = convertLargeNumericStrings(obj[k])
  }
  return obj
}

export function parseJson(text: string, intDecoding: IntDecoding = IntDecoding.MIXED): any {
  // eslint-disable-line @typescript-eslint/no-explicit-any
  switch (intDecoding) {
    case IntDecoding.UNSAFE:
      return JSON.parse(text)
    case IntDecoding.BIGINT: {
      const v = JSONBigAllBig.parse(text)
      return convertLargeNumericStrings(v)
    }
    case IntDecoding.SAFE: {
      const value = JSONBigMixed.parse(text)
      traverseAndThrowOnBigInt(value)
      return value
    }
    case IntDecoding.MIXED:
    default: {
      const v = JSONBigMixed.parse(text)
      return convertLargeNumericStrings(v)
    }
  }
}

export function stringifyJson(value: any): string {
  // eslint-disable-line @typescript-eslint/no-explicit-any
  return JSON.stringify(value, (_k, v) => (typeof v === 'bigint' ? v.toString() : v))
}
