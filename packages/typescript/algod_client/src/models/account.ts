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
  "total-apps-opted-in": bigint;
  "apps-total-schema"?: ApplicationStateSchema;
  "apps-total-extra-pages"?: bigint;
  assets?: AssetHolding[];
  "total-assets-opted-in": bigint;
  "created-apps"?: Application[];
  "total-created-apps": bigint;
  "created-assets"?: Asset[];
  "total-created-assets": bigint;
  "total-boxes"?: bigint;
  "total-box-bytes"?: bigint;
  participation?: AccountParticipation;
  "incentive-eligible"?: boolean;
  "pending-rewards": bigint;
  "reward-base"?: bigint;
  rewards: bigint;
  round: bigint;
  status: string;
  "sig-type"?: "sig" | "msig" | "lsig";
  "auth-addr"?: string;
  "last-proposed"?: bigint;
  "last-heartbeat"?: bigint;
};
