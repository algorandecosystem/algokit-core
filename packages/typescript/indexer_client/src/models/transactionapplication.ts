import type { BoxReference, OnCompletion, StateSchema } from "./index";

/**
 * Fields for application transactions.
 *
 * Definition:
 * data/transactions/application.go : ApplicationCallTxnFields
 */
export type TransactionApplication = {
  "application-id": number;
  "on-completion": OnCompletion;
  "application-args"?: string[];
  accounts?: string[];
  "box-references"?: BoxReference[];
  "foreign-apps"?: number[];
  "foreign-assets"?: number[];
  "local-state-schema"?: StateSchema;
  "global-state-schema"?: StateSchema;
  "approval-program"?: string;
  "clear-state-program"?: string;
  "extra-program-pages"?: number;
  "reject-version"?: number;
};
