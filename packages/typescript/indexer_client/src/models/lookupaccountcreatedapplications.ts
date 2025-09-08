import type { Application } from "./index";

/**
 * (empty)
 */
export type LookupAccountCreatedApplications = { applications: Application[]; currentRound: bigint; nextToken?: string };
