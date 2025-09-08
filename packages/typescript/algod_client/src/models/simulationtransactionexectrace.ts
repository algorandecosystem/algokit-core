import type { SimulationOpcodeTraceUnit } from "./index";

/**
 * The execution trace of calling an app or a logic sig, containing the inner app call trace in a recursive way.
 */
export type SimulationTransactionExecTrace = {
  approvalProgramTrace?: SimulationOpcodeTraceUnit[];
  approvalProgramHash?: string;
  clearStateProgramTrace?: SimulationOpcodeTraceUnit[];
  clearStateProgramHash?: string;
  clearStateRollback?: boolean;
  clearStateRollbackError?: string;
  logicSigTrace?: SimulationOpcodeTraceUnit[];
  logicSigHash?: string;
  innerTrace?: SimulationTransactionExecTrace[];
};
