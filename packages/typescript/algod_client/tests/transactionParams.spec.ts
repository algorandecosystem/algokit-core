import { expect, it, describe } from "vitest";
import { Client } from "../src/client";
import { maybeDescribe } from "./helpers/env";

maybeDescribe("Algod transactionParams", (env) => {
  it("fetches suggested params", async () => {
    const client = new Client({
      BASE: env.algodBaseUrl,
      INT_DECODING: "bigint",
      HEADERS: env.algodApiToken ? { "X-Algo-API-Token": env.algodApiToken } : undefined,
    });
    const params = await client.api.transactionParams();
    // Basic shape checks
    expect(params).toHaveProperty("genesis-id");
    expect(typeof params["last-round"]).toBe("bigint");
    expect(typeof params["min-fee"]).toBe("bigint");
  });
});
