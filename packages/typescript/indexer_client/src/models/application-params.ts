import type { ApplicationStateSchema, TealKeyValueStore } from './index'
/**
 * Stores the global information associated with an application.
 */
export type ApplicationParams = {
  /**
   * The address that created this application. This is the address where the parameters and global state for this application can be found.
   */
  creator?: string

  /**
   * approval program.
   */
  approvalProgram?: string

  /**
   * clear state program.
   */
  clearStateProgram?: string

  /**
   * the number of extra program pages available to this app.
   */
  extraProgramPages?: bigint
  localStateSchema?: ApplicationStateSchema
  globalStateSchema?: ApplicationStateSchema
  globalState?: TealKeyValueStore

  /**
   * the number of updates to the application programs
   */
  version?: bigint
}
