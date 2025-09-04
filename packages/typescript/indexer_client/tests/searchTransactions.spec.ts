import { expect, it } from "vitest";
import { Client } from "../src/client";
import { maybeDescribe } from "./helpers/env";

maybeDescribe("Indexer searchTransactions", (env) => {
  it("searches for recent transactions", async () => {
    const client = new Client({
      BASE: env.indexerBaseUrl,
      HEADERS: env.indexerApiToken ? { "X-Algo-API-Token": env.indexerApiToken } : undefined,
    });
    const res = await client.api.searchForTransactions({ limit: 5 });
    expect(res).toHaveProperty("transactions");
    expect(Array.isArray(res.transactions)).toBe(true);
  });
});
