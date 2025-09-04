import type { ApplicationParams } from "./index";

/**
 * Application index and its parameters
 */
export type Application = {
  id: bigint;
  deleted?: boolean;
  "created-at-round"?: bigint;
  "deleted-at-round"?: bigint;
  params: ApplicationParams;
};
