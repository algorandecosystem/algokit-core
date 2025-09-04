import type { OpenAPIConfig } from "../../src/core/OpenAPI";
import { Client as BaseClient } from "../../src/client";

// Re-export everything first
export * from "../../src/client";

// Override Client to default INT_DECODING to 'bigint' for generated smoke tests
export class Client extends BaseClient {
  constructor(config: OpenAPIConfig) {
    super({ INT_DECODING: config.INT_DECODING ?? "bigint", ...config });
  }
}
