import type { AccountParticipation } from "./index";

/**
 * Represents a participation key used by the node.
 */
export type ParticipationKey = {
  id: string;
  address: string;
  "effective-first-valid"?: bigint;
  "effective-last-valid"?: bigint;
  "last-vote"?: number;
  "last-block-proposal"?: number;
  "last-state-proof"?: number;
  key: AccountParticipation;
};
