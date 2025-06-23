import { expect, test, describe } from "bun:test";
import { testData } from "./common.ts";
import * as ed from "@noble/ed25519";
import {
  encodeTransaction,
  decodeTransaction,
  getEncodedTransactionType,
  Transaction,
  addressFromPubKey,
  getTransactionIdRaw,
  getTransactionId,
  assignFee,
  SignedTransaction,
  encodeSignedTransaction,
} from "..";

const onlineKeyRegistration = testData.onlineKeyRegistration;
const offlineKeyRegistration = testData.offlineKeyRegistration;
const nonParticipatingKeyRegistration = testData.nonParticipatingKeyRegistration;

describe("Key Registration", () => {
  // Polytest Suite: Key Registration

  describe("Transaction Tests", () => {
    // Polytest Group: Transaction Tests

    test("example", async () => {
      const aliceSk = ed.utils.randomPrivateKey();
      const alicePubKey = await ed.getPublicKeyAsync(aliceSk);
      const alice = addressFromPubKey(alicePubKey);

      // Example 1: Online key registration
      const onlineTxn: Transaction = {
        transactionType: "KeyRegistration",
        sender: alice,
        firstValid: 1337n,
        lastValid: 1347n,
        genesisHash: new Uint8Array(32).fill(65), // pretend this is a valid hash
        genesisId: "localnet",
        keyRegistration: {
          voteKey: new Uint8Array(32).fill(1), // 32-byte participation key
          selectionKey: new Uint8Array(32).fill(2), // 32-byte VRF key
          stateProofKey: new Uint8Array(64).fill(3), // 64-byte state proof key
          voteFirst: 100n,
          voteLast: 1000n,
          voteKeyDilution: 1000n,
        },
      };

      const onlineTxnWithFee = assignFee(onlineTxn, { feePerByte: 0n, minFee: 1000n });
      expect(onlineTxnWithFee.fee).toBe(1000n);

      // Example 2: Offline key registration (going offline)
      const offlineTxn: Transaction = {
        transactionType: "KeyRegistration",
        sender: alice,
        firstValid: 1337n,
        lastValid: 1347n,
        genesisHash: new Uint8Array(32).fill(65),
        genesisId: "localnet",
        keyRegistration: {}, // All fields undefined for offline
      };

      const offlineTxnWithFee = assignFee(offlineTxn, { feePerByte: 0n, minFee: 1000n });
      expect(offlineTxnWithFee.fee).toBe(1000n);

      // Example 3: Non-participating account
      const nonPartTxn: Transaction = {
        transactionType: "KeyRegistration",
        sender: alice,
        firstValid: 1337n,
        lastValid: 1347n,
        genesisHash: new Uint8Array(32).fill(65),
        genesisId: "localnet",
        keyRegistration: {
          nonParticipation: true, // Mark as non-participating for no rewards
        },
      };

      const nonPartTxnWithFee = assignFee(nonPartTxn, { feePerByte: 0n, minFee: 1000n });
      expect(nonPartTxnWithFee.fee).toBe(1000n);
    });

    test("get transaction id", () => {
      // Test with online key registration
      expect(getTransactionId(onlineKeyRegistration.transaction)).toBe(onlineKeyRegistration.id);
      expect(getTransactionIdRaw(onlineKeyRegistration.transaction)).toEqual(onlineKeyRegistration.idRaw);

      // Test with offline key registration
      expect(getTransactionId(offlineKeyRegistration.transaction)).toBe(offlineKeyRegistration.id);
      expect(getTransactionIdRaw(offlineKeyRegistration.transaction)).toEqual(offlineKeyRegistration.idRaw);
    });

    test("get encoded transaction type", () => {
      const onlineEncoded = encodeTransaction(onlineKeyRegistration.transaction);
      expect(getEncodedTransactionType(onlineEncoded)).toBe("KeyRegistration");

      const offlineEncoded = encodeTransaction(offlineKeyRegistration.transaction);
      expect(getEncodedTransactionType(offlineEncoded)).toBe("KeyRegistration");
    });

    test("decode without prefix", () => {
      // Remove TX prefix (first 2 bytes) and decode
      const onlineDecoded = decodeTransaction(onlineKeyRegistration.unsignedBytes.slice(2));
      expect(onlineDecoded).toEqual(onlineKeyRegistration.transaction);

      const offlineDecoded = decodeTransaction(offlineKeyRegistration.unsignedBytes.slice(2));
      expect(offlineDecoded).toEqual(offlineKeyRegistration.transaction);
    });

    test("decode with prefix", () => {
      // Decode with TX prefix
      const onlineDecoded = decodeTransaction(onlineKeyRegistration.unsignedBytes);
      expect(onlineDecoded).toEqual(onlineKeyRegistration.transaction);

      const offlineDecoded = decodeTransaction(offlineKeyRegistration.unsignedBytes);
      expect(offlineDecoded).toEqual(offlineKeyRegistration.transaction);
    });

    test("encode with auth address", async () => {
      const signature = await ed.signAsync(onlineKeyRegistration.unsignedBytes, onlineKeyRegistration.signingPrivateKey);

      const signedTxn: SignedTransaction = {
        transaction: onlineKeyRegistration.transaction,
        signature: signature,
        authAddress: onlineKeyRegistration.rekeyedSenderAuthAddress,
      };

      const encoded = encodeSignedTransaction(signedTxn);
      expect(encoded).toEqual(onlineKeyRegistration.rekeyedSenderSignedBytes);
    });

    test("encode with signature", async () => {
      const signature = await ed.signAsync(onlineKeyRegistration.unsignedBytes, onlineKeyRegistration.signingPrivateKey);

      const signedTxn: SignedTransaction = {
        transaction: onlineKeyRegistration.transaction,
        signature: signature,
      };

      const encoded = encodeSignedTransaction(signedTxn);
      expect(encoded).toEqual(onlineKeyRegistration.signedBytes);
    });

    test("encode", () => {
      // Test online key registration encoding
      const onlineEncoded = encodeTransaction(onlineKeyRegistration.transaction);
      expect(onlineEncoded).toEqual(onlineKeyRegistration.unsignedBytes);

      // Test offline key registration encoding
      const offlineEncoded = encodeTransaction(offlineKeyRegistration.transaction);
      expect(offlineEncoded).toEqual(offlineKeyRegistration.unsignedBytes);

      // Test non-participating key registration encoding
      const nonPartEncoded = encodeTransaction(nonParticipatingKeyRegistration.transaction);
      expect(nonPartEncoded).toEqual(nonParticipatingKeyRegistration.unsignedBytes);
    });
  });
});
