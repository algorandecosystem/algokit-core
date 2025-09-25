import { BaseHttpRequest, type ApiRequestOptions } from './BaseHttpRequest'
import { request } from './request'

export class FetchHttpRequest extends BaseHttpRequest {
  async request<T>(options: ApiRequestOptions): Promise<T> {
    return request(this.config, options)
  }
}
