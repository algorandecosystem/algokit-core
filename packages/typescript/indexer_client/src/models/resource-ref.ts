import type { BoxReference, HoldingRef, LocalsRef } from './index'
/**
 * ResourceRef names a single resource. Only one of the fields should be set.
 */
export type ResourceRef = {
  /**
   * \[d\] Account whose balance record is accessible by the executing ApprovalProgram or ClearStateProgram.
   */
  address?: string

  /**
   * \[p\] Application id whose GlobalState may be read by the executing
   * ApprovalProgram or ClearStateProgram.
   */
  applicationId?: bigint

  /**
   * \[s\] Asset whose AssetParams may be read by the executing
   * ApprovalProgram or ClearStateProgram.
   */
  assetId?: bigint
  box?: BoxReference
  holding?: HoldingRef
  local?: LocalsRef
}
