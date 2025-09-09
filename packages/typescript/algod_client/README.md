# algod_client

TypeScript client for algod interaction.

## Usage

Install dependencies and build:

- bun install (or pnpm/yarn)
- bun run build

### Configure base URL and create a client

```ts
import { AlgodClient } from "./dist/index.js";

const client = new AlgodClient({
  BASE: env.baseUrl,
  HEADERS: env.apiToken ? { "X-Algo-API-Token": env.apiToken } : undefined,
});

// client is now ready to use
```

To inject a custom HTTP implementation, implement your own request class extending `BaseHttpRequest` and pass it into your own wrapper if needed.
