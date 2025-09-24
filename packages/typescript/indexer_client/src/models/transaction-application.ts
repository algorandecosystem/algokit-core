import type { BoxReference, OnCompletion, ResourceRef, StateSchema } from './index'
/**
 * Fields for application transactions.
 *
 * Definition:
 * data/transactions/application.go : ApplicationCallTxnFields
 */
export type TransactionApplication = {
  /**
   * \[apid\] ID of the application being configured or empty if creating.
   */
  applicationId: bigint
  onCompletion: OnCompletion

  /**
   * \[apaa\] transaction specific arguments accessed from the application's approval-program and clear-state-program.
   */
  applicationArgs?: string[]

  /**
   * \[al\] Access unifies `accounts`, `foreign-apps`, `foreign-assets`, and `box-references` under a single list. If access is non-empty, these lists must be empty. If access is empty, those lists may be non-empty.
   */
  access?: ResourceRef[]

  /**
   * \[apat\] List of accounts in addition to the sender that may be accessed from the application's approval-program and clear-state-program.
   */
  accounts?: string[]

  /**
   * \[apbx\] the boxes that can be accessed by this transaction (and others in the same group).
   */
  boxReferences?: BoxReference[]

  /**
   * \[apfa\] Lists the applications in addition to the application-id whose global states may be accessed by this application's approval-program and clear-state-program. The access is read-only.
   */
  foreignApps?: bigint[]

  /**
   * \[apas\] lists the assets whose parameters may be accessed by this application's ApprovalProgram and ClearStateProgram. The access is read-only.
   */
  foreignAssets?: bigint[]
  localStateSchema?: StateSchema
  globalStateSchema?: StateSchema

  /**
   * \[apap\] Logic executed for every application transaction, except when on-completion is set to "clear". It can read and write global state for the application, as well as account-specific local state. Approval programs may reject the transaction.
   */
  approvalProgram?: string

  /**
   * \[apsu\] Logic executed for application transactions with on-completion set to "clear". It can read and write global state for the application, as well as account-specific local state. Clear state programs cannot reject the transaction.
   */
  clearStateProgram?: string

  /**
   * \[epp\] specifies the additional app program len requested in pages.
   */
  extraProgramPages?: bigint

  /**
   * \[aprv\] the lowest application version for which this transaction should immediately fail. 0 indicates that no version check should be performed.
   */
  rejectVersion?: bigint
}
