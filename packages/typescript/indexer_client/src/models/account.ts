import type { AccountParticipation, Application, ApplicationLocalState, ApplicationStateSchema, Asset, AssetHolding } from "./index";

/**
 * Account information at a given round.
 *
 * Definition:
 * data/basics/userBalance.go : AccountData
 */
export type Account = {
  address: string;
  amount: bigint;
  minBalance: bigint;
  amountWithoutPendingRewards: bigint;
  appsLocalState?: ApplicationLocalState[];
  appsTotalSchema?: ApplicationStateSchema;
  appsTotalExtraPages?: bigint;
  assets?: AssetHolding[];
  createdApps?: Application[];
  createdAssets?: Asset[];
  participation?: AccountParticipation;
  incentiveEligible?: boolean;
  pendingRewards: bigint;
  rewardBase?: bigint;
  rewards: bigint;
  round: bigint;
  status: string;
  sigType?: "sig" | "msig" | "lsig";
  totalAppsOptedIn: bigint;
  totalAssetsOptedIn: bigint;
  totalBoxBytes: bigint;
  totalBoxes: bigint;
  totalCreatedApps: bigint;
  totalCreatedAssets: bigint;
  authAddr?: string;
  lastProposed?: bigint;
  lastHeartbeat?: bigint;
  deleted?: boolean;
  createdAtRound?: bigint;
  closedAtRound?: bigint;
};
