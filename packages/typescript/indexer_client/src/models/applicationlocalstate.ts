import type { ApplicationStateSchema, TealKeyValueStore } from "./index";

/**
 * Stores local state associated with an application.
 */
export type ApplicationLocalState = {
  id: bigint;
  deleted?: boolean;
  optedInAtRound?: bigint;
  closedOutAtRound?: bigint;
  schema: ApplicationStateSchema;
  keyValue?: TealKeyValueStore;
};
