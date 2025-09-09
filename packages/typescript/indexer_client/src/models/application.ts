import type { ApplicationParams } from "./index";

/**
 * Application index and its parameters
 */
export type Application = {
  /**
   * application index.
   */
  id: bigint;

  /**
   * Whether or not this application is currently deleted.
   */
  deleted?: boolean;

  /**
   * Round when this application was created.
   */
  createdAtRound?: bigint;

  /**
   * Round when this application was deleted.
   */
  deletedAtRound?: bigint;
  params: ApplicationParams;
};
