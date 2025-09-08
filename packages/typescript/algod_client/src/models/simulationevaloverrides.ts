/**
 * The set of parameters and limits override during simulation. If this set of parameters is present, then evaluation parameters may differ from standard evaluation in certain ways.
 */
export type SimulationEvalOverrides = {
  allowEmptySignatures?: boolean;
  allowUnnamedResources?: boolean;
  maxLogCalls?: bigint;
  maxLogSize?: bigint;
  extraOpcodeBudget?: bigint;
  fixSigners?: boolean;
};
