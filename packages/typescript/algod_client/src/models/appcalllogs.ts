/**
 * The logged messages from an app call along with the app ID and outer transaction ID. Logs appear in the same order that they were emitted.
 */
export type AppCallLogs = { logs: string[]; "application-index": bigint; txId: string };
