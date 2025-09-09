/**
 * Fields relating to a protocol upgrade.
 */
export type BlockUpgradeState = {
  /**
   * \[proto\] The current protocol version.
   */
  currentProtocol: string;

  /**
   * \[nextproto\] The next proposed protocol version.
   */
  nextProtocol?: string;

  /**
   * \[nextyes\] Number of blocks which approved the protocol upgrade.
   */
  nextProtocolApprovals?: bigint;

  /**
   * \[nextswitch\] Round on which the protocol upgrade will take effect.
   */
  nextProtocolSwitchOn?: bigint;

  /**
   * \[nextbefore\] Deadline round for this protocol upgrade (No votes will be consider after this round).
   */
  nextProtocolVoteBefore?: bigint;
};
