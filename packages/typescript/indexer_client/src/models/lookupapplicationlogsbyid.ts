import type { ApplicationLogData } from "./index";

export type LookupApplicationLogsById = {
  /**
   * \[appidx\] application index.
   */
  applicationId: bigint;

  /**
   * Round at which the results were computed.
   */
  currentRound: bigint;

  /**
   * Used for pagination, when making another request provide this token with the next parameter.
   */
  nextToken?: string;
  logData?: ApplicationLogData[];
};
