import type { AppCallLogs } from "./index";

/**
 * All logs emitted in the given round. Each app call, whether top-level or inner, that contains logs results in a separate AppCallLogs object. Therefore there may be multiple AppCallLogs with the same application ID and outer transaction ID in the event of multiple inner app calls to the same app. App calls with no logs are not included in the response. AppCallLogs are returned in the same order that their corresponding app call appeared in the block (pre-order traversal of inner app calls)
 */
export type GetBlockLogs = { logs: AppCallLogs[] };
