import type {
  AccountStateDelta,
  StateDelta,
  TransactionApplication,
  TransactionAssetConfig,
  TransactionAssetFreeze,
  TransactionAssetTransfer,
  TransactionHeartbeat,
  TransactionKeyreg,
  TransactionPayment,
  TransactionSignature,
  TransactionStateProof,
} from "./index";

/**
 * Contains all fields common to all transactions and serves as an envelope to all transactions type. Represents both regular and inner transactions.
 *
 * Definition:
 * data/transactions/signedtxn.go : SignedTxn
 * data/transactions/transaction.go : Transaction
 */
export type Transaction = {
  "application-transaction"?: TransactionApplication;
  "asset-config-transaction"?: TransactionAssetConfig;
  "asset-freeze-transaction"?: TransactionAssetFreeze;
  "asset-transfer-transaction"?: TransactionAssetTransfer;
  "state-proof-transaction"?: TransactionStateProof;
  "heartbeat-transaction"?: TransactionHeartbeat;
  "auth-addr"?: string;
  "close-rewards"?: number;
  "closing-amount"?: bigint;
  "confirmed-round"?: bigint;
  "created-application-index"?: number;
  "created-asset-index"?: number;
  fee: bigint;
  "first-valid": number;
  "genesis-hash"?: string;
  "genesis-id"?: string;
  group?: string;
  id?: string;
  "intra-round-offset"?: number;
  "keyreg-transaction"?: TransactionKeyreg;
  "last-valid": number;
  lease?: string;
  note?: string;
  "payment-transaction"?: TransactionPayment;
  "receiver-rewards"?: number;
  "rekey-to"?: string;
  "round-time"?: number;
  sender: string;
  "sender-rewards"?: number;
  signature?: TransactionSignature;
  "tx-type": "pay" | "keyreg" | "acfg" | "axfer" | "afrz" | "appl" | "stpf" | "hb";
  "local-state-delta"?: AccountStateDelta[];
  "global-state-delta"?: StateDelta;
  logs?: string[];
  "inner-txns"?: Transaction[];
};
