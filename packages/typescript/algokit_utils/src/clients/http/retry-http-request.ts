import {
  ApiError,
  type ApiRequestOptions,
  BaseHttpRequest,
  decodeMsgPack,
  encodeMsgPack,
  type ClientConfig,
} from '@algorandfoundation/algod-client'

const RETRY_STATUS_CODES = [408, 413, 429, 500, 502, 503, 504]
const RETRY_ERROR_CODES = [
  'ETIMEDOUT',
  'ECONNRESET',
  'EADDRINUSE',
  'ECONNREFUSED',
  'EPIPE',
  'ENOTFOUND',
  'ENETUNREACH',
  'EAI_AGAIN',
  'EPROTO',
]

const DEFAULT_MAX_TRIES = 5
const DEFAULT_MAX_BACKOFF_MS = 10_000

const encodeURIPath = (path: string): string => encodeURI(path).replace(/%5B/g, '[').replace(/%5D/g, ']')

const toNumber = (value: unknown): number | undefined => {
  if (typeof value === 'number') {
    return Number.isNaN(value) ? undefined : value
  }
  if (typeof value === 'string') {
    const parsed = Number(value)
    return Number.isNaN(parsed) ? undefined : parsed
  }
  return undefined
}

const extractStatus = (error: unknown): number | undefined => {
  if (!error || typeof error !== 'object') {
    return undefined
  }
  const candidate = error as { status?: unknown; response?: { status?: unknown } }
  return toNumber(candidate.status ?? candidate.response?.status)
}

const extractCode = (error: unknown): string | undefined => {
  if (!error || typeof error !== 'object') {
    return undefined
  }
  const candidate = error as { code?: unknown; cause?: { code?: unknown } }
  const raw = candidate.code ?? candidate.cause?.code
  return typeof raw === 'string' ? raw : undefined
}

const delay = async (ms: number): Promise<void> =>
  new Promise((resolve) => {
    setTimeout(resolve, ms)
  })

export interface RetryHttpRequestOptions {
  maxTries?: number
  maxBackoffMs?: number
}

export class RetryHttpRequest extends BaseHttpRequest {
  private readonly maxTries: number
  private readonly maxBackoffMs: number

  constructor(config: ClientConfig, options?: RetryHttpRequestOptions) {
    super(config)
    this.maxTries = options?.maxTries ?? DEFAULT_MAX_TRIES
    this.maxBackoffMs = options?.maxBackoffMs ?? DEFAULT_MAX_BACKOFF_MS
  }

  async request<T>(options: ApiRequestOptions): Promise<T> {
    let attempt = 1
    let lastError: unknown
    while (attempt <= this.maxTries) {
      try {
        return await this.execute<T>(options)
      } catch (error) {
        lastError = error
        if (!this.shouldRetry(error, attempt)) {
          throw error
        }

        const backoff = attempt === 1 ? 0 : Math.min(1000 * 2 ** (attempt - 1), this.maxBackoffMs)
        if (backoff > 0) {
          // eslint-disable-next-line no-console
          console.warn(`Retrying request after ${backoff}ms due to error:`, error)
          await delay(backoff)
        }
        attempt += 1
      }
    }

    throw lastError ?? new Error('Request failed after exhausting retries')
  }

  private shouldRetry(error: unknown, attempt: number): boolean {
    if (attempt >= this.maxTries) {
      return false
    }

    const status = extractStatus(error)
    if (status !== undefined && RETRY_STATUS_CODES.includes(status)) {
      return true
    }

    const code = extractCode(error)
    if (code && RETRY_ERROR_CODES.includes(code)) {
      return true
    }

    return false
  }

  private async execute<T>(options: ApiRequestOptions): Promise<T> {
    let rawPath = options.url
    if (options.path) {
      for (const [key, value] of Object.entries(options.path)) {
        const serialized = typeof value === 'bigint' ? value.toString() : String(value)
        const encoded = this.config.encodePath ? this.config.encodePath(serialized) : encodeURIPath(serialized)
        rawPath = rawPath.replace(`{${key}}`, encoded)
      }
    }

    const url = new URL(rawPath, this.config.baseUrl)

    if (options.query) {
      for (const [key, value] of Object.entries(options.query)) {
        if (value === undefined || value === null) continue
        if (Array.isArray(value)) {
          for (const item of value) {
            url.searchParams.append(key, item.toString())
          }
        } else {
          url.searchParams.append(key, value.toString())
        }
      }
    }

    const headers: Record<string, string> = {
      ...(typeof this.config.headers === 'function' ? await this.config.headers() : this.config.headers ?? {}),
      ...(options.headers ?? {}),
    }

    if (this.config.apiToken) {
      headers['X-Algo-API-Token'] = this.config.apiToken
    }

    const token = typeof this.config.token === 'function' ? await this.config.token() : this.config.token
    if (token) {
      headers['Authorization'] = `Bearer ${token}`
    } else if (this.config.username && this.config.password) {
      headers['Authorization'] = `Basic ${btoa(`${this.config.username}:${this.config.password}`)}`
    }

    let payload: BodyInit | undefined
    if (options.body != null) {
      const { body } = options
      if (body instanceof Uint8Array) {
        payload = body
      } else if (typeof body === 'string') {
        payload = body
      } else if (options.mediaType?.includes('msgpack')) {
        payload = encodeMsgPack(body)
      } else if (options.mediaType?.includes('json')) {
        payload = JSON.stringify(body)
      } else {
        payload = JSON.stringify(body)
      }
    }

    const response = await fetch(url.toString(), {
      method: options.method,
      headers,
      body: payload,
      credentials: this.config.credentials,
    })

    if (!response.ok) {
      let errorBody: unknown
      try {
        const contentType = response.headers.get('content-type') ?? ''
        if (contentType.includes('application/msgpack')) {
          errorBody = decodeMsgPack(new Uint8Array(await response.arrayBuffer()))
        } else if (contentType.includes('application/json')) {
          errorBody = JSON.parse(await response.text())
        } else {
          errorBody = await response.text()
        }
      } catch {
        errorBody = undefined
      }
      throw new ApiError(url.toString(), response.status, errorBody)
    }

    if (options.responseHeader) {
      const value = response.headers.get(options.responseHeader)
      return value as unknown as T
    }

    const contentType = response.headers.get('content-type') ?? ''

    if (contentType.includes('application/msgpack')) {
      return new Uint8Array(await response.arrayBuffer()) as unknown as T
    }

    if (contentType.includes('application/octet-stream') || contentType.includes('application/x-binary')) {
      return new Uint8Array(await response.arrayBuffer()) as unknown as T
    }

    if (contentType.includes('application/json')) {
      return (await response.text()) as unknown as T
    }

    if (!contentType) {
      return new Uint8Array(await response.arrayBuffer()) as unknown as T
    }

    return (await response.text()) as unknown as T
  }
}
