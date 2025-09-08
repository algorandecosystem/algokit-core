import type { SimulateTransactionResult, SimulateUnnamedResourcesAccessed } from "./index";

/**
 * Simulation result for an atomic transaction group
 */
export type SimulateTransactionGroupResult = {
  txnResults: SimulateTransactionResult[];
  failureMessage?: string;
  failedAt?: bigint[];
  appBudgetAdded?: bigint;
  appBudgetConsumed?: bigint;
  unnamedResourcesAccessed?: SimulateUnnamedResourcesAccessed;
};
