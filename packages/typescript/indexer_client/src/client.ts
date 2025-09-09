import type { ClientConfig } from "./core/ClientConfig";
import { FetchHttpRequest } from "./core/FetchHttpRequest";
import { IndexerApi } from "./apis/api.service";

export class IndexerClient extends IndexerApi {
  public readonly request: FetchHttpRequest;

  constructor(config: ClientConfig) {
    super(new FetchHttpRequest(config));
    this.request = this.httpRequest as FetchHttpRequest;
  }
}
