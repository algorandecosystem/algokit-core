import { expect, it } from "bun:test";
import { IndexerClient } from "../src/client";
import { maybeDescribe } from "./config";

maybeDescribe("Indexer searchTransactions", (env) => {
  it("searches for recent transactions", async () => {
    const client = new IndexerClient({
      BASE: env.indexerBaseUrl,
      INT_DECODING: "bigint",
      HEADERS: env.indexerApiToken ? { "X-Algo-API-Token": env.indexerApiToken } : undefined,
    });
    const res = await client.searchForTransactions({ limit: 5 });
    expect(res).toHaveProperty("transactions");
    expect(Array.isArray(res.transactions)).toBe(true);
  });
});
