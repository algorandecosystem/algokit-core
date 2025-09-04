import { describe } from "vitest";

export interface IndexerEnvConfig {
  indexerBaseUrl: string;
  indexerApiToken?: string;
}

export function getIndexerEnv(): IndexerEnvConfig {
  return {
    indexerBaseUrl: process.env.INDEXER_BASE_URL ?? "http://localhost:8980",
    // Default token for localnet (if required by reverse proxy)
    indexerApiToken: process.env.INDEXER_API_TOKEN ?? "a".repeat(64),
  };
}

export function maybeDescribe(name: string, fn: (env: IndexerEnvConfig) => void) {
  describe(name, () => fn(getIndexerEnv()));
}
