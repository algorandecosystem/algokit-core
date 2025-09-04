import type { ApplicationStateSchema, TealKeyValueStore } from "./index";

/**
 * Stores the global information associated with an application.
 */
export type ApplicationParams = {
  creator?: string;
  "approval-program"?: string;
  "clear-state-program"?: string;
  "extra-program-pages"?: number;
  "local-state-schema"?: ApplicationStateSchema;
  "global-state-schema"?: ApplicationStateSchema;
  "global-state"?: TealKeyValueStore;
  version?: number;
};
