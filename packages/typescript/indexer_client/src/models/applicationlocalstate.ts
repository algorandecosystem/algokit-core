import type { ApplicationStateSchema, TealKeyValueStore } from "./index";

/**
 * Stores local state associated with an application.
 */
export type ApplicationLocalState = {
  id: bigint;
  deleted?: boolean;
  "opted-in-at-round"?: bigint;
  "closed-out-at-round"?: bigint;
  schema: ApplicationStateSchema;
  "key-value"?: TealKeyValueStore;
};
