/**
 * NodeStatus contains the information about a node status
 */
export type GetStatus = {
  catchupTime: bigint;
  lastRound: bigint;
  lastVersion: string;
  nextVersion: string;
  nextVersionRound: bigint;
  nextVersionSupported: boolean;
  stoppedAtUnsupportedRound: boolean;
  timeSinceLastRound: bigint;
  lastCatchpoint?: string;
  catchpoint?: string;
  catchpointTotalAccounts?: bigint;
  catchpointProcessedAccounts?: bigint;
  catchpointVerifiedAccounts?: bigint;
  catchpointTotalKvs?: bigint;
  catchpointProcessedKvs?: bigint;
  catchpointVerifiedKvs?: bigint;
  catchpointTotalBlocks?: bigint;
  catchpointAcquiredBlocks?: bigint;
  upgradeDelay?: bigint;
  upgradeNodeVote?: boolean;
  upgradeVotesRequired?: bigint;
  upgradeVotes?: bigint;
  upgradeYesVotes?: bigint;
  upgradeNoVotes?: bigint;
  upgradeNextProtocolVoteBefore?: bigint;
  upgradeVoteRounds?: bigint;
};
