# indexer_client

TypeScript client for indexer interaction.

## Usage

Install dependencies and build:

- bun install (or pnpm/yarn)
- bun run build

### Configure base URL and create a client

```ts
import { FetchHttpRequest, type ClientConfig, IndexerClient, IndexerApi } from "./dist/index.js";

const client = new AlgodClient({
  BASE: env.algodBaseUrl,
  HEADERS: env.algodApiToken ? { "X-Algo-API-Token": env.algodApiToken } : undefined,
});

// client is now ready to use
```

To inject a custom HTTP implementation, implement your own request class extending `BaseHttpRequest` and pass it into your own wrapper if needed.
