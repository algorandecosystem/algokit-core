import { expect, it } from "bun:test";
import algosdk from "algosdk";
import { AlgodClient } from "../src/client";
import { getSenderMnemonic, maybeDescribe } from "./config";
import { PendingTransactionResponse } from "../src/models";

maybeDescribe("Algod pendingTransaction", (env) => {
  it("submits a payment tx and queries pending info", async () => {
    const client = new AlgodClient({
      BASE: env.algodBaseUrl,
      INT_DECODING: "bigint",
      HEADERS: env.algodApiToken ? { "X-Algo-API-Token": env.algodApiToken } : undefined,
    });

    const mnemonic = env.senderMnemonic ?? (await getSenderMnemonic());
    const acct = algosdk.mnemonicToSecretKey(mnemonic);
    const sp = await client.api.transactionParams();

    // Build simple self-payment of 0 microalgos (allowed) as a noop
    const txn = algosdk.makePaymentTxnWithSuggestedParamsFromObject({
      sender: acct.addr,
      receiver: acct.addr,
      amount: 0,
      suggestedParams: {
        minFee: Number(sp["minFee"]),
        fee: Number(sp["minFee"]),
        firstValid: Number(sp["lastRound"]),
        flatFee: true,
        lastValid: Number(sp["lastRound"]) + 1000,
        genesisHash: algosdk.base64ToBytes(sp["genesisHash"]),
        genesisID: sp["genesisId"] as string,
      },
    });

    const signed = txn.signTxn(acct.sk);
    const sendResult = await client.api.rawTransaction({ body: signed });
    const txId = sendResult.txId as string;

    let pending: PendingTransactionResponse | undefined;
    const maxAttempts = 10;
    for (let i = 0; i < maxAttempts; i++) {
      pending = await client.api.pendingTransactionInformation(txId, { format: "msgpack" });
      if (pending?.confirmedRound || pending?.poolError) {
        break;
      }
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
    if (!pending) {
      throw new Error("Transaction confirmation timeout");
    }

    // Some nodes may omit 'txid' in pending response; require txn presence
    expect(pending).toHaveProperty("txn");
  }, 30_000);
});
