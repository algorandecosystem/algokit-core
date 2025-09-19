import type { ApplicationStateSchema, TealKeyValueStore } from "./index";

/**
 * Stores local state associated with an application.
 */
export type ApplicationLocalState = {
  /**
   * The application which this local state is for.
   */
  id: bigint;

  /**
   * Whether or not the application local state is currently deleted from its account.
   */
  deleted?: boolean;

  /**
   * Round when the account opted into the application.
   */
  optedInAtRound?: bigint;

  /**
   * Round when account closed out of the application.
   */
  closedOutAtRound?: bigint;
  schema: ApplicationStateSchema;
  keyValue?: TealKeyValueStore;
};
