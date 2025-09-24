import type { ClientConfig } from './ClientConfig'
import { ApiError } from './ApiError'
import { parseJson, stringifyJson } from './json'
import { decodeMsgPack, encodeMsgPack, normalizeMsgPackIntegers } from './msgpack'
import { toCamelCaseKeysDeep } from './casing'
import { toKebabCaseKeysDeep } from './casing'
import { serialize, deserialize } from './transformers'
import type { QueryParams, BodyValue } from './BaseHttpRequest'

const encodeURIPath = (path: string): string => encodeURI(path).replace(/%5B/g, '[').replace(/%5D/g, ']')

export async function request<T>(
  config: ClientConfig,
  options: {
    method: string
    url: string
    path?: Record<string, string | number | bigint>
    query?: QueryParams
    headers?: Record<string, string>
    body?: BodyValue
    mediaType?: string
    responseHeader?: string
    bodyModelKey?: string
    responseModelKey?: string
  },
): Promise<T> {
  // Replace path params before constructing URL to avoid encoded braces preventing replacement
  let rawPath = options.url
  if (options.path) {
    for (const [key, value] of Object.entries(options.path)) {
      const raw = typeof value === 'bigint' ? value.toString() : String(value)
      const replace = config.encodePath ? config.encodePath(raw) : encodeURIPath(raw)
      rawPath = rawPath.replace(`{${key}}`, replace)
    }
  }

  const url = new URL(rawPath, config.baseUrl)

  // Query params
  if (options.query) {
    for (const [key, value] of Object.entries(options.query)) {
      if (value === undefined || value === null) continue
      const v = typeof value === 'bigint' ? value.toString() : String(value)
      url.searchParams.append(key, v)
    }
  }

  const headers: Record<string, string> = {
    ...(typeof config.headers === 'function' ? await config.headers() : (config.headers ?? {})),
    ...(options.headers ?? {}),
  }

  // Auth: Bearer or Basic
  const token = typeof config.token === 'function' ? await config.token() : config.token
  if (token) headers['Authorization'] = `Bearer ${token}`
  if (!token && config.username && config.password) {
    headers['Authorization'] = `Basic ${btoa(`${config.username}:${config.password}`)}`
  }

  // Prepare body based on media type
  let body: BodyValue | undefined = undefined
  if (options.body != null) {
    if (options.mediaType?.includes('json')) {
      const mapped = serialize(options.bodyModelKey, options.body, options.mediaType)
      body = stringifyJson(mapped)
    } else if (options.mediaType?.includes('msgpack')) {
      if (options.body instanceof Uint8Array) {
        body = options.body
      } else {
        const mapped = serialize(options.bodyModelKey, options.body, options.mediaType)
        body = encodeMsgPack(mapped)
      }
    } else {
      // For binary/text, pass through as-is
      body = options.body
    }
  }

  const response = await fetch(url.toString(), {
    method: options.method,
    headers,
    body,
    credentials: config.credentials,
  })

  if (!response.ok) {
    let body: any = undefined
    try {
      const ct = response.headers.get('content-type')
      if (ct && ct.includes('application/json')) body = parseJson(await response.text(), config.intDecoding ?? 'mixed')
      else body = await response.text()
    } catch {}
    throw new ApiError(url.toString(), response.status, body)
  }

  if (options.responseHeader) {
    const value = response.headers.get(options.responseHeader)
    return value as unknown as T
  }

  // Parse response by content-type
  const contentType = response.headers.get('content-type') || ''

  // Handle msgpack responses - decode to typed models
  if (contentType.includes('application/msgpack')) {
    const buf = new Uint8Array(await response.arrayBuffer())
    const decoded = decodeMsgPack(buf)
    const normalized = normalizeMsgPackIntegers(decoded, config.intDecoding ?? 'bigint')
    return deserialize<T>(options.responseModelKey, normalized, 'application/msgpack')
  }

  // Handle raw binary responses (e.g., application/x-binary for raw transactions)
  if (contentType.includes('application/x-binary') || contentType.includes('application/octet-stream')) {
    // For raw binary, return as Uint8Array without decoding
    return new Uint8Array(await response.arrayBuffer()) as unknown as T
  }

  // Handle JSON responses
  if (contentType.includes('application/json')) {
    const text = await response.text()
    const parsed = parseJson(text, config.intDecoding ?? 'mixed')
    return deserialize<T>(options.responseModelKey, parsed, 'application/json')
  }

  // Fallback to text
  const text = await response.text()
  try {
    const parsed = JSON.parse(text)
    return deserialize<T>(options.responseModelKey, parsed, 'application/json')
  } catch {
    return text as unknown as T
  }
}
