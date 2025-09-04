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
  "min-balance": bigint;
  "amount-without-pending-rewards": bigint;
  "apps-local-state"?: ApplicationLocalState[];
  "apps-total-schema"?: ApplicationStateSchema;
  "apps-total-extra-pages"?: number;
  assets?: AssetHolding[];
  "created-apps"?: Application[];
  "created-assets"?: Asset[];
  participation?: AccountParticipation;
  "incentive-eligible"?: boolean;
  "pending-rewards": bigint;
  "reward-base"?: bigint;
  rewards: bigint;
  round: bigint;
  status: string;
  "sig-type"?: "sig" | "msig" | "lsig";
  "total-apps-opted-in": number;
  "total-assets-opted-in": number;
  "total-box-bytes": number;
  "total-boxes": number;
  "total-created-apps": number;
  "total-created-assets": number;
  "auth-addr"?: string;
  "last-proposed"?: bigint;
  "last-heartbeat"?: bigint;
  deleted?: boolean;
  "created-at-round"?: bigint;
  "closed-at-round"?: bigint;
};
