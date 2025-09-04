/**
 * DryrunSource is TEAL source text that gets uploaded, compiled, and inserted into transactions or application state.
 */
export type DryrunSource = { "field-name": string; source: string; "txn-index": number; "app-index": bigint };
