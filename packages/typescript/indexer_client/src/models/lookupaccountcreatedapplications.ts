import type { Application } from "./index";

/**
 * (empty)
 */
export type LookupAccountCreatedApplications = { applications: Application[]; "current-round": number; "next-token"?: string };
