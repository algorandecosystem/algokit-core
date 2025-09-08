import type { Application } from "./index";

/**
 * (empty)
 */
export type SearchForApplications = { applications: Application[]; currentRound: bigint; nextToken?: string };
