import type { Application } from "./index";

/**
 * (empty)
 */
export type SearchForApplications = { applications: Application[]; "current-round": number; "next-token"?: string };
