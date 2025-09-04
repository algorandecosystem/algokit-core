import type { SimulateInitialStates, SimulateTraceConfig, SimulateTransactionGroupResult, SimulationEvalOverrides } from "./index";

/**
 * Result of a transaction group simulation.
 */
export type SimulateTransaction = {
  version: bigint;
  "last-round": bigint;
  "txn-groups": SimulateTransactionGroupResult[];
  "eval-overrides"?: SimulationEvalOverrides;
  "exec-trace-config"?: SimulateTraceConfig;
  "initial-states"?: SimulateInitialStates;
};
