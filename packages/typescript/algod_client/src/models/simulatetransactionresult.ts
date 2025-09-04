import type { PendingTransactionResponse, SimulateUnnamedResourcesAccessed, SimulationTransactionExecTrace } from "./index";

/**
 * Simulation result for an individual transaction
 */
export type SimulateTransactionResult = {
  "txn-result": PendingTransactionResponse;
  "app-budget-consumed"?: number;
  "logic-sig-budget-consumed"?: number;
  "exec-trace"?: SimulationTransactionExecTrace;
  "unnamed-resources-accessed"?: SimulateUnnamedResourcesAccessed;
  "fixed-signer"?: string;
};
