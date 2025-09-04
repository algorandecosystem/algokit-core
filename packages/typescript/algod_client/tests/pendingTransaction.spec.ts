import { expect, it } from "vitest";
import algosdk from "algosdk";
import { Client } from "../src/client";
import { getSenderMnemonic, maybeDescribe } from "./helpers/env";
import { waitFor } from "./helpers/wait";

maybeDescribe("Algod pendingTransaction", (env) => {
  it("submits a payment tx and queries pending info", async () => {
    const client = new Client({
      BASE: env.algodBaseUrl,
      HEADERS: env.algodApiToken ? { "X-Algo-API-Token": env.algodApiToken } : undefined,
    });

    const mnemonic = env.senderMnemonic ?? (await getSenderMnemonic());
    const acct = algosdk.mnemonicToSecretKey(mnemonic);
    const sp = await client.api.transactionParams();

    // Build simple self-payment of 0 microalgos (allowed) as a noop
    const txn = algosdk.makePaymentTxnWithSuggestedParamsFromObject({
      from: acct.addr,
      to: acct.addr,
      amount: 0,
      suggestedParams: {
        fee: Number(sp["min-fee"]),
        firstRound: Number(sp["last-round"]),
        flatFee: true,
        lastRound: Number(sp["last-round"]) + 1000,
        genesisHash: sp["genesis-hash"] as string,
        genesisID: sp["genesis-id"] as string,
      },
    });

    const signed = txn.signTxn(acct.sk);
    const sendResult = await client.api.rawTransaction({ body: signed });
    const txId = sendResult.txId as string;

    const pending = await waitFor(
      async () => client.api.pendingTransactionInformation(txId, { format: "json" }),
      (r) => !!(r as any)["confirmed-round"] || !!(r as any)["pool-error"],
      { timeoutMs: 45_000, intervalMs: 1_000 },
    );

    // Some nodes may omit 'txid' in pending response; require txn presence
    expect(pending).toHaveProperty("txn");
  }, 30_000);
});
