import type { BlockRewards, BlockUpgradeState, BlockUpgradeVote, ParticipationUpdates, StateProofTracking, Transaction } from "./index";

/**
 * Block information.
 *
 * Definition:
 * data/bookkeeping/block.go : Block
 */
export type Block = {
  proposer?: string;
  feesCollected?: bigint;
  bonus?: bigint;
  proposerPayout?: bigint;
  genesisHash: string;
  genesisId: string;
  previousBlockHash: string;
  rewards?: BlockRewards;
  round: bigint;
  seed: string;
  stateProofTracking?: StateProofTracking[];
  timestamp: bigint;
  transactions?: Transaction[];
  transactionsRoot: string;
  transactionsRootSha256: string;
  txnCounter?: bigint;
  upgradeState?: BlockUpgradeState;
  upgradeVote?: BlockUpgradeVote;
  participationUpdates?: ParticipationUpdates;
};
