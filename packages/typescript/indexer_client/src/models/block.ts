import type { BlockRewards, BlockUpgradeState, BlockUpgradeVote, ParticipationUpdates, StateProofTracking, Transaction } from "./index";

/**
 * Block information.
 *
 * Definition:
 * data/bookkeeping/block.go : Block
 */
export type Block = {
  proposer?: string;
  "fees-collected"?: number;
  bonus?: number;
  "proposer-payout"?: number;
  "genesis-hash": string;
  "genesis-id": string;
  "previous-block-hash": string;
  rewards?: BlockRewards;
  round: bigint;
  seed: string;
  "state-proof-tracking"?: StateProofTracking[];
  timestamp: number;
  transactions?: Transaction[];
  "transactions-root": string;
  "transactions-root-sha256": string;
  "txn-counter"?: number;
  "upgrade-state"?: BlockUpgradeState;
  "upgrade-vote"?: BlockUpgradeVote;
  "participation-updates"?: ParticipationUpdates;
};
