/**
 * NodeStatus contains the information about a node status
 */
export type WaitForBlock = {
  "catchup-time": bigint;
  "last-round": bigint;
  "last-version": string;
  "next-version": string;
  "next-version-round": number;
  "next-version-supported": boolean;
  "stopped-at-unsupported-round": boolean;
  "time-since-last-round": bigint;
  "last-catchpoint"?: string;
  catchpoint?: string;
  "catchpoint-total-accounts"?: bigint;
  "catchpoint-processed-accounts"?: bigint;
  "catchpoint-verified-accounts"?: bigint;
  "catchpoint-total-kvs"?: bigint;
  "catchpoint-processed-kvs"?: bigint;
  "catchpoint-verified-kvs"?: bigint;
  "catchpoint-total-blocks"?: bigint;
  "catchpoint-acquired-blocks"?: bigint;
  "upgrade-delay"?: number;
  "upgrade-node-vote"?: boolean;
  "upgrade-votes-required"?: number;
  "upgrade-votes"?: number;
  "upgrade-yes-votes"?: number;
  "upgrade-no-votes"?: number;
  "upgrade-next-protocol-vote-before"?: number;
  "upgrade-vote-rounds"?: number;
};
