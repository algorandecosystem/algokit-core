import type { DryrunTxnResult } from "./index";

/**
 * DryrunResponse contains per-txn debug information from a dryrun.
 */
export type TealDryrun = { txns: DryrunTxnResult[]; error: string; "protocol-version": string };
