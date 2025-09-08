import type { AccountParticipation } from "./index";

/**
 * Represents a participation key used by the node.
 */
export type ParticipationKey = {
  id: string;
  address: string;
  effectiveFirstValid?: bigint;
  effectiveLastValid?: bigint;
  lastVote?: bigint;
  lastBlockProposal?: bigint;
  lastStateProof?: bigint;
  key: AccountParticipation;
};
