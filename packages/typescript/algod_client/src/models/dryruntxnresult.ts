import type { AccountStateDelta, DryrunState, StateDelta } from "./index";

/**
 * DryrunTxnResult contains any LogicSig or ApplicationCall program debug information and state updates from a dryrun.
 */
export type DryrunTxnResult = {
  disassembly: string[];
  "logic-sig-disassembly"?: string[];
  "logic-sig-trace"?: DryrunState[];
  "logic-sig-messages"?: string[];
  "app-call-trace"?: DryrunState[];
  "app-call-messages"?: string[];
  "global-delta"?: StateDelta;
  "local-deltas"?: AccountStateDelta[];
  logs?: string[];
  "budget-added"?: number;
  "budget-consumed"?: number;
};
