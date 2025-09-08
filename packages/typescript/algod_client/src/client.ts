import type { ClientConfig } from "./core/ClientConfig";
import { FetchHttpRequest } from "./core/FetchHttpRequest";
import { AlgodApi } from "./apis/api.service";

export class AlgodClient {
  public readonly request: FetchHttpRequest;
  public readonly api: AlgodApi;

  constructor(config: ClientConfig) {
    this.request = new FetchHttpRequest(config);
    this.api = new AlgodApi(this.request);
  }
}
