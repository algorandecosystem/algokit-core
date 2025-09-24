import { BaseHttpRequest, type ApiRequestOptions } from './BaseHttpRequest'
import { request as defaultRequest } from './request'

export class FetchHttpRequest extends BaseHttpRequest {
  async request<T>(options: ApiRequestOptions): Promise<T> {
    const request = defaultRequest
    return request(this.config, options)
  }
}
