/**
 * DryrunSource is TEAL source text that gets uploaded, compiled, and inserted into transactions or application state.
 */
export type DryrunSource = { fieldName: string; source: string; txnIndex: bigint; appIndex: bigint };
