import type { AccountStateDelta, StateDelta } from "./index";

/**
 * Details about a pending transaction. If the transaction was recently confirmed, includes confirmation details like the round and reward details.
 */
export type PendingTransactionResponse = {
  "asset-index"?: bigint;
  "application-index"?: bigint;
  "close-rewards"?: number;
  "closing-amount"?: bigint;
  "asset-closing-amount"?: bigint;
  "confirmed-round"?: bigint;
  "pool-error": string;
  "receiver-rewards"?: bigint;
  "sender-rewards"?: bigint;
  "local-state-delta"?: AccountStateDelta[];
  "global-state-delta"?: StateDelta;
  logs?: string[];
  "inner-txns"?: PendingTransactionResponse[];
  txn: {};
};
