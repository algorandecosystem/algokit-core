import type { AvmValue } from "./index";

/**
 * An operation against an application's global/local/box state.
 */
export type ApplicationStateOperation = { operation: string; appStateType: string; key: string; newValue?: AvmValue; account?: string };
