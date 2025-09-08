import type { ApplicationParams } from "./index";

/**
 * Application index and its parameters
 */
export type Application = { id: bigint; deleted?: boolean; createdAtRound?: bigint; deletedAtRound?: bigint; params: ApplicationParams };
