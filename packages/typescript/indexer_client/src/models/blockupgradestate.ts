/**
 * Fields relating to a protocol upgrade.
 */
export type BlockUpgradeState = {
  currentProtocol: string;
  nextProtocol?: string;
  nextProtocolApprovals?: bigint;
  nextProtocolSwitchOn?: bigint;
  nextProtocolVoteBefore?: bigint;
};
