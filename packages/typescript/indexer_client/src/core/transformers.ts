import { toCamelCaseKeysDeep, toKebabCaseKeysDeep } from './casing'
import type { BodyValue } from './BaseHttpRequest'

export type Media = 'application/json' | 'application/msgpack' | 'application/x-binary' | 'application/octet-stream' | string

export function serialize(modelKey: string | undefined, value: BodyValue, mediaType: Media): BodyValue | Uint8Array {
  if (value == null) return value
  if (value instanceof Uint8Array) return value

  if (typeof mediaType === 'string' && mediaType.includes('json')) {
    return toKebabCaseKeysDeep(value)
  }

  if (typeof mediaType === 'string' && mediaType.includes('msgpack')) {
    // Default: apply same key normalization as JSON; bytes should be provided as Uint8Array already
    return toKebabCaseKeysDeep(value)
  }

  return value
}

export function deserialize<T>(modelKey: string | undefined, raw: unknown, mediaType: Media): T {
  if (raw == null) return raw as T

  if (typeof mediaType === 'string' && (mediaType.includes('json') || mediaType.includes('msgpack'))) {
    return toCamelCaseKeysDeep(raw) as T
  }

  return raw as T
}
