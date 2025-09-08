import type { ApplicationStateOperation, AvmValue, ScratchChange } from "./index";

/**
 * The set of trace information and effect from evaluating a single opcode.
 */
export type SimulationOpcodeTraceUnit = {
  pc: bigint;
  scratchChanges?: ScratchChange[];
  stateChanges?: ApplicationStateOperation[];
  spawnedInners?: bigint[];
  stackPopCount?: bigint;
  stackAdditions?: AvmValue[];
};
