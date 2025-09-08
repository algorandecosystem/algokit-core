import type { SimulateRequestTransactionGroup, SimulateTraceConfig } from "./index";

/**
 * Request type for simulation endpoint.
 */
export type SimulateRequest = {
  txnGroups: SimulateRequestTransactionGroup[];
  round?: bigint;
  allowEmptySignatures?: boolean;
  allowMoreLogging?: boolean;
  allowUnnamedResources?: boolean;
  extraOpcodeBudget?: bigint;
  execTraceConfig?: SimulateTraceConfig;
  fixSigners?: boolean;
};
