import { expect, it } from "bun:test";
import { IndexerClient } from "../src/client";
import { maybeDescribe } from "./config";

maybeDescribe("Indexer searchApplications", (env) => {
  it("searches for applications", async () => {
    const client = new IndexerClient({
      BASE: env.indexerBaseUrl,
      INT_DECODING: "bigint",
      HEADERS: env.indexerApiToken ? { "X-Algo-API-Token": env.indexerApiToken } : undefined,
    });

    const res = await client.searchForApplications({ limit: 5 });

    expect(res).toHaveProperty("applications");
    expect(Array.isArray(res.applications)).toBe(true);

    if (res.applications && res.applications.length > 0) {
      const app = res.applications[0];
      expect(app).toHaveProperty("id");
      expect(app).toHaveProperty("params");

      const params = app.params as any;
      expect(params).toHaveProperty("creator");
      expect(params).toHaveProperty("approvalProgram");
      expect(params).toHaveProperty("clearStateProgram");
    }
  }, 30_000);

  it("searches for a specific application by ID", async () => {
    const client = new IndexerClient({
      BASE: env.indexerBaseUrl,
      INT_DECODING: "bigint",
      HEADERS: env.indexerApiToken ? { "X-Algo-API-Token": env.indexerApiToken } : undefined,
    });

    const searchRes = await client.searchForApplications({ limit: 1 });

    if (searchRes.applications && searchRes.applications.length > 0) {
      const appId = searchRes.applications[0].id;

      const res = await client.searchForApplications({ applicationId: appId });

      expect(res).toHaveProperty("applications");
      expect(res.applications).toHaveLength(1);
      expect(res.applications![0].id).toBe(appId);
    } else {
      console.log("No applications found in indexer, skipping specific ID test");
    }
  }, 30_000);

  it("searches applications with pagination", async () => {
    const client = new IndexerClient({
      BASE: env.indexerBaseUrl,
      INT_DECODING: "bigint",
      HEADERS: env.indexerApiToken ? { "X-Algo-API-Token": env.indexerApiToken } : undefined,
    });

    const firstPage = await client.searchForApplications({ limit: 2 });

    expect(firstPage).toHaveProperty("applications");

    if (firstPage["nextToken"]) {
      const secondPage = await client.searchForApplications({
        limit: 2,
        next: firstPage["nextToken"] as string,
      });

      expect(secondPage).toHaveProperty("applications");

      if (firstPage.applications?.length && secondPage.applications?.length) {
        const firstIds = firstPage.applications.map((a) => a.id);
        const secondIds = secondPage.applications.map((a) => a.id);

        expect(firstIds).not.toEqual(secondIds);
      }
    }
  }, 30_000);
});
