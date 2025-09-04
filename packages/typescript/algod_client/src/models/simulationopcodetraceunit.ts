import type { ApplicationStateOperation, AvmValue, ScratchChange } from "./index";

/**
 * The set of trace information and effect from evaluating a single opcode.
 */
export type SimulationOpcodeTraceUnit = {
  pc: number;
  "scratch-changes"?: ScratchChange[];
  "state-changes"?: ApplicationStateOperation[];
  "spawned-inners"?: number[];
  "stack-pop-count"?: number;
  "stack-additions"?: AvmValue[];
};
