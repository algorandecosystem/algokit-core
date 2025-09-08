import type { SimulateInitialStates, SimulateTraceConfig, SimulateTransactionGroupResult, SimulationEvalOverrides } from "./index";

/**
 * Result of a transaction group simulation.
 */
export type SimulateTransaction = {
  version: bigint;
  lastRound: bigint;
  txnGroups: SimulateTransactionGroupResult[];
  evalOverrides?: SimulationEvalOverrides;
  execTraceConfig?: SimulateTraceConfig;
  initialStates?: SimulateInitialStates;
};
