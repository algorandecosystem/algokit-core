import { describe, test, expect, beforeAll } from "vitest";
import { algorandFixture } from "@algorandfoundation/algokit-utils/testing";
import * as algosdk from "algosdk";
import * as algodPackage from "@algorandfoundation/algokit-algod-api";
import {
  encodeTransaction,
  Transaction,
  TransactionType,
  PaymentTransactionFields, AssetTransferTransactionFields
} from "@algorandfoundation/algokit-transact";
import { Blob } from "buffer";
import { HttpFile } from "../http/http";
import { TransactionSignerAccount } from "@algorandfoundation/algokit-utils/types/account";
import {AlgoAmount} from "@algorandfoundation/algokit-utils/types/amount";
import {PendingTransactionResponse} from "../models/PendingTransactionResponse";

// Helper function to create and sign a transaction, then return it as an HttpFile
const createSignedTxnHttpFile = (
    sender: algosdk.Address & TransactionSignerAccount & algosdk.Account,
    apiSuggestedParams: algodPackage.TransactionParams200Response,
    transactionType: TransactionType,
    payment?: PaymentTransactionFields,
    assetTransfer?: AssetTransferTransactionFields,
    note?: Uint8Array,
): HttpFile => {
  const transactionHeaderParams = {
    fee: BigInt(apiSuggestedParams.minFee || apiSuggestedParams.fee),
    firstValid: BigInt(apiSuggestedParams.lastRound),
    lastValid: BigInt(apiSuggestedParams.lastRound) + BigInt(1000),
    genesisHash: new Uint8Array(Buffer.from(apiSuggestedParams.genesisHash, "base64")),
    genesisId: apiSuggestedParams.genesisId,
  };

  const txn: Transaction = {
    transactionType,
    payment,
    assetTransfer,
    sender: { address: String(sender.addr), pubKey: sender.publicKey },
    note: note,
    ...transactionHeaderParams,
  };

  const encodedTxn = encodeTransaction(txn);
  const algosdkTxn = algosdk.decodeUnsignedTransaction(Buffer.from(encodedTxn.slice(2)));
  const signedTxnBytes = algosdkTxn.signTxn(sender.sk);

  const binaryData = new Blob([signedTxnBytes], { type: "application/x-binary" });
  // @ts-ignore - we know this matches the HttpFile structure
  // TODO: find more elegant way to do this
  const httpFile: HttpFile = Object.assign(binaryData, { name: "transaction.txn" });
  return httpFile;
};

async function waitForConfirmation(
    algodApi: algodPackage.AlgodApi,
    txId: string,
): Promise<PendingTransactionResponse> {
  while (true) {
    const pendingInfo = await algodApi.pendingTransactionInformation(txId);
    if (pendingInfo?.confirmedRound) {
      return pendingInfo;
    }
  }
}

describe("Transaction API Tests", () => {
  const fixture = algorandFixture();
  let algodApi: algodPackage.AlgodApi;

  beforeAll(async () => {
    await fixture.newScope();
    const authConfig: algodPackage.AuthMethodsConfiguration = {
      api_key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    };
    const serverConfig = new algodPackage.ServerConfiguration("http://localhost:4001", {});
    const configurationParameters = {
      httpApi: new algodPackage.IsomorphicFetchHttpLibrary(),
      baseServer: serverConfig,
      authMethods: authConfig,
      promiseMiddleware: [],
    };
    const config = algodPackage.createConfiguration(configurationParameters);
    algodApi = new algodPackage.AlgodApi(config);
  });

  test("should get transaction parameters", async () => {
    const params = await algodApi.transactionParams();
    expect(params).toBeDefined();
    expect(params.consensusVersion).toBeTypeOf("string");
    expect(params.fee).toBeTypeOf("number");
    expect(params.genesisHash).toBeTypeOf("string");
    expect(params.genesisId).toBeTypeOf("string");
    expect(params.lastRound).toBeTypeOf("number");
    expect(params.minFee).toBeTypeOf("number");
  });

  test("should submit a raw transaction", async () => {
    const { testAccount: sender } = fixture.context;
    const suggestedParams = await algodApi.transactionParams();

    const httpFile = createSignedTxnHttpFile(
      sender,
        suggestedParams,
      "Payment",
      {
        amount: BigInt(100000),
        receiver: { address: String(sender.addr), pubKey: sender.publicKey },
      },
    );

    const result = await algodApi.rawTransaction(httpFile);
    expect(result).not.toBeNull();
    expect(result?.txId).toBeDefined();
  });

  test("should get pending transaction info", async () => {
    const { testAccount: sender } = fixture.context;
    const suggestedParams = await algodApi.transactionParams();
    const note = new Uint8Array(Buffer.from("pending test"));

    const httpFile = createSignedTxnHttpFile(
      sender,
      suggestedParams,
      "Payment",
      {
        amount: BigInt(100000),
        receiver: { address: String(sender.addr), pubKey: sender.publicKey },
      },
      undefined,
      note,
    );

    const submitResult = await algodApi.rawTransaction(httpFile);
    expect(submitResult.txId).toBeDefined();

    await new Promise((resolve) => setTimeout(resolve, 100));

    const pendingTxnsResult = await algodApi.getPendingTransactions(10);
    expect(pendingTxnsResult).toBeDefined();
  });

  test.each(["json", "msgpack"])("should simulate transaction with format %s", async (format) => {
    const { testAccount: sender } = fixture.context;

    const suggestedParams = await algodApi.transactionParams();
    const signedTxnFile = createSignedTxnHttpFile(
      sender,
      suggestedParams,
      "Payment",
      {
        amount: BigInt(100000),
        receiver: {address: String(sender.addr), pubKey: sender.publicKey}
      },
    );

    // Note: This implementation differs from algosdk's simulateTransaction method.
    // Here we manually convert the signed transaction to base64-encoded string,
    // while algosdk takes an array of SignedTransaction objects and handles the encoding internally.
    const txnGroup: algodPackage.SimulateRequestTransactionGroup = {
      "txns": [Buffer.from(await signedTxnFile.arrayBuffer()).toString("base64")],
    }

    const traceConfig: algodPackage.SimulateTraceConfig = {
      "enable": true,
      "stackChange": true,
      "scratchChange": true,
      "stateChange": true,
    }

    const simulateRequest: algodPackage.SimulateRequest = {
      "allowEmptySignatures": true,
      "allowMoreLogging": true,
      "allowUnnamedResources": true,
      "txnGroups": [txnGroup],
      "execTraceConfig": traceConfig,
    }

    const result = await algodApi.simulateTransaction(simulateRequest, format as "msgpack" | "json", {
      headers: { "Content-Type": `application/${format}` },
    } as any);

    expect(result).toBeDefined();
  });

  // TODO: Add more tests based on other endpoints in AlgodApi related to transactions

  test('should submit an asset transfer transaction and await confirmation', async () => {
    const { testAccount: sender, generateAccount, algod } = fixture.context;
    const ephemeralAccount = await generateAccount({initialFunds: AlgoAmount.Algo(10)});

    // TODO: Use algokit-transact for the asset create transaction
    const suggestedParamsAssetCreate = await algod.getTransactionParams().do();
    const assetCreate = algosdk.makeAssetCreateTxnWithSuggestedParamsFromObject({
      sender,
      total: 10,
      decimals: 0,
      defaultFrozen: false,
      clawback: sender,
      suggestedParams: suggestedParamsAssetCreate,
    })
    const signedAssetCreate = algosdk.signTransaction(assetCreate, sender.sk)
    await algod.sendRawTransaction(signedAssetCreate.blob).do()
    const { assetIndex } = await algosdk.waitForConfirmation(algod, signedAssetCreate.txID, 10)

    const suggestedParamsAssetOptIn = await algodApi.transactionParams();
    const signedAssetOptIn = createSignedTxnHttpFile(
      ephemeralAccount,
      suggestedParamsAssetOptIn,
        "AssetTransfer",
        undefined,
        {
            assetId: assetIndex!,
            amount: BigInt(0),
            receiver: { address: String(ephemeralAccount.addr), pubKey: ephemeralAccount.publicKey },
        }
    )

    const resultAssetOptIn = await algodApi.rawTransaction(signedAssetOptIn);
    expect(resultAssetOptIn).not.toBeNull();
    expect(resultAssetOptIn?.txId).toBeDefined();
    await waitForConfirmation(algodApi, resultAssetOptIn.txId);

    const suggestedParamsAssetTransfer = await algodApi.transactionParams();
    const signedAssetTransfer = createSignedTxnHttpFile(
      sender,
      suggestedParamsAssetTransfer,
        "AssetTransfer",
        undefined,
        {
            assetId: assetIndex!,
            amount: BigInt(1),
            receiver: { address: String(ephemeralAccount.addr), pubKey: ephemeralAccount.publicKey },
        }
    )

    const resultAssetTransfer = await algodApi.rawTransaction(signedAssetTransfer);
    expect(resultAssetTransfer).not.toBeNull();
    expect(resultAssetTransfer?.txId).toBeDefined();
    await waitForConfirmation(algodApi, resultAssetTransfer.txId);

    const { assetHolding } = await algodApi.accountAssetInformation(ephemeralAccount.toString(), Number(assetIndex!))
    expect(assetHolding).not.toBeNull()
    expect(assetHolding?.amount).toBe(1);
  });
});
