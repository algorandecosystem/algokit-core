import type { ApplicationLogData } from "./index";

/**
 * (empty)
 */
export type LookupApplicationLogsById = { applicationId: bigint; currentRound: bigint; nextToken?: string; logData?: ApplicationLogData[] };
