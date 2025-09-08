import type { ClientConfig } from "./core/ClientConfig";
import { FetchHttpRequest } from "./core/FetchHttpRequest";
import { IndexerApi } from "./apis/api.service";

export class IndexerClient {
  public readonly request: FetchHttpRequest;
  public readonly api: IndexerApi;

  constructor(config: ClientConfig) {
    this.request = new FetchHttpRequest(config);
    this.api = new IndexerApi(this.request);
  }
}
