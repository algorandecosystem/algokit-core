import type { Transaction } from "./index";

export type LookupTransaction = {
  transaction: Transaction;

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint;
};
