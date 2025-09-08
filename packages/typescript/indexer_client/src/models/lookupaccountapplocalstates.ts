import type { ApplicationLocalState } from "./index";

/**
 * (empty)
 */
export type LookupAccountAppLocalStates = { appsLocalStates: ApplicationLocalState[]; currentRound: bigint; nextToken?: string };
