import type { ApplicationLogData } from "./index";

/**
 * (empty)
 */
export type LookupApplicationLogsById = {
  "application-id": number;
  "current-round": number;
  "next-token"?: string;
  "log-data"?: ApplicationLogData[];
};
