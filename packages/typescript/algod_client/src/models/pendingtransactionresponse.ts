import type { AccountStateDelta, StateDelta } from "./index";

/**
 * Details about a pending transaction. If the transaction was recently confirmed, includes confirmation details like the round and reward details.
 */
export type PendingTransactionResponse = {
  assetIndex?: bigint;
  applicationIndex?: bigint;
  closeRewards?: bigint;
  closingAmount?: bigint;
  assetClosingAmount?: bigint;
  confirmedRound?: bigint;
  poolError: string;
  receiverRewards?: bigint;
  senderRewards?: bigint;
  localStateDelta?: AccountStateDelta[];
  globalStateDelta?: StateDelta;
  logs?: string[];
  innerTxns?: PendingTransactionResponse[];
  txn: {};
};
