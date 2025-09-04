import type { SimulateTransactionResult, SimulateUnnamedResourcesAccessed } from "./index";

/**
 * Simulation result for an atomic transaction group
 */
export type SimulateTransactionGroupResult = {
  "txn-results": SimulateTransactionResult[];
  "failure-message"?: string;
  "failed-at"?: number[];
  "app-budget-added"?: number;
  "app-budget-consumed"?: number;
  "unnamed-resources-accessed"?: SimulateUnnamedResourcesAccessed;
};
