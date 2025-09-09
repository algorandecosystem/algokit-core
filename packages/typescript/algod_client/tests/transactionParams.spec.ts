import { expect, it } from "bun:test";
import { AlgodClient } from "../src/client";
import { maybeDescribe } from "./config";

maybeDescribe("Algod transactionParams", (env) => {
  it("fetches suggested params", async () => {
    const client = new AlgodClient({
      BASE: env.algodBaseUrl,
      INT_DECODING: "bigint",
      HEADERS: env.algodApiToken ? { "X-Algo-API-Token": env.algodApiToken } : undefined,
    });
    const params = await client.transactionParams();
    expect(params).toHaveProperty("genesisId");
    expect(typeof params["lastRound"]).toBe("bigint");
    expect(typeof params["minFee"]).toBe("bigint");
  });
});
