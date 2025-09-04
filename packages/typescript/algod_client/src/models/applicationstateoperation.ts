import type { AvmValue } from "./index";

/**
 * An operation against an application's global/local/box state.
 */
export type ApplicationStateOperation = {
  operation: string;
  "app-state-type": string;
  key: string;
  "new-value"?: AvmValue;
  account?: string;
};
