import { toCamelCaseKeysDeep, toKebabCaseKeysDeep } from './casing'
import { BYTES_FIELDS } from './bytes-map'
import type { BodyValue } from './BaseHttpRequest'

export type Media = 'application/json' | 'application/msgpack' | 'application/x-binary' | 'application/octet-stream' | string

export function serialize(modelKey: string | undefined, value: BodyValue, mediaType: Media): BodyValue | Uint8Array {
  if (value == null) return value
  if (value instanceof Uint8Array) return value

  if (typeof mediaType === 'string' && mediaType.includes('json')) {
    // Convert Uint8Array bytes fields to base64 for JSON
    const mapped = mapBytesFields(modelKey, value, 'toBase64')
    return toKebabCaseKeysDeep(mapped)
  }

  if (typeof mediaType === 'string' && mediaType.includes('msgpack')) {
    // Msgpack gets bytes natively; only key normalization
    return toKebabCaseKeysDeep(value)
  }

  return value
}

export function deserialize<T>(modelKey: string | undefined, raw: unknown, mediaType: Media): T {
  if (raw == null) return raw as T

  if (typeof mediaType === 'string' && (mediaType.includes('json') || mediaType.includes('msgpack'))) {
    const camel = toCamelCaseKeysDeep(raw)
    // For JSON, convert base64 strings into Uint8Array for bytes fields
    const mappedBytes = mediaType.includes('json') ? mapBytesFields(modelKey, camel, 'toBytes') : camel
    return mappedBytes as T
  }

  return raw as T
}

function mapBytesFields(modelKey: string | undefined, value: any, direction: 'toBase64' | 'toBytes'): any {
  // eslint-disable-line @typescript-eslint/no-explicit-any
  if (!modelKey || value == null || typeof value !== 'object') return value
  const fields = BYTES_FIELDS[modelKey]
  if (!fields || fields.length === 0) return value
  const out: any = Array.isArray(value) ? [...value] : { ...value }
  for (const f of fields) {
    if (f in out && out[f] != null) {
      if (direction === 'toBase64' && out[f] instanceof Uint8Array) {
        out[f] = btoa(String.fromCharCode(...(out[f] as Uint8Array)))
      } else if (direction === 'toBytes' && typeof out[f] === 'string') {
        const s = out[f] as string
        const bin = atob(s)
        const bytes = new Uint8Array(bin.length)
        for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i)
        out[f] = bytes
      }
    }
  }
  return out
}
