import type { PendingTransactionResponse, SimulateUnnamedResourcesAccessed, SimulationTransactionExecTrace } from "./index";

/**
 * Simulation result for an individual transaction
 */
export type SimulateTransactionResult = {
  txnResult: PendingTransactionResponse;
  appBudgetConsumed?: bigint;
  logicSigBudgetConsumed?: bigint;
  execTrace?: SimulationTransactionExecTrace;
  unnamedResourcesAccessed?: SimulateUnnamedResourcesAccessed;
  fixedSigner?: string;
};
