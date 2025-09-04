/**
 * Fields relating to a protocol upgrade.
 */
export type BlockUpgradeState = {
  "current-protocol": string;
  "next-protocol"?: string;
  "next-protocol-approvals"?: number;
  "next-protocol-switch-on"?: number;
  "next-protocol-vote-before"?: number;
};
