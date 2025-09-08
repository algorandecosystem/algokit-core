import type { Account, Application, DryrunSource } from "./index";

/**
 * Request data type for dryrun endpoint. Given the Transactions and simulated ledger state upload, run TEAL scripts and return debugging information.
 */
export type DryrunRequest = {
  txns: string[];
  accounts: Account[];
  apps: Application[];
  protocolVersion: string;
  round: bigint;
  latestTimestamp: bigint;
  sources: DryrunSource[];
};
