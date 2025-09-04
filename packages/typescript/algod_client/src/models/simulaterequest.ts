import type { SimulateRequestTransactionGroup, SimulateTraceConfig } from "./index";

/**
 * Request type for simulation endpoint.
 */
export type SimulateRequest = {
  "txn-groups": SimulateRequestTransactionGroup[];
  round?: bigint;
  "allow-empty-signatures"?: boolean;
  "allow-more-logging"?: boolean;
  "allow-unnamed-resources"?: boolean;
  "extra-opcode-budget"?: number;
  "exec-trace-config"?: SimulateTraceConfig;
  "fix-signers"?: boolean;
};
