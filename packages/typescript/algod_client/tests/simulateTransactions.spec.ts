import { expect, it } from "bun:test";
import { AlgodClient, ClientConfig, SimulateRequest } from "../src";
import { getSenderMnemonic, maybeDescribe } from "./config";
import algosdk from "algosdk";

maybeDescribe("simulateTransactions", (env) => {
  it("should simulate two transactions and decode msgpack response", async () => {
    const client = new AlgodClient({
      BASE: env.algodBaseUrl,
      HEADERS: { "X-Algo-API-Token": env.algodApiToken ?? "" },
      INT_DECODING: "bigint",
    } as ClientConfig);

    const mnemonic = env.senderMnemonic ?? (await getSenderMnemonic());
    const acct = algosdk.mnemonicToSecretKey(mnemonic);
    let sp = await client.transactionParams();

    // Create two transactions similar to Rust test
    // Transaction 1: Simple payment
    const txn1 = algosdk.makePaymentTxnWithSuggestedParamsFromObject({
      sender: acct.addr,
      receiver: acct.addr,
      amount: 100000, // 0.1 ALGO
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

    // Transaction 2: Payment with note (matching Rust test)
    sp = await client.transactionParams();
    const txn2 = algosdk.makePaymentTxnWithSuggestedParamsFromObject({
      sender: acct.addr,
      receiver: acct.addr,
      amount: 100000, // 0.1 ALGO
      note: new TextEncoder().encode("0aa50d27-b8f7-4d77-a1fb-551fd55df2bc"),
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

    // Create simulate request matching Rust structure
    const simulateRequest: SimulateRequest = {
      txnGroups: [
        {
          txns: [
            {
              txn: JSON.parse(algosdk.encodeJSON(txn1)),
            },
            {
              txn: JSON.parse(algosdk.encodeJSON(txn2)),
            },
          ],
        },
      ],
      allowEmptySignatures: true,
      allowMoreLogging: true,
      allowUnnamedResources: true,
      extraOpcodeBudget: 1000n,
      execTraceConfig: {
        enable: true,
        stackChange: true,
        scratchChange: true,
        stateChange: true,
      },
      fixSigners: true,
    };

    const res = await client.simulateTransaction({ format: "msgpack", body: simulateRequest });
    expect(res.txnGroups.length).toBe(1);
    expect(res.txnGroups[0].txnResults.length).toBe(2);
  }, 20000);
});
