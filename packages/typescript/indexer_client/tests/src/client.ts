import type { OpenAPIConfig } from "../../src/core/OpenAPI";
import { Client as BaseClient } from "../../src/client";

export * from "../../src/client";

export class Client extends BaseClient {
  constructor(config: OpenAPIConfig) {
    super({ INT_DECODING: config.INT_DECODING ?? "bigint", ...config });
  }
}
