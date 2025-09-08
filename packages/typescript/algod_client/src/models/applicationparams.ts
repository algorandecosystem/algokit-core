import type { ApplicationStateSchema, TealKeyValueStore } from "./index";

/**
 * Stores the global information associated with an application.
 */
export type ApplicationParams = {
  creator: string;
  approvalProgram: string;
  clearStateProgram: string;
  extraProgramPages?: bigint;
  localStateSchema?: ApplicationStateSchema;
  globalStateSchema?: ApplicationStateSchema;
  globalState?: TealKeyValueStore;
  version?: bigint;
};
