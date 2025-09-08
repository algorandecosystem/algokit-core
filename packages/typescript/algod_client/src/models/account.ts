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
  totalAppsOptedIn: bigint;
  appsTotalSchema?: ApplicationStateSchema;
  appsTotalExtraPages?: bigint;
  assets?: AssetHolding[];
  totalAssetsOptedIn: bigint;
  createdApps?: Application[];
  totalCreatedApps: bigint;
  createdAssets?: Asset[];
  totalCreatedAssets: bigint;
  totalBoxes?: bigint;
  totalBoxBytes?: bigint;
  participation?: AccountParticipation;
  incentiveEligible?: boolean;
  pendingRewards: bigint;
  rewardBase?: bigint;
  rewards: bigint;
  round: bigint;
  status: string;
  sigType?: "sig" | "msig" | "lsig";
  authAddr?: string;
  lastProposed?: bigint;
  lastHeartbeat?: bigint;
};
