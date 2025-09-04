import type { SimulationOpcodeTraceUnit } from "./index";

/**
 * The execution trace of calling an app or a logic sig, containing the inner app call trace in a recursive way.
 */
export type SimulationTransactionExecTrace = {
  "approval-program-trace"?: SimulationOpcodeTraceUnit[];
  "approval-program-hash"?: string;
  "clear-state-program-trace"?: SimulationOpcodeTraceUnit[];
  "clear-state-program-hash"?: string;
  "clear-state-rollback"?: boolean;
  "clear-state-rollback-error"?: string;
  "logic-sig-trace"?: SimulationOpcodeTraceUnit[];
  "logic-sig-hash"?: string;
  "inner-trace"?: SimulationTransactionExecTrace[];
};
