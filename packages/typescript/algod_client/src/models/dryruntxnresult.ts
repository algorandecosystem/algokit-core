import type { AccountStateDelta, DryrunState, StateDelta } from "./index";

/**
 * DryrunTxnResult contains any LogicSig or ApplicationCall program debug information and state updates from a dryrun.
 */
export type DryrunTxnResult = {
  disassembly: string[];
  logicSigDisassembly?: string[];
  logicSigTrace?: DryrunState[];
  logicSigMessages?: string[];
  appCallTrace?: DryrunState[];
  appCallMessages?: string[];
  globalDelta?: StateDelta;
  localDeltas?: AccountStateDelta[];
  logs?: string[];
  budgetAdded?: bigint;
  budgetConsumed?: bigint;
};
