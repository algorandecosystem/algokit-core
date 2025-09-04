/**
 * The set of parameters and limits override during simulation. If this set of parameters is present, then evaluation parameters may differ from standard evaluation in certain ways.
 */
export type SimulationEvalOverrides = {
  "allow-empty-signatures"?: boolean;
  "allow-unnamed-resources"?: boolean;
  "max-log-calls"?: number;
  "max-log-size"?: number;
  "extra-opcode-budget"?: number;
  "fix-signers"?: boolean;
};
