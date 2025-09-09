/**
 * Fields relating to voting for a protocol upgrade.
 */
export type BlockUpgradeVote = {
  /**
   * \[upgradeyes\] Indicates a yes vote for the current proposal.
   */
  upgradeApprove?: boolean;

  /**
   * \[upgradedelay\] Indicates the time between acceptance and execution.
   */
  upgradeDelay?: bigint;

  /**
   * \[upgradeprop\] Indicates a proposed upgrade.
   */
  upgradePropose?: string;
};
