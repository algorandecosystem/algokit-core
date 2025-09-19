import type { Account } from "./index";

export type LookupAccountById = {
  account: Account;

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint;
};
