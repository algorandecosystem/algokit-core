import type { BoxReference, OnCompletion, StateSchema } from "./index";

/**
 * Fields for application transactions.
 *
 * Definition:
 * data/transactions/application.go : ApplicationCallTxnFields
 */
export type TransactionApplication = {
  applicationId: bigint;
  onCompletion: OnCompletion;
  applicationArgs?: string[];
  accounts?: string[];
  boxReferences?: BoxReference[];
  foreignApps?: bigint[];
  foreignAssets?: bigint[];
  localStateSchema?: StateSchema;
  globalStateSchema?: StateSchema;
  approvalProgram?: string;
  clearStateProgram?: string;
  extraProgramPages?: bigint;
  rejectVersion?: bigint;
};
