import { describe, expect, it } from "vitest";
import { Client } from "../src/client";

const BASE = "http://localhost/";

declare global {
  // eslint-disable-next-line no-var, vars-on-top
  var fetch: any;
}

describe("JSON BigInt parsing", () => {
  it("parses bigint values using json-bigint", async () => {
    const big = "18446744073709551615"; // > Number.MAX_SAFE_INTEGER
    global.fetch = async () => ({
      ok: true,
      headers: new Map([["content-type", "application/json"]]),
      text: async () => JSON.stringify({ value: big }),
      arrayBuffer: async () => new ArrayBuffer(0),
    });

    const client = new Client({ BASE, INT_DECODING: "bigint" });
    // Directly use request core to keep template-agnostic
    const res = await client.request.request<{ value: bigint }>({ method: "GET", url: "/foo" });
    expect(typeof res.value).toBe("bigint");
  });
});

describe("Msgpack binary response", () => {
  it("decodes msgpack into structured data", async () => {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const { encode } = require("algo-msgpack-with-bigint");
    const payload = { ok: 1n, text: "hi" };
    const bytes: Uint8Array = encode(payload);
    global.fetch = async () => ({
      ok: true,
      headers: new Map([["content-type", "application/msgpack"]]),
      text: async () => "",
      arrayBuffer: async () => bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength),
    });

    const client = new Client({ BASE });
    const res = await client.request.request<{ ok: bigint | number; text: string }>({ method: "GET", url: "/foo", expectBinary: true });
    expect(res).toHaveProperty("ok");
    expect(res).toHaveProperty("text", "hi");
  });
});
